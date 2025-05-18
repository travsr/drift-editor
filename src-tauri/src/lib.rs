mod classes;
mod models;
mod traits;

use std::sync::Arc;

use classes::{app_logic_layer::AppLogicLayer, tauri_control_layer::TauriAppControlLayer};

use tauri::{async_runtime::Mutex, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
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
