use std::str::FromStr;

use owlnest::net::p2p::{protocols::kad::OutEvent, swarm::behaviour::BehaviourEvent};

use super::*;

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-kad")
        .setup(|app| {
            app.manage(peer_manager.clone());
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Kad(ev)) = ev.as_ref() {
                        if let Ok(ev) = ev.try_into() {
                            let _ = app_handle.emit_all::<KadEmit>("owlnest-kad-emit", ev);
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
            lookup
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
    let _ = state.kad().bootstrap().await;
    Ok(())
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
    let result = state.kad().lookup(peer_id).await;
    Ok(format!("{:?}", result))
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
