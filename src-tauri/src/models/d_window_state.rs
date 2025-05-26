use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{
    d_content_item::DContentItem, d_file_tree_node::DFileTreeNode, d_interface::DInterface,
    d_tab::DTab,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowContent {
    pub content_items: Vec<DContentItem>,
    // pub layout_type // This is where other meta properties could go for each tab (Like layout, frame widths, etc)
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowState {
    pub id: String,
    pub tabs: Vec<DTab>,
    pub content: DWindowContent,
    pub file_path: String,
    pub file_list: Vec<DFileTreeNode>,
    pub ui: DInterface,
}
