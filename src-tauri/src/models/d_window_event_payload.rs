use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{
    d_file_tree_node::DFileTreeNode,
    d_tab::DTab,
    d_window_state::{DWindowContent, DWindowState},
    d_window_state_scope::DWindowStateScope,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayload {
    pub scope: DWindowStateScope,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadAll {
    pub scope: DWindowStateScope,
    pub window_state: DWindowState,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadTabs {
    pub scope: DWindowStateScope,
    pub tabs: Vec<DTab>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadContent {
    pub scope: DWindowStateScope,
    pub content: DWindowContent,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadFileMap {
    pub scope: DWindowStateScope,
    pub file_map: HashMap<String, DFileTreeNode>,
}
