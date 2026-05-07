pub mod app_state;
pub mod commands;
pub mod config;
pub mod db;
pub mod stats;
pub mod summary;
pub mod tracking;
pub mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(app_state::AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::stats::get_dashboard_stats,
            commands::summary::generate_boot_summary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
