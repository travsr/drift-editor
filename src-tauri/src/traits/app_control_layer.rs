use anyhow::Result;
use tauri::AppHandle;

use crate::models::d_app_state::DAppState;

pub trait AppControlLayer: Send + Sync {
    fn emit_event(&self, window_id: &str, tab_id: &str) -> Result<()>;
    fn init_frontend(&self, app_state: &DAppState) -> Result<()>;
}
