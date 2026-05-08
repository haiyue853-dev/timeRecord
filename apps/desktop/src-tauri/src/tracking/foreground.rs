use anyhow::{anyhow, Result};
use chrono::Utc;

use crate::tracking::session::ForegroundSnapshot;

pub trait ForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot>;
}

pub struct WindowsForegroundSource;

#[cfg(target_os = "windows")]
impl ForegroundSource for WindowsForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot> {
        use std::path::Path;

        use windows::{
            core::PWSTR,
            Win32::{
                Foundation::{CloseHandle, HWND},
                System::Threading::{
                    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
                    PROCESS_QUERY_LIMITED_INFORMATION,
                },
                UI::WindowsAndMessaging::{
                    GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
                    GetWindowThreadProcessId,
                },
            },
        };

        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.0.is_null() {
                return Err(anyhow!("no foreground window available"));
            }

            let title_length = GetWindowTextLengthW(hwnd);
            let mut title_buffer = vec![0u16; title_length as usize + 1];
            let copied = GetWindowTextW(hwnd, &mut title_buffer);
            let window_title = String::from_utf16_lossy(&title_buffer[..copied as usize])
                .trim()
                .to_string();

            let mut process_id = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut process_id));
            if process_id == 0 {
                return Err(anyhow!("foreground process id unavailable"));
            }

            let process_name = match OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION,
                false,
                process_id,
            ) {
                Ok(process) => {
                    let mut path_buffer = vec![0u16; 512];
                    let mut buffer_length = path_buffer.len() as u32;
                    let full_path = match QueryFullProcessImageNameW(
                        process,
                        PROCESS_NAME_WIN32,
                        PWSTR(path_buffer.as_mut_ptr()),
                        &mut buffer_length,
                    ) {
                        Ok(_) => String::from_utf16_lossy(&path_buffer[..buffer_length as usize]),
                        Err(_) => format!("process-{process_id}.exe"),
                    };
                    let _ = CloseHandle(process);

                    Path::new(&full_path)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("unknown.exe")
                        .to_string()
                }
                Err(_) => format!("process-{process_id}.exe"),
            };

            let app_name = process_name.clone();
            let window_title = if window_title.is_empty() {
                format!("{app_name} 正在前台运行")
            } else {
                window_title
            };

            Ok(ForegroundSnapshot::new(
                hwnd.0 as i64,
                process_name,
                app_name,
                window_title,
                Utc::now(),
            ))
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl ForegroundSource for WindowsForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot> {
        Err(anyhow!("foreground window collection is only implemented on Windows"))
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use chrono::Utc;

    use super::{ForegroundSource, WindowsForegroundSource};

    #[test]
    fn snapshot_reads_live_foreground_window() {
        let source = WindowsForegroundSource;
        let snapshot = source.snapshot().expect("foreground snapshot should succeed");

        assert!(!snapshot.process_name.trim().is_empty());
        assert!(!snapshot.app_name.trim().is_empty());
        assert!(!snapshot.window_title.trim().is_empty());
        assert!(snapshot.captured_at <= Utc::now());
    }
}
