use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DailyHistory {
    pub date: NaiveDate,
    pub total_active_seconds: i64,
    pub learning_seconds: i64,
    pub development_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionTrendBucket {
    pub bucket_started_at: DateTime<Utc>,
    pub active_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionTrendPoint {
    pub label: String,
    pub active_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WeeklySummary {
    pub current_week_total_seconds: i64,
    pub previous_week_total_seconds: i64,
    pub current_week_learning_seconds: i64,
    pub current_week_average_seconds: i64,
    pub delta_seconds: i64,
    pub best_day: DailyHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LearningHeatmapCell {
    pub date: NaiveDate,
    pub learning_seconds: i64,
    pub level: u8,
}

pub fn build_weekly_summary(anchor: NaiveDate, days: &[DailyHistory]) -> WeeklySummary {
    let current_start = anchor - Duration::days(6);
    let previous_start = current_start - Duration::days(7);
    let previous_end = current_start - Duration::days(1);

    let mut current_window = days
        .iter()
        .filter(|day| day.date >= current_start && day.date <= anchor)
        .cloned()
        .collect::<Vec<_>>();
    current_window.sort_by_key(|day| day.date);

    let previous_window = days
        .iter()
        .filter(|day| day.date >= previous_start && day.date <= previous_end)
        .cloned()
        .collect::<Vec<_>>();

    let current_week_total_seconds = current_window
        .iter()
        .map(|day| day.total_active_seconds)
        .sum();
    let previous_week_total_seconds = previous_window
        .iter()
        .map(|day| day.total_active_seconds)
        .sum();
    let current_week_learning_seconds = current_window
        .iter()
        .map(|day| day.learning_seconds)
        .sum();
    let current_week_average_seconds = current_week_total_seconds / 7;
    let delta_seconds = current_week_total_seconds - previous_week_total_seconds;

    let best_day = current_window
        .into_iter()
        .max_by_key(|day| day.total_active_seconds)
        .unwrap_or(DailyHistory {
            date: anchor,
            total_active_seconds: 0,
            learning_seconds: 0,
            development_seconds: 0,
        });

    WeeklySummary {
        current_week_total_seconds,
        previous_week_total_seconds,
        current_week_learning_seconds,
        current_week_average_seconds,
        delta_seconds,
        best_day,
    }
}

pub fn build_learning_heatmap(
    anchor: NaiveDate,
    days: usize,
    items: &[DailyHistory],
) -> Vec<LearningHeatmapCell> {
    let mut cells = Vec::with_capacity(days);

    for offset in (0..days).rev() {
        let date = anchor - Duration::days(offset as i64);
        let learning_seconds = items
            .iter()
            .find(|item| item.date == date)
            .map(|item| item.learning_seconds)
            .unwrap_or(0);

        cells.push(LearningHeatmapCell {
            date,
            learning_seconds,
            level: learning_level(learning_seconds),
        });
    }

    cells
}

pub fn build_session_trend_points(buckets: &[SessionTrendBucket]) -> Vec<SessionTrendPoint> {
    let mut sorted = buckets.to_vec();
    sorted.sort_by_key(|bucket| bucket.bucket_started_at);

    sorted
        .into_iter()
        .map(|bucket| SessionTrendPoint {
            label: format!(
                "{:02}:{:02}",
                bucket.bucket_started_at.hour(),
                bucket.bucket_started_at.minute()
            ),
            active_seconds: bucket.active_seconds,
        })
        .collect()
}

fn learning_level(learning_seconds: i64) -> u8 {
    let minutes = learning_seconds / 60;
    match minutes {
        0 => 0,
        1..=20 => 1,
        21..=50 => 2,
        51..=90 => 3,
        _ => 4,
    }
}
