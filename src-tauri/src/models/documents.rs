use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DDocumentType {
    File,
    Terminal,
    Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DDocument {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub doc_type: DDocumentType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DDocumentFileStatus {
    New,
    SavedToFs,
    Modified,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DDocumentFile {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub doc_type: DDocumentType,
    pub file_path: String,
    pub buffer: String,
    pub status: DDocumentFileStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DDocumentRef {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub doc_type: DDocumentType,
}

// Same as DDocument but separate type
#[derive(Debug, Serialize, Deserialize)]
pub struct DDocumentTerminal {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub doc_type: DDocumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DTab {
    pub title: String,
    pub document_refs: Vec<DDocumentRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DTabViewProps {
    pub tabs: Vec<DTab>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DDocumentUnion {
    Basic(DDocument),
    File(DDocumentFile),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DContentViewProps {
    pub documents: Vec<DDocumentUnion>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DDetailViewType {
    FileInfo,
    Explorer,
    Diagnostics,
    Git,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DDetailViewProps {
    #[serde(rename = "type")]
    pub detail_type: DDetailViewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DUIState {
    pub tab_view: DTabViewProps,
    pub content_view: DContentViewProps,
    pub detail_view: DDetailViewProps,
}
