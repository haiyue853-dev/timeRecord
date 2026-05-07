pub mod context;
pub mod deepseek;
pub mod local;

use anyhow::Result;

pub use context::SummaryContext;
use deepseek::{generate_summary, DeepSeekConfig};
pub use local::LocalSummaryProvider;

pub struct SummaryService {
    local: LocalSummaryProvider,
    ai: Option<DeepSeekConfig>,
}

impl SummaryService {
    pub fn with_failing_ai(local: LocalSummaryProvider) -> Self {
        Self {
            local,
            ai: Some(DeepSeekConfig {
                base_url: "https://api.deepseek.com".into(),
                api_key: "test".into(),
                model: "deepseek-chat".into(),
            }),
        }
    }

    pub async fn generate(&self, context: SummaryContext) -> Result<String> {
        if let Some(config) = &self.ai {
            if let Ok(summary) = generate_summary(config, &context).await {
                return Ok(summary);
            }
        }

        Ok(self.local.generate(&context))
    }
}
