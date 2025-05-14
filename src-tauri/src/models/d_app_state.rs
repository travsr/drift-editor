use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use super::d_window_state::DWindowState;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAppState {
    pub windows: HashMap<String, DWindowState>,
}
