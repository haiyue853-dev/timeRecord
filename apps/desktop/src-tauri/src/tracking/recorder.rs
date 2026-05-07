use chrono::{DateTime, Utc};

use super::{
    idle::IdleDecision,
    media::{looks_like_media_app, MediaPlaybackState},
    session::ForegroundSnapshot,
};

pub struct RecordingTick {
    pub snapshot: ForegroundSnapshot,
    pub last_input_at: DateTime<Utc>,
    pub media_state: MediaPlaybackState,
    pub idle_seconds: u64,
}

impl RecordingTick {
    pub fn decide(&self) -> IdleDecision {
        IdleDecision::from_inputs(
            self.snapshot.captured_at,
            self.last_input_at,
            looks_like_media_app(&self.snapshot.process_name),
            self.media_state,
            self.idle_seconds,
        )
    }
}
