use serde::{Serialize, Deserialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DFileTreeNodeType {
    File,
    Folder,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DFileTreeNode {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub r#type: DFileTreeNodeType,
    pub is_expanded: Option<bool>,
    pub children: Option<Vec<DFileTreeNode>>,
}