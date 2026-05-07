use super::context::SummaryContext;

pub struct LocalSummaryProvider {
    seed: u64,
}

impl LocalSummaryProvider {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn generate(&self, context: &SummaryContext) -> String {
        let top_app_name = humanize_app_name(&context.top_app_name);
        let tone = if context.total_active_seconds == 0 {
            "这次开机还没积累到足够多的使用数据"
        } else if context.development_seconds >= context.learning_seconds {
            "这次开机的重心更偏向开发投入"
        } else {
            "这次开机的重心更偏向学习积累"
        };

        let closing = if self.seed % 2 == 0 {
            "继续保持这个节奏。"
        } else {
            "已经不错了，稳住就好。"
        };

        format!(
            "{tone}，当前投入最多的是 {top_app_name}，累计 {} 分钟。{closing}",
            context.top_app_seconds / 60
        )
    }
}

fn humanize_app_name(app_name: &str) -> &str {
    match app_name {
        "code.exe" => "VS Code",
        "chrome.exe" => "Google Chrome",
        other => other,
    }
}
