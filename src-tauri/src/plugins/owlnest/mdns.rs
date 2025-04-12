use super::*;
use dashmap::DashMap;
use owlnest::net::p2p::swarm::behaviour::BehaviourEvent;

#[derive(Debug, Clone, Default)]
struct State {
    node_list: Arc<DashMap<PeerId, HashSet<Multiaddr>>>,
}
impl State {
    pub(crate) fn on_discovered(&self, new_nodes: &Vec<(PeerId, Multiaddr)>) {
        for node in new_nodes {
            if let Some(mut entry) = self.node_list.get_mut(&node.0) {
                entry.value_mut().insert(node.1.clone());
            } else {
                let mut set = HashSet::new();
                set.insert(node.1.clone());
                self.node_list.insert(node.0, set);
            }
        }
    }
    pub(crate) fn on_expired(&self, expired_nodes: &Vec<(PeerId, Multiaddr)>) {
        for node in expired_nodes {
            if let Some(mut entry) = self.node_list.get_mut(&node.0) {
                entry.value_mut().remove(&node.1);
                if entry.value().is_empty() {
                    self.node_list.remove(&node.0);
                }
            }
        }
    }
}

pub fn init<R: Runtime>(manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-mdns")
        .setup(|app, _api| {
            let state = State::default();
            let state_clone = state.clone();
            async_runtime::spawn(async move {
                let mut listener = manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Mdns(ev)) = ev.as_ref() {
                        match ev {
                            libp2p::mdns::Event::Discovered(new_nodes) => {
                                state.on_discovered(new_nodes)
                            }
                            libp2p::mdns::Event::Expired(expired_nodes) => {
                                state.on_expired(expired_nodes)
                            }
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
    let mut map = HashMap::new();
    state
        .node_list
        .iter()
        .map(|entry| map.insert(*entry.key(), entry.value().clone()))
        .count();
    Ok(map)
}
