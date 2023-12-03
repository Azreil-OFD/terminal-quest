// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
static mut ARGS: Vec<String> = Vec::new();

#[tauri::command]
fn get_launch_parameters() -> &'static [String] {
    unsafe {
        ARGS = env::args().collect();
        &ARGS[1..]
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_launch_parameters])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
