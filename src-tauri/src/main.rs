#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(associated_type_bounds)]
mod net;
mod plugins;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(crate::plugins::grpc::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
