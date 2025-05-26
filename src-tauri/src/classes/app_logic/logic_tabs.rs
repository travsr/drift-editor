use std::path::Path;

use anyhow::{bail, Result};
use uuid::Uuid;

use crate::models::{
    d_content_item::{DContentItem, DContentItemRef, DContentItemType},
    d_tab::DTab,
};

use super::app_logic_layer::{AppLogicLayer, TabLogic};

impl TabLogic for AppLogicLayer {
    fn tab_open(&mut self, window_id: &str, file_path: &str) -> Result<()> {
        let app_state = self.get_transaction_app_state_mut()?;
        let window_state = app_state.get_window_state_mut(window_id)?;

        // let file_content = fs::read_to_string(file_path)
        //     .with_context(|| format!("Failed to read file at '{}'", file_path))?;
        // let file_content = "file content".to_string();

        let tab_id = Uuid::new_v4().to_string();

        let path = Path::new(file_path);
        let document_title = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| file_path.to_string());

        // let document_title = "Test title".to_string();

        let document_id = Uuid::new_v4().to_string();

        for tab in &mut window_state.tabs {
            tab.is_selected = false;
        }

        let content_item_ref = DContentItemRef {
            r#type: DContentItemType::File,
            document_id: Some(document_id.clone()),
        };

        window_state.content.content_items = vec![DContentItem {
            r#type: DContentItemType::File,
            document: None,
        }];

        window_state.tabs.insert(
            0,
            DTab {
                id: tab_id,
                title: document_title,
                content_item_refs: vec![content_item_ref],
                is_selected: true,
            },
        );

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
        let app_state = self.get_transaction_app_state_mut()?;
        let window_state = app_state.get_window_state_mut(window_id)?;

        for tab in &mut window_state.tabs {
            tab.is_selected = false
        }

        let found_tab = window_state.tabs.iter_mut().find(|tab| tab.id == tab_id);

        if let Some(found_tab) = found_tab {
            found_tab.is_selected = true;
        } else {
            bail!("Tab not found.")
        }

        Ok(())
    }
}
