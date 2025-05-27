use std::{collections::HashMap, sync::Arc, vec};

use anyhow::{bail, Result};

use crate::{
    helpers::{
        create_file_tree_vec::create_file_tree_vec, get_scopes_for_diff::get_scopes_for_diff,
    },
    models::{
        d_app_state::DAppState,
        d_content_item::{DContentItem, DContentItemType},
        d_document::DDocument,
        d_interface::DInterface,
        d_tab::DTab,
        d_window_event_scope::DWindowEventScope,
        d_window_state::{DWindowContent, DWindowState},
    },
    traits::app_control_layer::AppControlLayer,
};

pub struct AppLogicTransaction {
    app_state: DAppState,
}

pub struct AppLogicLayer {
    app_state: DAppState,
    app_control_layer: Arc<dyn AppControlLayer>,
    pub document_cache: HashMap<String, DDocument>,
    pub transaction: Option<AppLogicTransaction>,
}

pub trait TabLogic {
    fn tab_open(&mut self, window_id: &str, file_path: &str) -> Result<()>;
    fn tab_close(&mut self, window_id: &str, tab_id: &str) -> Result<()>;
    fn tab_select(&mut self, window_id: &str, tab_id: &str) -> Result<()>;
}

pub trait DocumentCacheLogic {
    fn document_cache_retrieve(&mut self, file_path: &str) -> Result<&DDocument>;
    fn document_cache_flush(&mut self) -> Result<()>;
}

impl AppLogicLayer {
    pub fn new(app_control_layer: Arc<dyn AppControlLayer>) -> Result<Self> {
        let mut window_map = HashMap::new();

        window_map.insert(
            "window-1".to_string(),
            DWindowState {
                id: "window-1".to_string(),
                content: DWindowContent {
                    content_items: vec![DContentItem {
                        r#type: DContentItemType::File,
                        document: None,
                    }],
                },
                tabs: vec![DTab {
                    id: "tab-1".to_string(),
                    title: "My Tab".to_string(),
                    content_item_refs: vec![],
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
                content: DWindowContent {
                    content_items: vec![],
                },
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
            document_cache: HashMap::new(),
        })
    }

    pub fn window_hydrate(&self, window_id: &str) -> Result<()> {
        if let Some(window) = self.app_state.windows.get(window_id) {
            self.app_control_layer
                .emit_window_state(DWindowEventScope::All, window)?;
        }
        Ok(())
    }

    pub fn get_transaction_app_state_mut(&mut self) -> Result<&mut DAppState> {
        if let Some(ref mut transaction) = self.transaction {
            Ok(&mut transaction.app_state)
        } else {
            bail!("No staged app state is present.");
        }
    }

    pub fn get_transaction_app_state(&self) -> Result<&DAppState> {
        if let Some(ref transaction) = self.transaction {
            Ok(&transaction.app_state)
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
                    let scopes = get_scopes_for_diff(prev_window_state, &next_window_state);

                    for scope in scopes {
                        self.app_control_layer
                            .emit_window_state(scope, &next_window_state)?;
                    }
                } else {
                    println!("Falling back to emit the ALL scope");
                    self.app_control_layer
                        .emit_window_state(DWindowEventScope::All, &next_window_state)?;
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
