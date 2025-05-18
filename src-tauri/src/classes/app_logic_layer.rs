use std::{collections::HashMap, fs, sync::Arc, vec};

use anyhow::{bail, Context, Result};
use uuid::Uuid;

use crate::{
    models::{
        d_app_state::DAppState,
        d_document::{DDocument, DDocumentType},
        d_document_ref::DDocumentRef,
        d_tab::DTab,
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
        let app_state = DAppState {
            windows: HashMap::new(),
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

        let file_content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file at '{}'", file_path))?;

        let tab_id = Uuid::new_v4().to_string();

        let document_id = Uuid::new_v4().to_string();
        let document_title = file_path.to_string();

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
            title: document_title,
            r#type: DDocumentType::File,
            file_path: file_path.to_string(),
            status: crate::models::d_document::DDocumentStatus::SavedToFs,
            buffer: file_content,
        }];

        window_state.tabs.push(DTab {
            id: tab_id,
            title: file_path.to_string(),
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
            self.app_state = transaction.app_state;
            self.app_control_layer.emit_app_state(&self.app_state)
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
