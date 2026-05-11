pub mod app_state;
pub mod commands;
pub mod config;
pub mod db;
pub mod history;
pub mod stats;
pub mod summary;
pub mod tracking;
pub mod tray;

use app_state::AppState;
use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new();
    let setup_state = state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(state)
        .setup(move |app| {
            if should_exit_for_existing_instance() {
                std::process::exit(0);
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_data_dir = app.path().app_data_dir()?;
            setup_state.initialize(app_data_dir)?;
            setup_state.start_tracking_loop();
            tray::create_tray_icon(app.handle())?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::stats::get_dashboard_stats,
            commands::summary::generate_boot_summary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
fn should_exit_for_existing_instance() -> bool {
    use windows::{
        core::PCWSTR,
        Win32::{
            Foundation::{ERROR_ALREADY_EXISTS, GetLastError, HWND},
            System::Threading::CreateMutexW,
            UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOW},
        },
    };

    let mutex_name = to_wide("TimeRecord.SingleInstance");
    let handle = match unsafe { CreateMutexW(None, false, PCWSTR(mutex_name.as_ptr())) } {
        Ok(handle) => handle,
        Err(_) => return false,
    };
    if handle.is_invalid() {
        return false;
    }

    let already_exists = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
    if already_exists {
        let title = to_wide("TimeRecord");
        let hwnd: HWND = unsafe { FindWindowW(None, PCWSTR(title.as_ptr())) }.unwrap_or_default();
        if !hwnd.0.is_null() {
            unsafe {
                let _ = ShowWindow(hwnd, SW_RESTORE);
                let _ = ShowWindow(hwnd, SW_SHOW);
                let _ = SetForegroundWindow(hwnd);
            }
        }
    }

    already_exists
}

#[cfg(not(target_os = "windows"))]
fn should_exit_for_existing_instance() -> bool {
    false
}

#[cfg(target_os = "windows")]
fn to_wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
