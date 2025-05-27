use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::models::d_document::{DDocument, DDocumentStatus};

use super::app_logic_layer::{AppLogicLayer, DocumentCacheLogic};

impl DocumentCacheLogic for AppLogicLayer {
    fn document_cache_retrieve(&mut self, file_path: &str) -> Result<&DDocument> {
        if !self.document_cache.contains_key(file_path) {
            let file_content = fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read file at '{}'", file_path))?;

            let title = Path::new(file_path)
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| file_path.to_string());

            self.document_cache.insert(
                file_path.to_owned(),
                DDocument {
                    title,
                    file_path: file_path.to_owned(),
                    buffer: file_content,
                    status: DDocumentStatus::SavedToFs,
                },
            );
        }

        self.document_cache
            .get(file_path)
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve document after insert"))
    }

    fn document_cache_flush(&mut self) -> Result<()> {
        self.document_cache.clear();
        Ok(())
    }
}
