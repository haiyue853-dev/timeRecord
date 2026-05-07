pub const MIGRATIONS: &[&str] = &[
    r#"
    CREATE TABLE IF NOT EXISTS settings (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        idle_seconds INTEGER NOT NULL,
        ai_enabled INTEGER NOT NULL,
        deepseek_base_url TEXT NOT NULL,
        deepseek_model TEXT NOT NULL,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS boot_sessions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        started_at TEXT NOT NULL,
        ended_at TEXT
    );
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS activity_records (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        session_id INTEGER NOT NULL,
        started_at TEXT NOT NULL,
        ended_at TEXT,
        activity_type TEXT NOT NULL,
        payload_json TEXT NOT NULL DEFAULT '{}',
        FOREIGN KEY (session_id) REFERENCES boot_sessions(id)
    );
    "#,
];
