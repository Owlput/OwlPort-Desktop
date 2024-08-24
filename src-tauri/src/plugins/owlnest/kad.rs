use super::*;
use libp2p::kad::Mode;
use owlnest::net::p2p::protocols::kad::OutEvent;
use std::{str::FromStr, sync::atomic::AtomicBool};

#[derive(Debug, Clone)]
struct State {
    /// Current Kad mode. `true` for server mode(active sharing),
    /// `false` for client mode(listen only)
    mode: Arc<AtomicBool>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            mode: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-kad")
        .setup(|app| {
            let app_handle = app.clone();
            let state = State::default();
            app.manage(state.clone());
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Kad(ev)) = ev.as_ref() {
                        if let Ok(ev) = ev.try_into() {
                            let _ = app_handle.emit_all::<KadEmit>("owlnest-kad-emit", ev);
                        }
                        match ev {
                            libp2p::kad::Event::InboundRequest { .. } => {}
                            libp2p::kad::Event::OutboundQueryProgressed { .. } => {}
                            libp2p::kad::Event::RoutingUpdated { .. } => {}
                            libp2p::kad::Event::UnroutablePeer { .. } => {}
                            libp2p::kad::Event::RoutablePeer { .. } => {}
                            libp2p::kad::Event::PendingRoutablePeer { .. } => {}
                            libp2p::kad::Event::ModeChanged { new_mode } => match new_mode {
                                Mode::Client => {
                                    state.mode.store(false, std::sync::atomic::Ordering::SeqCst)
                                }
                                Mode::Server => {
                                    state.mode.store(true, std::sync::atomic::Ordering::SeqCst)
                                }
                            },
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            insert_default,
            bootstrap,
            insert_node,
            lookup,
            get_mode,
            set_mode,
            get_all_records
        ])
        .build()
}

#[tauri::command]
async fn insert_default(state: tauri::State<'_, swarm::Manager>) -> Result<(), String> {
    state
        .kad()
        .insert_node(
            PeerId::from_str("QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN").unwrap(),
            "/dnsaddr/bootstrap.libp2p.io".parse::<Multiaddr>().unwrap(),
        )
        .await;
    state
        .kad()
        .insert_node(
            PeerId::from_str("QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa").unwrap(),
            "/dnsaddr/bootstrap.libp2p.io".parse::<Multiaddr>().unwrap(),
        )
        .await;
    state
        .kad()
        .insert_node(
            PeerId::from_str("QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb").unwrap(),
            "/dnsaddr/bootstrap.libp2p.io".parse::<Multiaddr>().unwrap(),
        )
        .await;
    state
        .kad()
        .insert_node(
            PeerId::from_str("QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt").unwrap(),
            "/dnsaddr/bootstrap.libp2p.io".parse::<Multiaddr>().unwrap(),
        )
        .await;
    state
        .kad()
        .insert_node(
            PeerId::from_str("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ").unwrap(),
            "/ip4/104.131.131.82/tcp/4001".parse::<Multiaddr>().unwrap(),
        )
        .await;
    Ok(())
}

#[tauri::command]
async fn bootstrap(state: tauri::State<'_, swarm::Manager>) -> Result<(), String> {
    match state.kad().bootstrap().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
async fn insert_node(
    state: tauri::State<'_, swarm::Manager>,
    peer_id: String,
    address: String,
) -> Result<String, String> {
    let peer_id = PeerId::from_str(&peer_id).map_err(|e| e.to_string())?;
    let address = Multiaddr::from_str(&address).map_err(|e| e.to_string())?;
    let routing_update = state.kad().insert_node(peer_id, address).await;
    Ok(format!("{:?}", routing_update))
}

#[tauri::command]
async fn lookup(
    state: tauri::State<'_, swarm::Manager>,
    peer_id: String,
) -> Result<String, String> {
    let peer_id = PeerId::from_str(&peer_id).map_err(|e| e.to_string())?;
    let result = state.kad().lookup(&peer_id).await;
    Ok(format!("{:?}", result))
}

#[tauri::command]
async fn get_mode(state: tauri::State<'_, State>) -> Result<bool, ()> {
    Ok(state.mode.load(std::sync::atomic::Ordering::Relaxed))
}

#[tauri::command]
async fn set_mode(state: tauri::State<'_, swarm::Manager>, mode: bool) -> Result<(), ()> {
    let _ = if mode {
        state.kad().set_mode(Some(Mode::Server)).await
    } else {
        state.kad().set_mode(Some(Mode::Client)).await
    };
    Ok(())
}

#[tauri::command]
async fn get_all_records(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Vec<(PeerId, Vec<Multiaddr>)>, ()> {
    Ok(state
        .kad()
        .all_records()
        .await
        .into_iter()
        .map(|(peer, addrs)| (peer, addrs.into_vec()))
        .collect::<Vec<(PeerId, Vec<Multiaddr>)>>())
}

#[derive(Debug, Clone, Serialize)]
enum KadEmit {
    RoutingUpdated {
        peer: PeerId,
        is_new_peer: bool,
        addresses: Vec<String>,
    },
}
impl TryFrom<&OutEvent> for KadEmit {
    type Error = ();
    fn try_from(value: &OutEvent) -> Result<Self, Self::Error> {
        let ev = match value {
            OutEvent::RoutingUpdated {
                peer,
                is_new_peer,
                addresses,
                ..
            } => Self::RoutingUpdated {
                peer: *peer,
                is_new_peer: *is_new_peer,
                addresses: addresses.iter().map(|addr| addr.to_string()).collect(),
            },
            _ => return Err(()),
        };
        Ok(ev)
    }
}
