use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundSnapshot {
    pub window_handle: i64,
    pub process_name: String,
    pub app_name: String,
    pub window_title: String,
    pub captured_at: chrono::DateTime<chrono::Utc>,
}

impl ForegroundSnapshot {
    pub fn new(
        window_handle: i64,
        process_name: impl Into<String>,
        app_name: impl Into<String>,
        window_title: impl Into<String>,
        captured_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            window_handle,
            process_name: process_name.into(),
            app_name: app_name.into(),
            window_title: window_title.into(),
            captured_at,
        }
    }
}

pub trait ForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot>;
}

pub struct WindowsForegroundSource;

impl ForegroundSource for WindowsForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot> {
        Err(anyhow!("foreground window collection is not wired yet"))
    }
}
