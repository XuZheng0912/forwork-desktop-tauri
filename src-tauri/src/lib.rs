// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod core;
mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_all_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
