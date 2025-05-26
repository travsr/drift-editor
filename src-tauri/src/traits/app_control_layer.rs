use anyhow::Result;

use crate::models::{
    d_app_state::DAppState, d_window_event_scope::DWindowEventScope, d_window_state::DWindowState,
};

pub trait AppControlLayer: Send + Sync {
    fn emit_app_state(&self, app_state: &DAppState) -> Result<()>;
    fn emit_window_state(
        &self,
        scope: DWindowEventScope,
        window_state: &DWindowState,
    ) -> Result<()>;
    fn init_frontend(&self, app_state: &DAppState) -> Result<()>;
}
