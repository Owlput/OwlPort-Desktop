use super::*;
use ::owlnest::net::p2p::SwarmConfig;
use tauri::generate_handler;
use tokio::{fs, io::AsyncWriteExt};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("owlport-config-writer")
        .setup(|_app, _api| Ok(()))
        .invoke_handler(generate_handler![write_default_config,])
        .build()
}

/// Write the default config to the given path.
/// The path must point to a file. Won't overwrite.
#[tauri::command]
async fn write_default_config(path: String) -> Result<(), String> {
    use std::path::PathBuf;
    let path = PathBuf::from(path);
    if path.exists() {
        return Err("Example file already exist.".into());
    }
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
    {
        Ok(file) => file,
        Err(e) => return Err(format!("Cannot open file {path:?}: {}", e.to_string())),
    };
    let default_config = SwarmConfig::default();
    if let Err(e) = file
        .write(
            toml::to_string_pretty(&default_config)
                .expect("Serialization to succeed")
                .as_bytes(),
        )
        .await
    {
        tracing::error!("Cannot write example config: {}", e);
        return Err(format!("Cannot write to {path:?}: {}",e.to_string()));
    }
    let _ = file.flush().await;
    drop(file);
    tracing::info!("Example config file has been generated.");
    Ok(())
}
