use timerecord_lib::summary::{LocalSummaryProvider, SummaryBundle, SummaryContext, SummaryService};

#[test]
fn local_summary_mentions_yesterday_and_returns_encouragement() {
    let context = SummaryContext {
        yesterday_total_seconds: 7_200,
        yesterday_learning_seconds: 3_000,
        yesterday_development_seconds: 2_400,
        previous_day_total_seconds: 5_400,
        current_week_total_seconds: 18_000,
        current_week_average_seconds: 2_571,
    };

    let bundle = LocalSummaryProvider::new(7).generate(&context);

    assert!(bundle.summary.contains("昨天"));
    assert!(bundle.summary.contains("120"));
    assert!(!bundle.encouragement.is_empty());
    assert_eq!(bundle.source, "local");
}

#[test]
fn falls_back_to_local_summary_when_ai_fails() {
    let context = SummaryContext {
        yesterday_total_seconds: 7_200,
        yesterday_learning_seconds: 1_800,
        yesterday_development_seconds: 3_600,
        previous_day_total_seconds: 6_000,
        current_week_total_seconds: 21_600,
        current_week_average_seconds: 3_085,
    };

    let local = LocalSummaryProvider::new(7);
    let service = SummaryService::with_failing_ai(local);
    let summary = tauri::async_runtime::block_on(service.generate(context)).unwrap();

    assert_eq!(
        summary,
        SummaryBundle {
            summary: summary.summary.clone(),
            encouragement: summary.encouragement.clone(),
            source: "local".into(),
        }
    );
    assert!(summary.summary.contains("昨天"));
    assert!(!summary.encouragement.is_empty());
}
