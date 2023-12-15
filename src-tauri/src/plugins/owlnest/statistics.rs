use super::*;

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    Builder::new("swarm")
        .setup(|app| {
            let app_handle = app.clone();
            let mut listener = manager.event_subscriber().subscribe();
            async_runtime::spawn(async move {
                loop {
                    if let Ok(ev) = listener.recv().await {
                        if let Ok(ev) = TryInto::<SwarmEventEmit>::try_into(ev.as_ref()) {
                            let _ = app_handle.emit_all("swarm-event", ev);
                        }
                    }
                }
            });
            app.manage(State {
                swarm_manager: manager,
            });
            Ok(())
        })
        .invoke_handler(generate_handler![dial])
        .build()
}

pub enum StatisticsEmit{

}