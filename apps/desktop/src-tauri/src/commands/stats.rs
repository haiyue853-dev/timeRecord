use serde::Serialize;
use tauri::State;

use crate::{
    app_state::AppState,
    history::{LearningHeatmapCell, SessionTrendPoint, WeeklySummary},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUsageItem {
    pub app_name: String,
    pub seconds: i64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendPointDto {
    pub label: String,
    pub active_seconds: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyHistoryDto {
    pub date: String,
    pub total_active_seconds: i64,
    pub learning_seconds: i64,
    pub development_seconds: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklySummaryDto {
    pub current_week_total_seconds: i64,
    pub previous_week_total_seconds: i64,
    pub current_week_learning_seconds: i64,
    pub current_week_average_seconds: i64,
    pub delta_seconds: i64,
    pub best_day: DailyHistoryDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LearningHeatmapCellDto {
    pub date: String,
    pub learning_seconds: i64,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub total_active_seconds: i64,
    pub current_app_name: String,
    pub current_window_title: String,
    pub apps: Vec<AppUsageItem>,
    pub summary: String,
    pub encouragement: String,
    pub summary_source: String,
    pub trend_points: Vec<TrendPointDto>,
    pub weekly_summary: WeeklySummaryDto,
    pub learning_heatmap: Vec<LearningHeatmapCellDto>,
}

#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, AppState>) -> Result<DashboardStats, String> {
    let snapshot = state.dashboard_snapshot();

    Ok(DashboardStats {
        total_active_seconds: snapshot.total_active_seconds,
        current_app_name: snapshot.current_app_name,
        current_window_title: snapshot.current_window_title,
        apps: snapshot
            .apps
            .into_iter()
            .map(|item| AppUsageItem {
                app_name: item.app_name,
                seconds: item.seconds,
                category: item.category,
            })
            .collect(),
        summary: snapshot.summary,
        encouragement: snapshot.encouragement,
        summary_source: snapshot.summary_source,
        trend_points: snapshot
            .trend_points
            .into_iter()
            .map(map_trend_point)
            .collect(),
        weekly_summary: map_weekly_summary(snapshot.weekly_summary),
        learning_heatmap: snapshot
            .learning_heatmap
            .into_iter()
            .map(map_heatmap_cell)
            .collect(),
    })
}

fn map_trend_point(point: SessionTrendPoint) -> TrendPointDto {
    TrendPointDto {
        label: point.label,
        active_seconds: point.active_seconds,
    }
}

fn map_weekly_summary(summary: WeeklySummary) -> WeeklySummaryDto {
    WeeklySummaryDto {
        current_week_total_seconds: summary.current_week_total_seconds,
        previous_week_total_seconds: summary.previous_week_total_seconds,
        current_week_learning_seconds: summary.current_week_learning_seconds,
        current_week_average_seconds: summary.current_week_average_seconds,
        delta_seconds: summary.delta_seconds,
        best_day: DailyHistoryDto {
            date: summary.best_day.date.format("%Y-%m-%d").to_string(),
            total_active_seconds: summary.best_day.total_active_seconds,
            learning_seconds: summary.best_day.learning_seconds,
            development_seconds: summary.best_day.development_seconds,
        },
    }
}

fn map_heatmap_cell(cell: LearningHeatmapCell) -> LearningHeatmapCellDto {
    LearningHeatmapCellDto {
        date: cell.date.format("%Y-%m-%d").to_string(),
        learning_seconds: cell.learning_seconds,
        level: cell.level,
    }
}
