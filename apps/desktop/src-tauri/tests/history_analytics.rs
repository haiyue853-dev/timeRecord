use chrono::{Duration, NaiveDate, Utc};
use timerecord_lib::history::{
    build_learning_heatmap, build_session_trend_points, build_weekly_summary, DailyHistory,
    SessionTrendBucket,
};

#[test]
fn weekly_summary_compares_recent_seven_days_against_previous_window() {
    let anchor = NaiveDate::from_ymd_opt(2026, 5, 8).unwrap();
    let mut days = Vec::new();

    for offset in 0..10 {
        let date = anchor - Duration::days(offset);
        days.push(DailyHistory {
            date,
            total_active_seconds: (offset + 1) * 600,
            learning_seconds: if offset < 3 { 1_800 } else { 600 },
            development_seconds: 900,
        });
    }

    let summary = build_weekly_summary(anchor, &days);

    assert_eq!(summary.current_week_total_seconds, 16_800);
    assert_eq!(summary.previous_week_total_seconds, 16_200);
    assert_eq!(summary.current_week_learning_seconds, 7_800);
    assert_eq!(summary.best_day.date, anchor - Duration::days(6));
    assert_eq!(summary.best_day.total_active_seconds, 4_200);
}

#[test]
fn learning_heatmap_returns_recent_days_with_levels() {
    let anchor = NaiveDate::from_ymd_opt(2026, 5, 8).unwrap();
    let days = vec![
        DailyHistory {
            date: anchor - Duration::days(2),
            total_active_seconds: 7_200,
            learning_seconds: 6_600,
            development_seconds: 600,
        },
        DailyHistory {
            date: anchor - Duration::days(1),
            total_active_seconds: 3_600,
            learning_seconds: 2_400,
            development_seconds: 1_200,
        },
    ];

    let cells = build_learning_heatmap(anchor, 7, &days);

    assert_eq!(cells.len(), 7);
    assert_eq!(cells.last().unwrap().date, anchor);
    assert_eq!(cells[4].learning_seconds, 6_600);
    assert_eq!(cells[4].level, 4);
    assert_eq!(cells[5].level, 2);
    assert_eq!(cells[6].level, 0);
}

#[test]
fn session_trend_points_are_sorted_and_keep_bucket_values() {
    let start = Utc::now();
    let buckets = vec![
        SessionTrendBucket {
            bucket_started_at: start + Duration::minutes(10),
            active_seconds: 180,
        },
        SessionTrendBucket {
            bucket_started_at: start,
            active_seconds: 60,
        },
        SessionTrendBucket {
            bucket_started_at: start + Duration::minutes(5),
            active_seconds: 300,
        },
    ];

    let points = build_session_trend_points(&buckets);

    assert_eq!(points.len(), 3);
    assert_eq!(points[0].active_seconds, 60);
    assert_eq!(points[1].active_seconds, 300);
    assert_eq!(points[2].active_seconds, 180);
    assert!(points[0].label <= points[1].label);
}
