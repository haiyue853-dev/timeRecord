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
