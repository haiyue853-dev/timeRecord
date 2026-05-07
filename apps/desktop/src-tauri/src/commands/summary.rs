use crate::summary::{LocalSummaryProvider, SummaryContext, SummaryService};

#[tauri::command]
pub async fn generate_boot_summary() -> Result<String, String> {
    SummaryService::local_only(LocalSummaryProvider::new(2))
        .generate(SummaryContext {
            total_active_seconds: 0,
            top_app_name: "code.exe".into(),
            top_app_seconds: 0,
            learning_seconds: 0,
            development_seconds: 0,
        })
        .await
        .map_err(|error| error.to_string())
}
