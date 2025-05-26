/// tauri_command_layer
///
/// This is the interface between the Tauri Frontend and the Logic Layer.
use std::sync::Mutex;

use tauri::State;

use super::app_logic::app_logic_layer::{AppLogicLayer, TabLogic};

#[tauri::command]
pub fn tc_tab_open(
    window: tauri::Window,
    logic_layer: State<'_, Mutex<AppLogicLayer>>,
    file_path: String,
) -> Result<(), String> {
    let mut logic_layer = logic_layer.lock().unwrap();

    logic_layer
        .with_transaction(|layer| {
            layer.tab_open(window.label(), &file_path)?;
            Ok(())
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tc_tab_close(
    window: tauri::Window,
    logic_layer: State<'_, Mutex<AppLogicLayer>>,
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
    logic_layer: State<'_, Mutex<AppLogicLayer>>,
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

#[tauri::command]
pub fn tc_window_ready(
    window: tauri::Window,
    my_logic_layer: State<'_, Mutex<AppLogicLayer>>,
) -> Result<(), String> {
    let logic_layer = my_logic_layer.lock().unwrap();

    logic_layer
        .window_hydrate(window.label())
        .map_err(|e| e.to_string())
}
