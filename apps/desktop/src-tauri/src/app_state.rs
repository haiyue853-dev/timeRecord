use std::sync::{Arc, Mutex};

use crate::config::AppSettings;

#[derive(Clone)]
pub struct AppState {
    settings: Arc<Mutex<AppSettings>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(Mutex::new(AppSettings::default())),
        }
    }

    pub fn new_for_test() -> anyhow::Result<Self> {
        Ok(Self::new())
    }

    pub fn settings(&self) -> AppSettings {
        self.settings.lock().expect("settings mutex poisoned").clone()
    }

    pub fn replace_settings(&self, settings: AppSettings) {
        *self.settings.lock().expect("settings mutex poisoned") = settings;
    }
}
