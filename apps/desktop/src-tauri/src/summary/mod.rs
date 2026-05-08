pub mod context;
pub mod deepseek;
pub mod local;

use anyhow::Result;

pub use context::SummaryContext;
use deepseek::{generate_summary, DeepSeekConfig};
pub use local::LocalSummaryProvider;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryBundle {
    pub summary: String,
    pub encouragement: String,
    pub source: String,
}

pub struct SummaryService {
    local: LocalSummaryProvider,
    ai: Option<DeepSeekConfig>,
}

impl SummaryService {
    pub fn local_only(local: LocalSummaryProvider) -> Self {
        Self { local, ai: None }
    }

    pub fn with_ai(local: LocalSummaryProvider, config: DeepSeekConfig) -> Self {
        Self {
            local,
            ai: Some(config),
        }
    }

    pub fn with_failing_ai(local: LocalSummaryProvider) -> Self {
        Self::with_ai(
            local,
            DeepSeekConfig {
                base_url: "https://api.deepseek.com".into(),
                api_key: "test".into(),
                model: "deepseek-chat".into(),
            },
        )
    }

    pub async fn generate(&self, context: SummaryContext) -> Result<SummaryBundle> {
        if let Some(config) = &self.ai {
            if let Ok(summary) = generate_summary(config, &context).await {
                return Ok(summary);
            }
        }

        Ok(self.local.generate(&context))
    }
}
