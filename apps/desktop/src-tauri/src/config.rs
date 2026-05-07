use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppSettings {
    pub idle_seconds: u64,
    pub ai_enabled: bool,
    pub deepseek_base_url: String,
    pub deepseek_model: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            idle_seconds: 180,
            ai_enabled: false,
            deepseek_base_url: "https://api.deepseek.com".into(),
            deepseek_model: "deepseek-chat".into(),
        }
    }
}
