/// tauri_command_layer
///
/// This is the interface between the Tauri Frontend and the Logic Layer.
use std::sync::{Arc, Mutex};

use super::app_logic_layer::AppLogicLayer;

#[tauri::command]
pub fn tc_tab_open(
    window: tauri::Window,
    logic_layer: tauri::State<'_, Arc<Mutex<AppLogicLayer>>>,
    document_id: String,
) -> Result<(), String> {
    let mut logic_layer = logic_layer.lock().unwrap();

    logic_layer
        .with_transaction(|layer| {
            layer.tab_open(window.label(), &document_id)?;
            Ok(())
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tc_tab_close(
    window: tauri::Window,
    logic_layer: tauri::State<'_, Arc<Mutex<AppLogicLayer>>>,
    tab_id: String,
) -> Result<(), String> {
    let mut logic_layer = logic_layer.lock().unwrap();

    logic_layer
        .with_transaction(|layer| {
            layer.tab_close(window.label(), &tab_id)?;
            Ok(())
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tc_tab_select(
    window: tauri::Window,
    logic_layer: tauri::State<'_, Arc<Mutex<AppLogicLayer>>>,
    tab_id: String,
) -> Result<(), String> {
    let mut logic_layer = logic_layer.lock().unwrap();

    logic_layer
        .with_transaction(|layer| {
            layer.tab_select(window.label(), &tab_id)?;
            Ok(())
        })
        .map_err(|e| e.to_string())
}
