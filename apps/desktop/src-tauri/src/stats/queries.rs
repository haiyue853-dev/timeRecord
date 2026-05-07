#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppUsageStat {
    pub app_name: String,
    pub duration_seconds: i64,
}
