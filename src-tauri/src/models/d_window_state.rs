use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use super::{d_document::DDocument, d_file_tree_node::DFileTreeNode, d_interface::DInterface, d_tab::DTab};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWindowContent {
    pub documents: Vec<DDocument>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWindowState {
    pub id: String,
    pub tabs: Vec<DTab>,
    pub content: DWindowContent,
    pub file_tree: DFileTreeNode,
    pub ui: DInterface,
}
