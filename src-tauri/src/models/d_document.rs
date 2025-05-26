use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DDocumentStatus {
    New,
    SavedToFs,
    Modified,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DDocument {
    pub id: String,
    pub title: String,
    pub file_path: String,
    pub buffer: String,
    pub status: DDocumentStatus,
}
