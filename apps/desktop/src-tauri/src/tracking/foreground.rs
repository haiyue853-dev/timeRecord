use anyhow::{anyhow, Result};
use crate::tracking::session::ForegroundSnapshot;

pub trait ForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot>;
}

pub struct WindowsForegroundSource;

impl ForegroundSource for WindowsForegroundSource {
    fn snapshot(&self) -> Result<ForegroundSnapshot> {
        Err(anyhow!("foreground window collection is not wired yet"))
    }
}
