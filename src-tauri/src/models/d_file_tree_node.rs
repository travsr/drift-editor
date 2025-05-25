use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DFileTreeNode {
    pub id: String,
    pub name: String,
    pub is_expanded: Option<bool>,
    pub children: Vec<String>,
}
