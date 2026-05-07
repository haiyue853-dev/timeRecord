#[derive(Debug, Clone)]
pub struct SummaryContext {
    pub total_active_seconds: i64,
    pub top_app_name: String,
    pub top_app_seconds: i64,
    pub learning_seconds: i64,
    pub development_seconds: i64,
}
