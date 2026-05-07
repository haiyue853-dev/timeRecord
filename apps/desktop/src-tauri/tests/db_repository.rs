use rusqlite::Connection;
use timerecord_lib::config::AppSettings;
use timerecord_lib::db::repository::SettingsRepository;

fn open_repo() -> SettingsRepository {
    let conn = timerecord_lib::db::open_in_memory().unwrap();
    timerecord_lib::db::run_migrations(&conn).unwrap();
    SettingsRepository::new(conn)
}

fn table_columns(conn: &Connection, table: &str) -> Vec<String> {
    let pragma = format!("PRAGMA table_info({table})");
    let mut stmt = conn.prepare(&pragma).unwrap();

    stmt.query_map([], |row| row.get::<_, String>(1))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[test]
fn returns_default_settings_when_no_record_exists() {
    let repo = open_repo();

    let loaded = repo.load().unwrap();

    assert_eq!(loaded, AppSettings::default());
}

#[test]
fn saves_and_round_trips_non_default_settings() {
    let repo = open_repo();
    let settings = AppSettings {
        idle_seconds: 420,
        ai_enabled: true,
        deepseek_base_url: "https://example.internal/api".into(),
        deepseek_model: "deepseek-reasoner".into(),
    };

    repo.save(&settings).unwrap();
    let loaded = repo.load().unwrap();

    assert_eq!(loaded, settings);
}

#[test]
fn save_overwrites_existing_settings() {
    let repo = open_repo();

    repo.save(&AppSettings {
        idle_seconds: 60,
        ai_enabled: false,
        deepseek_base_url: "https://first.example".into(),
        deepseek_model: "first-model".into(),
    })
    .unwrap();

    repo.save(&AppSettings {
        idle_seconds: 900,
        ai_enabled: true,
        deepseek_base_url: "https://second.example".into(),
        deepseek_model: "second-model".into(),
    })
    .unwrap();

    let loaded = repo.load().unwrap();

    assert_eq!(
        loaded,
        AppSettings {
            idle_seconds: 900,
            ai_enabled: true,
            deepseek_base_url: "https://second.example".into(),
            deepseek_model: "second-model".into(),
        }
    );
}

#[test]
fn migrations_record_schema_version() {
    let conn = timerecord_lib::db::open_in_memory().unwrap();

    timerecord_lib::db::run_migrations(&conn).unwrap();

    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap();
    assert_eq!(version, 1);
}

#[test]
fn migrations_create_structured_activity_tables() {
    let conn = timerecord_lib::db::open_in_memory().unwrap();

    timerecord_lib::db::run_migrations(&conn).unwrap();

    let boot_columns = table_columns(&conn, "boot_sessions");
    assert_eq!(
        boot_columns,
        vec![
            "id",
            "boot_id",
            "started_at",
            "ended_at",
            "total_active_seconds",
            "created_at",
            "updated_at",
        ]
    );

    let activity_columns = table_columns(&conn, "activity_records");
    assert_eq!(
        activity_columns,
        vec![
            "id",
            "boot_session_id",
            "process_name",
            "app_name",
            "window_title",
            "window_handle",
            "started_at",
            "ended_at",
            "duration_seconds",
            "idle_rule_triggered",
            "media_playback_kept_alive",
            "created_at",
        ]
    );
}
