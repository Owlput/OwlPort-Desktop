use super::*;
use owlnest::net::p2p::protocols::messaging::OutEvent;
use owlnest::net::p2p::{
    protocols::messaging::Message,
    swarm::{self, behaviour::BehaviourEvent},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Clone)]
struct State {
    pub manager: swarm::Manager,
    pub connected_peers: Arc<RwLock<HashMap<PeerId, (bool, bool)>>>,
}

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    let state = State {
        manager: manager.clone(),
        connected_peers: Arc::new(RwLock::new(HashMap::new())),
    };
    Builder::new("owlnest-messaging")
        .setup(move |app| {
            let app_handle = app.clone();
            let state_clone = state.clone();
            async_runtime::spawn(async move {
                let mut listener = manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Messaging(ev)) = ev.as_ref()
                    {
                        match ev {
                            OutEvent::IncomingMessage { .. } => {
                                if let Err(e) = app_handle.emit_all::<MessagingEmit>(
                                    "owlnest-messaging-emit",
                                    ev.try_into().unwrap(),
                                ) {
                                    warn!("{:?}", e)
                                };
                            }
                            OutEvent::InboundNegotiated(peer_id) => {
                                let mut guard = state.connected_peers.write().await;
                                if let Some(v) = guard.get_mut(peer_id) {
                                    v.0 = true;
                                } else {
                                    guard.insert(*peer_id, (true, false));

                                    if let Err(e) = app_handle.emit_all::<MessagingEmit>(
                                        "owlnest-messaging-emit",
                                        ev.try_into().unwrap(),
                                    ) {
                                        warn!("{:?}", e)
                                    };
                                }

                                drop(guard)
                            }
                            OutEvent::OutboundNegotiated(peer_id) => {
                                let mut guard = state.connected_peers.write().await;
                                if let Some(v) = guard.get_mut(peer_id) {
                                    v.1 = true;
                                } else {
                                    guard.insert(*peer_id, (false, true));
                                }
                                if let Err(e) = app_handle.emit_all::<MessagingEmit>(
                                    "owlnest-messaging-emit",
                                    ev.try_into().unwrap(),
                                ) {
                                    warn!("{:?}", e)
                                };

                                drop(guard)
                            }
                            _ => {}
                        }
                        continue;
                    }
                    if let swarm::SwarmEvent::ConnectionClosed {
                        peer_id,
                        num_established,
                        ..
                    } = ev.as_ref()
                    {
                        if *num_established == 0 {
                            state.connected_peers.write().await.remove(peer_id);
                        }
                    }
                }
            });
            app.manage(state_clone);
            Ok(())
        })
        .invoke_handler(generate_handler![send_msg, setup])
        .build()
}

#[tauri::command]
async fn send_msg(
    state: tauri::State<'_, State>,
    target: String,
    msg: String,
) -> Result<(), String> {
    let peer_id = match target.parse::<PeerId>() {
        Ok(v) => v,
        Err(e) => {
            info!("Failed to parse peer ID for input {}: {}", target, e);
            return Err("Invalid PeerId".into());
        }
    };
    let manager = &state.manager;
    match manager
        .messaging()
        .send_message(
            peer_id,
            Message::new(manager.identity().get_peer_id(), peer_id, msg),
        )
        .await
    {
        Ok(_dur) => Ok(()),
        Err(e) => Err(format!("Failed to send message to {}: {}", peer_id, e)),
    }
}

#[tauri::command]
async fn setup(state: tauri::State<'_, State>) -> Result<HashMap<PeerId,(bool,bool)>, String> {
    let map = state.connected_peers.read().await.clone();
    Ok(map)
}

#[derive(Debug, Clone, Serialize)]
pub enum MessagingEmit {
    IncomingMessage { from: PeerId, msg: Message },
    InboundNegotiated { peer: PeerId },
    OutboundNegotiated { peer: PeerId },
}
impl TryFrom<&OutEvent> for MessagingEmit {
    type Error = ();
    fn try_from(value: &OutEvent) -> Result<Self, Self::Error> {
        use OutEvent::*;
        let ev = match value {
            IncomingMessage { from, msg } => Self::IncomingMessage {
                from: from.clone(),
                msg: msg.clone(),
            },
            OutboundNegotiated(peer) => Self::OutboundNegotiated { peer: *peer },
            InboundNegotiated(peer) => Self::InboundNegotiated { peer: *peer },
            _ => return Err(()),
        };

        Ok(ev)
    }
}
