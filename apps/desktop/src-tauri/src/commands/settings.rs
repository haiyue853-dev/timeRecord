use tauri::State;

use crate::{app_state::AppState, config::AppSettings};

pub async fn load_settings(state: AppState) -> anyhow::Result<AppSettings> {
    Ok(state.settings())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    load_settings(state.inner().clone())
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    input: AppSettings,
) -> Result<AppSettings, String> {
    state
        .replace_settings(input.clone())
        .map_err(|error| error.to_string())?;
    Ok(input)
}
