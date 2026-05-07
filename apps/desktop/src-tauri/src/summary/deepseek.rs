use anyhow::{anyhow, Result};

use super::context::SummaryContext;

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

pub async fn generate_summary(
    _config: &DeepSeekConfig,
    _context: &SummaryContext,
) -> Result<String> {
    Err(anyhow!("DeepSeek call not configured"))
}
