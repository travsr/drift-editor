use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::d_document::{DDocumentStatus, DDocumentType};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDocumentRef {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: DDocumentType,
    pub file_path: String,
    pub status: DDocumentStatus,
}
