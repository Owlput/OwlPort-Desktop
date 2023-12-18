use owlnest::net::p2p::swarm::behaviour::BehaviourEvent;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::RwLock;

use super::*;

#[derive(Debug, Clone, Default)]
struct State {
    node_list: Arc<RwLock<HashMap<PeerId, HashSet<Multiaddr>>>>,
}

pub fn init<R: Runtime>(manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-mdns")
        .setup(|app| {
            let state = State::default();
            let state_clone = state.clone();
            async_runtime::spawn(async move {
                let mut listener = manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Mdns(ev)) = ev.as_ref() {
                        match ev {
                            libp2p::mdns::Event::Discovered(nodes) => {
                                let mut guard = state.node_list.write().await;
                                for entry in nodes {
                                    if let Some(v) = guard.get_mut(&entry.0) {
                                        v.insert(entry.1.clone());
                                    } else {
                                        let mut set = HashSet::new();
                                        set.insert(entry.1.clone());
                                        guard.insert(entry.0, set);
                                    }
                                }
                            }
                            libp2p::mdns::Event::Expired(nodes) => {
                                let mut guard = state.node_list.write().await;
                                for entry in nodes {
                                    if let Some(v) = guard.get_mut(&entry.0) {
                                        v.remove(&entry.1);
                                        if v.len() < 1{
                                            guard.remove(&entry.0);
                                        }
                                    }
                                    
                                }
                            }
                        }
                    }
                    if let swarm::SwarmEvent::ConnectionClosed {
                        peer_id,
                        num_established,
                        ..
                    } = ev.as_ref()
                    {
                        let mut guard = state.node_list.write().await;
                        if *num_established < 1 {
                            guard.remove(peer_id);
                        }
                    }
                }
            });
            app.manage(state_clone);
            Ok(())
        })
        .invoke_handler(generate_handler![list_discovered])
        .build()
}

#[tauri::command]
async fn list_discovered(
    state: tauri::State<'_, State>,
) -> Result<HashMap<PeerId, HashSet<Multiaddr>>, String> {
    let map = state.node_list.read().await.clone();
    Ok(map)
}
