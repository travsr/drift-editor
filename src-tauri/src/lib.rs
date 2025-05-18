mod classes;
mod models;
mod traits;

use std::sync::Arc;

use classes::{
    app_logic_layer::AppLogicLayer, tauri_command_layer, tauri_control_layer::TauriAppControlLayer,
};

use tauri::{async_runtime::Mutex, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tauri_command_layer::tc_tab_open,
            tauri_command_layer::tc_tab_close,
            tauri_command_layer::tc_tab_select
        ])
        .setup(|app| {
            let handle = app.handle();
            let app_control_layer = Arc::new(TauriAppControlLayer::new(handle.to_owned()));
            let app_logic_layer = Mutex::new(AppLogicLayer::new(app_control_layer.clone()));

            app.manage(app_logic_layer);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
