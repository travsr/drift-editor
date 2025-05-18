use anyhow::Result;

use crate::models::d_app_state::DAppState;

pub trait AppControlLayer: Send + Sync {
    fn emit_app_state(&self, app_state: &DAppState) -> Result<()>;
    fn init_frontend(&self, app_state: &DAppState) -> Result<()>;
}
