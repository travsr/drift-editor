use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::d_content_item::DContentItemRef;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DTab {
    pub id: String,
    pub title: String,
    pub is_selected: bool,
    pub content_item_refs: Vec<DContentItemRef>,
}
