use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppUsageStat {
    pub app_name: String,
    pub duration_seconds: i64,
}
