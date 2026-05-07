use timerecord_lib::{app_state::AppState, commands::settings::load_settings};

#[test]
fn returns_default_settings() {
    let state = AppState::new_for_test().unwrap();
    let settings = tauri::async_runtime::block_on(load_settings(state)).unwrap();

    assert_eq!(settings.idle_seconds, 180);
    assert!(!settings.ai_enabled);
}
