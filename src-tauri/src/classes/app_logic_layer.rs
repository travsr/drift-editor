use std::{collections::HashMap, sync::Arc, vec};

use crate::{
    models::{d_app_state::DAppState, d_tab::DTab},
    traits::app_control_layer::AppControlLayer,
};

pub struct AppLogicLayer {
    app_state: DAppState,
    app_control_layer: Arc<dyn AppControlLayer>,
}

impl AppLogicLayer {
    pub fn new(app_control_layer: Arc<dyn AppControlLayer>) -> Self {
        let app_state = DAppState {
            windows: HashMap::new(),
        };

        app_control_layer.init_frontend(&app_state);

        AppLogicLayer {
            app_state,
            app_control_layer,
        }
    }

    pub fn write_document(&self, window_id: &str, document_id: &str) {}
    pub fn open_tab(&mut self, window_id: &str, document_id: &str) {
        if let Some(window) = self.app_state.windows.get_mut(window_id) {
            window.tabs.push(DTab {
                id: "".to_string(),
                title: "".to_string(),
                document_refs: vec![],
                is_selected: true,
            });
        } else {
            // handle missing window_id gracefully
        }
    }
    pub fn close_tab(&self, window_id: &str, tab_id: &str) {}
    pub fn select_tab(&self, window_id: &str, tab_id: &str) {}
}
