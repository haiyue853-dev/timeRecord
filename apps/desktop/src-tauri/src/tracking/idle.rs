use chrono::{DateTime, Utc};

use super::media::MediaPlaybackState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleDecision {
    pub count_as_active: bool,
    pub media_playback_kept_alive: bool,
}

impl IdleDecision {
    pub fn from_inputs(
        now: DateTime<Utc>,
        last_input_at: DateTime<Utc>,
        foreground_is_media_app: bool,
        media_state: MediaPlaybackState,
        idle_seconds: u64,
    ) -> Self {
        let idle_for = (now - last_input_at).num_seconds().max(0) as u64;
        if idle_for < idle_seconds {
            return Self {
                count_as_active: true,
                media_playback_kept_alive: false,
            };
        }

        if foreground_is_media_app && matches!(media_state, MediaPlaybackState::Playing) {
            return Self {
                count_as_active: true,
                media_playback_kept_alive: true,
            };
        }

        Self {
            count_as_active: false,
            media_playback_kept_alive: false,
        }
    }
}
