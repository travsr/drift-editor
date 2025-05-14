use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use super::d_document_ref::DDocumentRef;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DTab {
    pub id: String,
    pub title: String,
    pub document_refs: Vec<DDocumentRef>,
    pub is_selected: bool,
}
