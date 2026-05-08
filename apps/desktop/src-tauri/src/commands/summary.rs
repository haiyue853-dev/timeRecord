use chrono::Utc;
use serde::Serialize;
use tauri::State;

use crate::{
    app_state::AppState,
    summary::{deepseek::DeepSeekConfig, LocalSummaryProvider, SummaryService},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryPayload {
    pub summary: String,
    pub encouragement: String,
    pub source: String,
}

#[tauri::command]
pub async fn generate_boot_summary(state: State<'_, AppState>) -> Result<SummaryPayload, String> {
    let settings = state.settings();
    let context = state.summary_context();
    let local = LocalSummaryProvider::new(Utc::now().timestamp().unsigned_abs());
    let service = if settings.ai_enabled && !settings.deepseek_api_key.trim().is_empty() {
        SummaryService::with_ai(
            local,
            DeepSeekConfig {
                base_url: settings.deepseek_base_url,
                api_key: settings.deepseek_api_key,
                model: settings.deepseek_model,
            },
        )
    } else {
        SummaryService::local_only(local)
    };
    let bundle = service.generate(context).await.map_err(|error| error.to_string())?;

    Ok(SummaryPayload {
        summary: bundle.summary,
        encouragement: bundle.encouragement,
        source: bundle.source,
    })
}
