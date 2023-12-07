// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;
mod event;
extern crate owlnest;
extern crate tokio;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    setup_logging();
    let peer_manager = plugins::owlnest::setup_peer();
    tauri::Builder::default()
        .plugin(plugins::owlnest::swarm_plugin::init(peer_manager.clone()))
        .plugin(plugins::popup_test::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_logging() {
    use owlnest::logging_prelude::*;
    use tracing_subscriber::Layer;
    use std::sync::Mutex;
    let time = chrono::Local::now().timestamp_micros();
    let log_file_handle = match std::fs::create_dir("./logs") {
        Ok(_) => std::fs::File::create(format!("./logs/{}.log", time)).unwrap(),
        Err(e) => {
            let error = format!("{:?}", e);
            if error.contains("AlreadyExists") {
                std::fs::File::create(format!("./logs/{}.log", time)).unwrap()
            } else {
                std::fs::File::create(format!("{}.log", time)).unwrap()
            }
        }
    };
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("owlnest", Level::DEBUG)
        .with_target("rustyline", LevelFilter::ERROR)
        .with_target("libp2p_noise", Level::WARN)
        .with_target("libp2p_mdns", Level::DEBUG)
        .with_target("hickory_proto", Level::WARN)
        .with_target("", Level::TRACE);
    let layer = tracing_subscriber::fmt::Layer::default()
        .with_ansi(false)
        .with_writer(Mutex::new(log_file_handle))
        .with_filter(filter);
    let reg = tracing_subscriber::registry().with(layer);
    tracing::subscriber::set_global_default(reg).expect("you can only set global default once");
    LogTracer::init().unwrap()
}