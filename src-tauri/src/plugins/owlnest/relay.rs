use std::cmp::Ordering;

use super::*;
use libp2p::StreamProtocol;

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
                static RELAY_HOP: StreamProtocol =
                    StreamProtocol::new("/libp2p/circuit/relay/0.2.0/hop");
                static RELAY_STOP: StreamProtocol =
                    StreamProtocol::new("/libp2p/circuit/relay/0.2.0/stop");
                static advertise: StreamProtocol = StreamProtocol::new("/owlnest/advertise/0.0.1");
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Identify(ev)) = ev.as_ref()
                    {
                        match ev {
                            identify::OutEvent::Received { peer_id, info } => {
                                if info.protocols.contains(&RELAY_HOP)
                                    && info.protocols.contains(&RELAY_STOP)
                                {
                                    let mut connected_list = state.connected.write().await;
                                    if let None = connected_list.get(peer_id) {
                                        connected_list.insert(
                                            *peer_id,
                                            Relay {
                                                address: HashSet::from_iter(
                                                    info.listen_addrs.iter().cloned(),
                                                ),
                                                listened_address: HashSet::new(),
                                                supports_ext: info.protocols.contains(&advertise),
                                                latency: -1,
                                            },
                                        );
                                        continue;
                                    };

                                    let entry_to_update = connected_list.get_mut(peer_id).unwrap();
                                    entry_to_update.listened_address.retain(|v| {
                                        info.listen_addrs
                                            .iter()
                                            .filter(|new_addr| {
                                                v.to_string().contains(&new_addr.to_string())
                                            })
                                            .next()
                                            .is_some()
                                    });
                                    entry_to_update.address = HashSet::from_iter(
                                        info.listen_addrs
                                            .iter()
                                            .filter(|v| {
                                                !entry_to_update.listened_address.contains(*v)
                                            })
                                            .cloned(),
                                    );
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    match ev.as_ref() {
                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                            let mut relay_list = state.connected.write().await;
                            let address_string = address.to_string();
                            if let Some((_, v)) = relay_list
                                .iter_mut()
                                .filter(|(k, v)| {
                                    // get the one that has the address that is part of the newly listened address
                                    v.address
                                        .iter()
                                        .filter(|_| {
                                            address_string.contains(&format!("{}/p2p-circuit", k))
                                        })
                                        .next()
                                        .is_some()
                                })
                                .next()
                            {
                                v.address
                                    .retain(|v| !address_string.contains(&v.to_string()));
                                v.listened_address.insert(address.clone());
                            };
                            continue;
                        }
                        libp2p::swarm::SwarmEvent::ListenerClosed { addresses, .. } => {
                            let mut relay_list = state.connected.write().await;
                            for address in addresses {
                                if let Some(v) = relay_list
                                    .values_mut()
                                    .filter(|v| {
                                        // get the one that has the address that is part of the newly closed address
                                        v.listened_address
                                            .iter()
                                            .filter(|v| **v == *address)
                                            .next()
                                            .is_some()
                                    })
                                    .next()
                                {
                                    v.listened_address.remove(address);
                                };
                            }
                        }
                        _ => {}
                    }
                    if let swarm::SwarmEvent::ConnectionClosed {
                        peer_id,
                        num_established,
                        ..
                    } = ev.as_ref()
                    {
                        if *num_established < 1 {
                            state.connected.write().await.remove(peer_id);
                        }
                        continue;
                    }
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Ping(ev)) = ev.as_ref() {
                        if let Some(v) = state.connected.write().await.get_mut(&ev.peer) {
                            if let Err(_) = ev.result {
                                v.latency = -1;
                            } else {
                                v.latency = ev
                                    .result
                                    .as_ref()
                                    .unwrap()
                                    .as_millis()
                                    .try_into()
                                    .unwrap_or(isize::MAX);
                            }
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
            set_local_provider_state,
            get_relay_status
        ])
        .build()
}

#[tauri::command]
async fn list_relays(state: tauri::State<'_, State>) -> Result<Vec<(PeerId, isize)>, String> {
    let relays = state.connected.read().await;
    let mut list = relays
        .iter()
        .map(|(k, v)| (*k, v.latency))
        .collect::<Vec<(PeerId, isize)>>();
    drop(relays);
    list.sort_unstable_by(|a, b| {
        if a.1 >= 0 {
            a.1.cmp(&b.1)
        } else {
            Ordering::Greater
        }
    });
    Ok(list)
}

#[tauri::command]
async fn get_relay_status(
    state: tauri::State<'_, State>,
    relay: PeerId,
) -> Result<Option<Relay>, String> {
    Ok(state.connected.read().await.get(&relay).cloned())
}

#[tauri::command]
async fn query_advertised(
    state: tauri::State<'_, State>,
    relay: PeerId,
) -> Result<Vec<PeerId>, String> {
    state
        .manager
        .advertise()
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
        .advertise()
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
        .advertise()
        .set_provider_state(provider_state)
        .await;
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
struct Relay {
    pub address: HashSet<Multiaddr>,
    pub listened_address: HashSet<Multiaddr>,
    pub supports_ext: bool,
    pub latency: isize,
}
