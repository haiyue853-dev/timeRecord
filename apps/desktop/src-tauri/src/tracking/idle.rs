use anyhow::Result;
use chrono::{DateTime, Duration, Utc};

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

#[cfg(target_os = "windows")]
pub fn read_last_input_at(now: DateTime<Utc>) -> Result<DateTime<Utc>> {
    use std::mem::size_of;

    use windows::Win32::{
        System::SystemInformation::GetTickCount64,
        UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
    };

    unsafe {
        use anyhow::anyhow;

        let mut info = LASTINPUTINFO {
            cbSize: size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        if !GetLastInputInfo(&mut info).as_bool() {
            return Err(anyhow!("GetLastInputInfo failed"));
        }

        let tick_count = GetTickCount64();
        let idle_millis = tick_count.saturating_sub(info.dwTime as u64);

        Ok(now - Duration::milliseconds(idle_millis as i64))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn read_last_input_at(now: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(now)
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use chrono::Utc;

    use super::read_last_input_at;

    #[test]
    fn last_input_timestamp_is_not_in_future() {
        let now = Utc::now();
        let last_input_at = read_last_input_at(now).expect("idle timestamp should succeed");

        assert!(last_input_at <= now);
    }
}
