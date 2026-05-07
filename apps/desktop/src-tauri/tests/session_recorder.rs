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
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].duration_seconds, 90);

    let current = recorder.current().unwrap();
    assert_eq!(current.process_name, "chrome.exe");
}

#[test]
fn repeated_snapshot_for_same_window_keeps_single_current_record() {
    let start = Utc::now();
    let later = start + Duration::seconds(45);

    let first = ForegroundSnapshot::new(1, "code.exe", "VS Code", "main.rs", start);
    let repeated = ForegroundSnapshot::new(1, "code.exe", "VS Code", "main.rs", later);

    let mut recorder = SessionRecorder::new_for_test("boot-1");
    recorder.observe(first).unwrap();
    recorder.observe(repeated).unwrap();

    let records = recorder.records();
    assert!(records.is_empty());

    let current = recorder.current().unwrap();
    assert_eq!(current.process_name, "code.exe");
    assert_eq!(current.captured_at, start);
}

#[test]
fn keeps_latest_snapshot_as_current_after_window_switch() {
    let start = Utc::now();
    let second = start + Duration::seconds(90);

    let first = ForegroundSnapshot::new(1, "code.exe", "VS Code", "main.rs", start);
    let next = ForegroundSnapshot::new(2, "chrome.exe", "Chrome", "Rust Docs", second);

    let mut recorder = SessionRecorder::new_for_test("boot-1");
    recorder.observe(first).unwrap();
    recorder.observe(next).unwrap();

    let current = recorder.current().unwrap();
    assert_eq!(current.window_handle, 2);
    assert_eq!(current.process_name, "chrome.exe");
    assert_eq!(current.window_title, "Rust Docs");
}
