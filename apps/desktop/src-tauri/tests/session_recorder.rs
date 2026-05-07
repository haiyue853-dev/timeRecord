use chrono::{Duration, Utc};
use timerecord_lib::tracking::session::{ForegroundSnapshot, SessionRecorder};

#[test]
fn switches_window_and_closes_previous_record() {
    let start = Utc::now();
    let second = start + Duration::seconds(90);

    let first = ForegroundSnapshot::new(1, "code.exe", "VS Code", "main.rs", start);
    let next = ForegroundSnapshot::new(2, "chrome.exe", "Chrome", "Rust Docs", second);

    let mut recorder = SessionRecorder::new_for_test("boot-1");
    recorder.observe(first).unwrap();
    recorder.observe(next).unwrap();

    let records = recorder.records();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].duration_seconds, 90);
    assert_eq!(records[1].process_name, "chrome.exe");
}
