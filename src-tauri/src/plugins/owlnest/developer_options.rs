use owlnest::net::p2p::protocols::gossipsub::HashType;

use super::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("owlnest-developer-options")
        .invoke_handler(generate_handler![swarm_event_listener, print_struct])
        .build()
}

#[tauri::command]
async fn swarm_event_listener<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("SwarmEventListener") {
        let _ = window.set_focus();
        return Ok(());
    }
    tauri::WebviewWindowBuilder::new(
        &app,
        "BlobTransfer",
        tauri::WebviewUrl::App("#/dev/swarm-event-listener".into()),
    )
    .focused(true)
    .title("Developer Tool - Swarm Event Listener")
    .build()
    .expect("New window to be created successfully");
    Ok(())
}

#[tauri::command]
async fn print_struct(
    _state: tauri::State<'_, swarm::Manager>,
) -> Result<HashType, String> {
    Ok(HashType::Identity)
}