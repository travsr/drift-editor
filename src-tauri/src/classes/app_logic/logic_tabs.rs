use std::path::Path;

use anyhow::{bail, Context, Result};
use uuid::Uuid;

use crate::models::{
    d_content_item::{DContentItem, DContentItemRef, DContentItemType},
    d_document::{DDocument, DDocumentStatus},
    d_tab::DTab,
};

use super::app_logic_layer::{AppLogicLayer, DocumentCacheLogic, TabLogic};

impl TabLogic for AppLogicLayer {
    fn tab_open(&mut self, window_id: &str, file_path: &str) -> Result<()> {
        let document = {
            let document = self.document_cache_retrieve(file_path)?;
            document.clone()
        };

        let app_state = self.get_transaction_app_state_mut()?;
        let window_state = app_state.get_window_state_mut(window_id)?;

        // Deselect any selected tabs
        for tab in &mut window_state.tabs {
            tab.is_selected = false;
        }

        // Check for an existing tab
        let existing_tab = window_state.tabs.iter_mut().find(|tab| {
            tab.content_item_refs.iter().any(|content_ref| {
                if let Some(doc_file_path) = &content_ref.document_file_path {
                    doc_file_path == file_path
                } else {
                    false
                }
            })
        });

        let content_item_ref = DContentItemRef {
            r#type: DContentItemType::File,
            document_file_path: Some(file_path.to_owned()),
        };

        let content_item = DContentItem {
            r#type: DContentItemType::File,
            document: Some(document),
        };

        window_state.content.content_items = vec![content_item];

        if let Some(existing_tab) = existing_tab {
            existing_tab.is_selected = true;
        } else {
            window_state.tabs.insert(
                0,
                DTab {
                    id: Uuid::new_v4().to_string(),
                    title: window_state.content.content_items[0]
                        .document
                        .as_ref()
                        .unwrap()
                        .title
                        .clone(),
                    content_item_refs: vec![content_item_ref],
                    is_selected: true,
                },
            );
        }

        Ok(())
    }

    fn tab_close(&mut self, window_id: &str, tab_id: &str) -> Result<()> {
        let app_state = self.get_transaction_app_state_mut()?;
        let window_state = app_state.get_window_state_mut(window_id)?;
        let does_tab_exist = window_state.tabs.iter().any(|tab| tab.id == tab_id);

        if !does_tab_exist {
            bail!("Tab ID '{}' not found in window '{}'", tab_id, window_id);
        }

        window_state.tabs.retain(|tab| tab.id != tab_id);
        window_state.content.content_items = vec![];

        Ok(())
    }

    fn tab_select(&mut self, window_id: &str, tab_id: &str) -> Result<()> {
        let content_items = {
            let app_state = self.get_transaction_app_state()?;
            let window_state = app_state.get_window_state(window_id)?;
            let found_tab = window_state.tabs.iter().find(|tab| tab.id == tab_id);

            if let Some(found_tab) = found_tab {
                let file_paths: Vec<String> = found_tab
                    .content_item_refs
                    .iter()
                    .filter_map(|item| item.document_file_path.clone())
                    .collect();

                let content_items: Vec<DContentItem> = file_paths
                    .iter()
                    .map(|doc_path| DContentItem {
                        r#type: DContentItemType::File,
                        document: self.document_cache_retrieve(doc_path).ok().cloned(),
                    })
                    .collect();

                content_items

                // window_state.content.content_items = content_items;
            } else {
                bail!("Tab not found.")
            }
        };

        let app_state_mut = self.get_transaction_app_state_mut()?;
        let window_state_mut = app_state_mut.get_window_state_mut(window_id)?;

        for tab in &mut window_state_mut.tabs {
            if tab.id == tab_id {
                tab.is_selected = true
            } else {
                tab.is_selected = false
            }
        }

        window_state_mut.content.content_items = content_items;

        Ok(())
    }
}
