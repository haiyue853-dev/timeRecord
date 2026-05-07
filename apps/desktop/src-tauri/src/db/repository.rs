use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};

use crate::config::AppSettings;

pub struct SettingsRepository {
    conn: Connection,
}

impl SettingsRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn save(&self, settings: &AppSettings) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO settings (
                id,
                idle_seconds,
                ai_enabled,
                deepseek_base_url,
                deepseek_model,
                updated_at
            ) VALUES (1, ?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
            ON CONFLICT(id) DO UPDATE SET
                idle_seconds = excluded.idle_seconds,
                ai_enabled = excluded.ai_enabled,
                deepseek_base_url = excluded.deepseek_base_url,
                deepseek_model = excluded.deepseek_model,
                updated_at = CURRENT_TIMESTAMP
            "#,
            params![
                settings.idle_seconds as i64,
                settings.ai_enabled,
                settings.deepseek_base_url,
                settings.deepseek_model,
            ],
        )?;

        Ok(())
    }

    pub fn load(&self) -> Result<AppSettings> {
        let settings = self
            .conn
            .query_row(
                r#"
                SELECT idle_seconds, ai_enabled, deepseek_base_url, deepseek_model
                FROM settings
                WHERE id = 1
                "#,
                [],
                |row| {
                    Ok(AppSettings {
                        idle_seconds: row.get::<_, i64>(0)? as u64,
                        ai_enabled: row.get(1)?,
                        deepseek_base_url: row.get(2)?,
                        deepseek_model: row.get(3)?,
                    })
                },
            )
            .optional()?;

        Ok(settings.unwrap_or_default())
    }
}
