use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use super::{SummaryBundle, SummaryContext};

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    temperature: f32,
    response_format: DeepSeekResponseFormat,
}

#[derive(Debug, Serialize)]
struct DeepSeekMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Serialize)]
struct DeepSeekResponseFormat {
    #[serde(rename = "type")]
    response_type: &'static str,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekMessageResponse,
}

#[derive(Debug, Deserialize)]
struct DeepSeekMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekSummaryPayload {
    summary: String,
    encouragement: String,
}

pub async fn generate_summary(config: &DeepSeekConfig, context: &SummaryContext) -> Result<SummaryBundle> {
    let request = DeepSeekRequest {
        model: config.model.clone(),
        temperature: 1.0,
        response_format: DeepSeekResponseFormat {
            response_type: "json_object",
        },
        messages: vec![
            DeepSeekMessage {
                role: "system",
                content: "你是一个温柔但直白的学习/效率总结助手。你要根据用户昨天的桌面活跃数据写一段中文小结，再给一句单独的鼓励话。请只返回 JSON，格式为 {\"summary\":\"...\",\"encouragement\":\"...\"}。summary 不超过 90 字，encouragement 不超过 35 字。".into(),
            },
            DeepSeekMessage {
                role: "user",
                content: format!(
                    "昨天总活跃时长：{} 分钟\n昨天学习时长：{} 分钟\n昨天开发时长：{} 分钟\n前一天总活跃时长：{} 分钟\n本周累计时长：{} 分钟\n本周日均时长：{} 分钟\n请输出简短总结和一句鼓励话。",
                    context.yesterday_total_seconds / 60,
                    context.yesterday_learning_seconds / 60,
                    context.yesterday_development_seconds / 60,
                    context.previous_day_total_seconds / 60,
                    context.current_week_total_seconds / 60,
                    context.current_week_average_seconds / 60,
                ),
            },
        ],
    };

    let body = serde_json::to_string(&request).context("failed to serialize DeepSeek request")?;
    let endpoint = format!(
        "{}/chat/completions",
        config.base_url.trim_end_matches('/')
    );
    let output = tauri::async_runtime::spawn_blocking({
        let endpoint = endpoint.clone();
        let api_key = config.api_key.clone();
        move || invoke_powershell_request(&endpoint, &api_key, &body)
    })
    .await
    .context("failed to join DeepSeek request task")??;

    let payload = serde_json::from_str::<DeepSeekResponse>(&output)
        .context("failed to decode DeepSeek response")?;
    let content = payload
        .choices
        .first()
        .map(|choice| choice.message.content.trim())
        .filter(|content| !content.is_empty())
        .ok_or_else(|| anyhow!("DeepSeek response did not contain content"))?;
    let bundle = serde_json::from_str::<DeepSeekSummaryPayload>(content)
        .context("failed to parse DeepSeek JSON summary payload")?;

    Ok(SummaryBundle {
        summary: bundle.summary.trim().to_string(),
        encouragement: bundle.encouragement.trim().to_string(),
        source: "deepseek".into(),
    })
}

fn invoke_powershell_request(endpoint: &str, api_key: &str, body: &str) -> Result<String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to build temp filename")?
        .as_millis();
    let body_path = std::env::temp_dir().join(format!("timerecord-deepseek-{nonce}.json"));
    fs::write(&body_path, body).context("failed to write DeepSeek request body")?;

    let script = r#"
$headers = @{
  Authorization = "Bearer $env:DEEPSEEK_API_KEY"
  "Content-Type" = "application/json"
}
$body = Get-Content $env:DEEPSEEK_BODY_PATH -Raw
$response = Invoke-RestMethod -Uri $env:DEEPSEEK_ENDPOINT -Method Post -Headers $headers -Body $body
$response | ConvertTo-Json -Depth 20 -Compress
"#;

    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(script)
        .env("DEEPSEEK_ENDPOINT", endpoint)
        .env("DEEPSEEK_API_KEY", api_key.trim())
        .env("DEEPSEEK_BODY_PATH", &body_path)
        .output()
        .context("failed to launch PowerShell for DeepSeek request")?;

    let _ = fs::remove_file(&body_path);

    if !output.status.success() {
        return Err(anyhow!(
            "DeepSeek request failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8(output.stdout).context("DeepSeek response was not valid UTF-8")?)
}
