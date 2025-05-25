use anyhow::Result;

use crate::models::{
    d_app_state::DAppState, d_window_state::DWindowState, d_window_state_scope::DWindowStateScope,
};

pub trait AppControlLayer: Send + Sync {
    fn emit_app_state(&self, app_state: &DAppState) -> Result<()>;
    fn emit_window_state(
        &self,
        scope: DWindowStateScope,
        window_state: &DWindowState,
    ) -> Result<()>;
    fn init_frontend(&self, app_state: &DAppState) -> Result<()>;
}
