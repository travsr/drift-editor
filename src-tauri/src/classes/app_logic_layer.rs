use std::{collections::HashMap, path::Path, sync::Arc, vec};

use anyhow::{bail, Result};
use uuid::Uuid;

use crate::{
    helpers::{create_file_tree_vec::create_file_tree_vec, diff_scopes::diff_scopes},
    models::{
        d_app_state::DAppState,
        d_document::{DDocument, DDocumentType},
        d_document_ref::DDocumentRef,
        d_interface::DInterface,
        d_tab::DTab,
        d_window_state::{DWindowContent, DWindowState},
        d_window_state_scope::DWindowStateScope,
    },
    traits::app_control_layer::AppControlLayer,
};

pub struct AppLogicTransaction {
    app_state: DAppState,
}

pub struct AppLogicLayer {
    app_state: DAppState,
    transaction: Option<AppLogicTransaction>,
    app_control_layer: Arc<dyn AppControlLayer>,
}

impl AppLogicLayer {
    pub fn new(app_control_layer: Arc<dyn AppControlLayer>) -> Result<Self> {
        let mut window_map = HashMap::new();

        window_map.insert(
            "window-1".to_string(),
            DWindowState {
                id: "window-1".to_string(),
                content: DWindowContent {
                    documents: vec![DDocument {
                        id: "doc-1".to_string(),
                        r#type: DDocumentType::File,
                        title: "document 1".to_string(),
                        file_path: "".to_string(),
                        buffer: "My buffer".to_string(),
                        status: crate::models::d_document::DDocumentStatus::Modified,
                    }],
                },
                tabs: vec![DTab {
                    id: "tab-1".to_string(),
                    title: "My Tab".to_string(),
                    document_refs: vec![],
                    is_selected: true,
                }],
                file_path: "/Users/travis/Developer".to_string(),
                file_list: create_file_tree_vec("/Users/travis/Developer"),
                ui: DInterface {
                    is_overlay_active: false,
                    sidebar: crate::models::d_interface::DSidebarType::Tree,
                },
            },
        );
        window_map.insert(
            "window-2".to_string(),
            DWindowState {
                id: "window-2".to_string(),
                content: DWindowContent { documents: vec![] },
                tabs: vec![],
                file_path: "".to_string(),
                file_list: vec![],
                ui: DInterface {
                    is_overlay_active: false,
                    sidebar: crate::models::d_interface::DSidebarType::Tabs,
                },
            },
        );
        let app_state = DAppState {
            windows: window_map,
        };

        app_control_layer.init_frontend(&app_state)?;

        Ok(AppLogicLayer {
            app_state,
            transaction: None,
            app_control_layer,
        })
    }

    pub fn tab_open(&mut self, window_id: &str, file_path: &str) -> Result<()> {
        let app_state = self.get_transaction_app_state_mut()?;
        let window_state = app_state.get_window_state_mut(window_id)?;

        // let file_content = fs::read_to_string(file_path)
        //     .with_context(|| format!("Failed to read file at '{}'", file_path))?;
        let file_content = "file content".to_string();

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

        let document_ref = DDocumentRef {
            id: document_id.clone(),
            title: document_title.clone(),
            r#type: DDocumentType::File,
            file_path: file_path.to_string(),
            status: crate::models::d_document::DDocumentStatus::SavedToFs,
        };

        window_state.content.documents = vec![DDocument {
            id: document_id,
            title: document_title.clone(),
            r#type: DDocumentType::File,
            file_path: file_path.to_string(),
            status: crate::models::d_document::DDocumentStatus::SavedToFs,
            buffer: file_content,
        }];

        window_state.tabs.push(DTab {
            id: tab_id,
            title: document_title,
            document_refs: vec![document_ref],
            is_selected: true,
        });

        Ok(())
    }

    pub fn tab_close(&mut self, window_id: &str, tab_id: &str) -> Result<()> {
        let app_state = self.get_transaction_app_state_mut()?;
        let window_state = app_state.get_window_state_mut(window_id)?;
        let does_tab_exist = window_state.tabs.iter().any(|tab| tab.id == tab_id);

        if !does_tab_exist {
            bail!("Tab ID '{}' not found in window '{}'", tab_id, window_id);
        }

        window_state.tabs.retain(|tab| tab.id != tab_id);
        window_state.content.documents = vec![];

        Ok(())
    }

    pub fn tab_select(&mut self, window_id: &str, tab_id: &str) -> Result<()> {
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

    pub fn window_hydrate(&self, window_id: &str) -> Result<()> {
        if let Some(window) = self.app_state.windows.get(window_id) {
            self.app_control_layer
                .emit_window_state(DWindowStateScope::All, window)?;
        }
        Ok(())
    }

    fn get_transaction_app_state_mut(&mut self) -> Result<&mut DAppState> {
        if let Some(ref mut transaction) = self.transaction {
            Ok(&mut transaction.app_state)
        } else {
            bail!("No staged app state is present.");
        }
    }

    pub fn begin_transaction(&mut self) -> Result<()> {
        if self.transaction.is_some() {
            bail!("Transaction already in progress.");
        }

        self.transaction = Some(AppLogicTransaction {
            app_state: self.app_state.clone(),
        });
        Ok(())
    }

    pub fn commit(&mut self) -> Result<()> {
        if let Some(transaction) = self.transaction.take() {
            let next_app_state = transaction.app_state.clone();

            for (window_id, next_window_state) in next_app_state.windows.clone() {
                if let Some(prev_window_state) = self.app_state.windows.get(&window_id) {
                    let scopes = diff_scopes(prev_window_state, &next_window_state);

                    for scope in scopes {
                        self.app_control_layer
                            .emit_window_state(scope, &next_window_state)?;
                    }
                } else {
                    println!("Falling back to emit the ALL scope");
                    self.app_control_layer
                        .emit_window_state(DWindowStateScope::All, &next_window_state)?;
                }
            }

            self.app_state = next_app_state;

            Ok(())
        } else {
            bail!("No transaction to commit");
        }
    }

    pub fn rollback(&mut self) {
        self.transaction = None;
    }

    pub fn with_transaction<F>(&mut self, transaction_work: F) -> Result<()>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        self.begin_transaction()?;
        if let Err(e) = transaction_work(self) {
            self.rollback();
            return Err(e);
        }
        self.commit()
    }
}
