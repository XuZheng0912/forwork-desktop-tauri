// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod core;
mod environment;
mod persistence;

use crate::environment::init_environment;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_environment();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_all_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
