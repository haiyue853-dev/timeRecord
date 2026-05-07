#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaPlaybackState {
    Playing,
    Paused,
    Unknown,
}

pub fn looks_like_media_app(process_name: &str) -> bool {
    matches!(
        process_name,
        "chrome.exe" | "msedge.exe" | "firefox.exe" | "vlc.exe" | "potplayer.exe"
    )
}
