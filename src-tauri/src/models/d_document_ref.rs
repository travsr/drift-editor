use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use super::d_document::DDocumentType;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDocumentRef {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: DDocumentType,
}
