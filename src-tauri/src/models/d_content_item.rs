use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::d_document::DDocument;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DContentItemType {
    File,
    Terminal,
    Settings,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DContentItemRef {
    #[serde(rename = "type")]
    pub r#type: DContentItemType,

    pub document_file_path: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DContentItem {
    #[serde(rename = "type")]
    pub r#type: DContentItemType,

    pub document: Option<DDocument>,
}
