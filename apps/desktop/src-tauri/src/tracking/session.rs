pub use crate::tracking::foreground::ForegroundSnapshot;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityRecord {
    pub boot_id: String,
    pub process_name: String,
    pub app_name: String,
    pub window_title: String,
    pub window_handle: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub duration_seconds: i64,
    pub idle_rule_triggered: bool,
    pub media_playback_kept_alive: bool,
}

#[derive(Debug)]
pub struct SessionRecorder {
    boot_id: String,
    current: Option<ForegroundSnapshot>,
    records: Vec<ActivityRecord>,
}

impl SessionRecorder {
    pub fn new_for_test(boot_id: impl Into<String>) -> Self {
        Self {
            boot_id: boot_id.into(),
            current: None,
            records: Vec::new(),
        }
    }

    pub fn observe(&mut self, snapshot: ForegroundSnapshot) -> anyhow::Result<()> {
        if self.current.is_none() {
            self.current = Some(snapshot);
            return Ok(());
        }

        self.close_current(snapshot.captured_at);
        self.current = Some(snapshot);
        Ok(())
    }

    pub fn close_current(&mut self, ended_at: DateTime<Utc>) -> Option<ActivityRecord> {
        let current = self.current.take()?;
        let duration_seconds = (ended_at - current.captured_at).num_seconds().max(0);
        let record = ActivityRecord {
            boot_id: self.boot_id.clone(),
            process_name: current.process_name,
            app_name: current.app_name,
            window_title: current.window_title,
            window_handle: current.window_handle,
            started_at: current.captured_at,
            ended_at,
            duration_seconds,
            idle_rule_triggered: false,
            media_playback_kept_alive: false,
        };
        self.records.push(record.clone());
        Some(record)
    }

    pub fn records(&self) -> Vec<ActivityRecord> {
        let mut records = self.records.clone();
        if let Some(current) = &self.current {
            records.push(ActivityRecord {
                boot_id: self.boot_id.clone(),
                process_name: current.process_name.clone(),
                app_name: current.app_name.clone(),
                window_title: current.window_title.clone(),
                window_handle: current.window_handle,
                started_at: current.captured_at,
                ended_at: current.captured_at,
                duration_seconds: 0,
                idle_rule_triggered: false,
                media_playback_kept_alive: false,
            });
        }
        records
    }
}
