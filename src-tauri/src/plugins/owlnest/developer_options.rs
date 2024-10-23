use super::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("owlnest-developer-options")
        .invoke_handler(generate_handler![swarm_event_listener])
        .build()
}

#[tauri::command]
async fn swarm_event_listener<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    if let Some(window) = app.get_window("SwarmEventListener") {
        let _ = window.set_focus();
        return Ok(());
    }
    tauri::WindowBuilder::new(
        &app,
        "BlobTransfer",
        tauri::WindowUrl::App("#/dev/swarm-event-listener".into()),
    )
    .focused(true)
    .title("Developer Tool - Swarm Event Listener")
    .build()
    .expect("New window to be created successfully");
    Ok(())
}
