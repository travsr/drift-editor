use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{
    d_file_tree_node::DFileTreeNode,
    d_tab::DTab,
    d_window_event_scope::DWindowEventScope,
    d_window_state::{DWindowContent, DWindowState},
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayload {
    pub scope: DWindowEventScope,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadAll {
    pub scope: DWindowEventScope,
    pub window_state: DWindowState,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadTabs {
    pub scope: DWindowEventScope,
    pub tabs: Vec<DTab>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadContent {
    pub scope: DWindowEventScope,
    pub content: DWindowContent,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DWindowEventPayloadFileList {
    pub scope: DWindowEventScope,
    pub file_list: Vec<DFileTreeNode>,
}
