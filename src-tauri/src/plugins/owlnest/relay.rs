use libp2p::StreamProtocol;

use super::*;
use std::collections::HashMap;

#[derive(Clone)]
struct State {
    connected: Arc<RwLock<HashMap<PeerId, Relay>>>,
    manager: swarm::Manager,
}
impl State {
    pub fn new(manager: swarm::Manager) -> Self {
        Self {
            connected: Default::default(),
            manager,
        }
    }
}

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-relay")
        .setup(|app| {
            let state = State::new(peer_manager.clone());
            app.manage(state.clone());
            let _app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Identify(ev)) = ev.as_ref()
                    {
                        match ev {
                            identify::OutEvent::Received { peer_id, info } => {
                                if info.protocols.contains(&StreamProtocol::new(
                                    "/libp2p/circuit/relay/0.2.0/hop",
                                )) && info.protocols.contains(&StreamProtocol::new(
                                    "/libp2p/circuit/relay/0.2.0/stop",
                                )) {
                                    state.connected.write().expect("Not poisoned").insert(
                                        *peer_id,
                                        Relay {
                                            address: info.listen_addrs.clone(),
                                            supports_ext: info.protocols.contains(
                                                &StreamProtocol::new("/owlnest/relay_ext/0.0.1"),
                                            ),
                                        },
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    if let swarm::SwarmEvent::ConnectionClosed {
                        peer_id,
                        num_established,
                        ..
                    } = ev.as_ref()
                    {
                        if *num_established < 1 {
                            state
                                .connected
                                .write()
                                .expect("Not poisoned")
                                .remove(peer_id);
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            list_relays,
            query_advertised,
            set_remote_advertisement,
            set_local_provider_state
        ])
        .build()
}

#[tauri::command]
async fn list_relays(state: tauri::State<'_, State>) -> Result<HashMap<PeerId, Relay>, String> {
    Ok(state.connected.read().expect("Not poisoned").clone())
}

#[tauri::command]
async fn query_advertised(
    state: tauri::State<'_, State>,
    relay: PeerId,
) -> Result<Vec<PeerId>, String> {
    state
        .manager
        .relay_ext()
        .query_advertised_peer(relay)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_remote_advertisement(
    state: tauri::State<'_, State>,
    relay: PeerId,
    advertisement_state: bool,
) -> Result<(), String> {
    Ok(state
        .manager
        .relay_ext()
        .set_remote_advertisement(relay, advertisement_state)
        .await)
}

#[tauri::command]
async fn set_local_provider_state(
    state: tauri::State<'_, State>,
    provider_state: bool,
) -> Result<(), String> {
    state
        .manager
        .relay_ext()
        .set_provider_state(provider_state)
        .await;
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
struct Relay {
    pub address: Vec<Multiaddr>,
    pub supports_ext: bool,
}
