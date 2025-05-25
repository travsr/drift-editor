mod classes;
mod helpers;
mod models;
mod traits;

use std::sync::{Arc, Mutex};

use classes::{
    app_logic_layer::AppLogicLayer, tauri_command_layer, tauri_control_layer::TauriAppControlLayer,
};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            let app_control_layer = Arc::new(TauriAppControlLayer::new(handle.to_owned()));

            let logic_layer = AppLogicLayer::new(app_control_layer.clone()).map_err(|e| {
                println!("Failed to initialize AppLogicLayer: {}", e);
                e
            })?;

            app.manage(Mutex::new(logic_layer));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_command_layer::tc_tab_open,
            tauri_command_layer::tc_tab_close,
            tauri_command_layer::tc_tab_select,
            tauri_command_layer::tc_window_ready
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
