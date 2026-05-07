use timerecord_lib::summary::{LocalSummaryProvider, SummaryContext, SummaryService};

#[test]
fn falls_back_to_local_summary_when_ai_fails() {
    let context = SummaryContext {
        total_active_seconds: 7_200,
        top_app_name: "code.exe".into(),
        top_app_seconds: 3_600,
        learning_seconds: 1_800,
        development_seconds: 3_600,
    };

    let local = LocalSummaryProvider::new(7);
    let service = SummaryService::with_failing_ai(local);
    let summary = tauri::async_runtime::block_on(service.generate(context)).unwrap();

    assert!(summary.contains("VS Code") || summary.contains("code.exe"));
}
