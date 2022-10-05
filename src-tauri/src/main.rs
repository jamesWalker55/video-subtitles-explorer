#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod vtt;
mod vtt_locator;

use vtt::read_vtt;
use vtt_locator::locate_vtt;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![locate_vtt, read_vtt])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri appl}
