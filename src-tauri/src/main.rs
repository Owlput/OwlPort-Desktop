// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Ok;

mod event;
mod macros;
mod plugins;
extern crate owlnest;
extern crate tokio;

fn main() -> anyhow::Result<()> {
    setup_logging();
    let peer_manager = plugins::owlnest::setup_peer()?;
    tauri::Builder::default()
        .manage(peer_manager.clone())
        .plugin(plugins::owlnest::swarm_plugin::init(peer_manager.clone()))
        .plugin(plugins::owlnest::messaging::init(peer_manager.clone()))
        .plugin(plugins::owlnest::mdns::init(peer_manager.clone()))
        .plugin(plugins::owlnest::kad::init(peer_manager.clone()))
        .plugin(plugins::owlnest::blob::init())
        .plugin(plugins::owlnest::autonat::init(peer_manager.clone()))
        .plugin(plugins::owlnest::upnp::init(peer_manager.clone()))
        .plugin(plugins::owlnest::relay::init(peer_manager.clone()))
        .plugin(plugins::owlnest::advertise::init())
        .plugin(plugins::owlnest::gossipsub::init(peer_manager.clone()))
        .plugin(plugins::popup_test::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

fn setup_logging() {
    use owlnest::logging_prelude::*;
    use std::sync::Mutex;
    use tracing_subscriber::Layer;
    let time = chrono::Local::now().timestamp_micros();
    let log_file_handle = match std::fs::create_dir("./logs") {
        core::result::Result::Ok(_) => {
            std::fs::File::create(format!("./logs/{}.log", time)).unwrap()
        }
        Err(e) => {
            let error = format!("{:?}", e);
            if error.contains("AlreadyExists") {
                std::fs::File::create(format!("./logs/{}.log", time)).unwrap()
            } else {
                std::fs::File::create(format!("{}.log", time)).unwrap()
            }
        }
    };
    let filter = tracing_subscriber::filter::Targets::new().with_target("", Level::WARN);
    let layer = tracing_subscriber::fmt::Layer::default()
        .with_ansi(false)
        .with_writer(Mutex::new(log_file_handle))
        .with_filter(filter);
    let reg = tracing_subscriber::registry().with(layer);
    tracing::subscriber::set_global_default(reg).expect("you can only set global default once");
}
