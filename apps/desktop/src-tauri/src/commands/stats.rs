use serde::Serialize;

use crate::{
    stats::queries::AppUsageStat,
    summary::{LocalSummaryProvider, SummaryContext, SummaryService},
};

#[derive(Debug, Clone, Serialize)]
pub struct DashboardStats {
    pub total_active_seconds: i64,
    pub current_app_name: String,
    pub current_window_title: String,
    pub apps: Vec<AppUsageStat>,
    pub summary: String,
}

#[tauri::command]
pub async fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let context = SummaryContext {
        total_active_seconds: 0,
        top_app_name: "code.exe".into(),
        top_app_seconds: 0,
        learning_seconds: 0,
        development_seconds: 0,
    };
    let summary = SummaryService::local_only(LocalSummaryProvider::new(1))
        .generate(context)
        .await
        .map_err(|error| error.to_string())?;

    Ok(DashboardStats {
        total_active_seconds: 0,
        current_app_name: "TimeRecord".into(),
        current_window_title: "等待活动采集".into(),
        apps: vec![AppUsageStat {
            app_name: "TimeRecord".into(),
            duration_seconds: 0,
        }],
        summary,
    })
}
