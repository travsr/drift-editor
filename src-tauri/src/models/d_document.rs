use serde::{Serialize, Deserialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DDocumentType {
    File,
    Terminal,
    Settings,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DDocumentStatus {
    New,
    SavedToFs,
    Modified,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDocument {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: DDocumentType,
    pub file_path: String,
    pub buffer: String,
    pub status: DDocumentStatus,
}
