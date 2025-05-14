use serde::{Serialize, Deserialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DSidebarType {
    Tree,
    Tabs,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DInterface {
    pub is_overlay_active: bool,
    pub sidebar: DSidebarType,
}
