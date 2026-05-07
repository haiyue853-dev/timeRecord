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
