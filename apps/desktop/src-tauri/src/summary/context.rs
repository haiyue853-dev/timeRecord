#[derive(Debug, Clone)]
pub struct SummaryContext {
    pub yesterday_total_seconds: i64,
    pub yesterday_learning_seconds: i64,
    pub yesterday_development_seconds: i64,
    pub previous_day_total_seconds: i64,
    pub current_week_total_seconds: i64,
    pub current_week_average_seconds: i64,
}
