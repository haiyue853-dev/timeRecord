pub const CURRENT_SCHEMA_VERSION: i32 = 1;

pub const MIGRATIONS: &[(i32, &str)] = &[
    (
        1,
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            idle_seconds INTEGER NOT NULL,
            ai_enabled INTEGER NOT NULL,
            deepseek_base_url TEXT NOT NULL,
            deepseek_model TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS boot_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            boot_id TEXT NOT NULL,
            started_at TEXT NOT NULL,
            ended_at TEXT,
            total_active_seconds INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_boot_sessions_boot_id
            ON boot_sessions(boot_id);

        CREATE INDEX IF NOT EXISTS idx_boot_sessions_started_at
            ON boot_sessions(started_at);

        CREATE TABLE IF NOT EXISTS activity_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            boot_session_id INTEGER NOT NULL,
            process_name TEXT NOT NULL,
            app_name TEXT NOT NULL,
            window_title TEXT NOT NULL,
            window_handle TEXT,
            started_at TEXT NOT NULL,
            ended_at TEXT,
            duration_seconds INTEGER NOT NULL DEFAULT 0,
            idle_rule_triggered INTEGER NOT NULL DEFAULT 0,
            media_playback_kept_alive INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (boot_session_id) REFERENCES boot_sessions(id)
        );

        CREATE INDEX IF NOT EXISTS idx_activity_records_boot_session_id
            ON activity_records(boot_session_id);

        CREATE INDEX IF NOT EXISTS idx_activity_records_started_at
            ON activity_records(started_at);

        CREATE INDEX IF NOT EXISTS idx_activity_records_process_name
            ON activity_records(process_name);
        "#,
    ),
];
