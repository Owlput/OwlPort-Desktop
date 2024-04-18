use std::{num::NonZeroU32, str::FromStr};

use super::*;
use owlnest::net::p2p::protocols::messaging::*;
use tracing::{info, warn};

#[derive(Clone)]
struct State {
    pub manager: swarm::Manager,
    pub connected_peers: Arc<RwLock<HashMap<PeerId, (bool, bool)>>>,
    pub message_history_store: Arc<RwLock<HashMap<PeerId, Vec<Message>>>>,
}
impl State {
    pub async fn push_history(&self, peer: &PeerId, msg: Message) {
        let mut history = self.message_history_store.write().await;
        if let Some(v) = history.get_mut(peer) {
            v.push(msg);
        } else {
            history.insert(*peer, vec![msg]);
        }
    }
}

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    let state = State {
        manager: manager.clone(),
        connected_peers: Default::default(),
        message_history_store: Default::default(),
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
                            OutEvent::IncomingMessage { from, msg } => {
                                if let Err(e) = app_handle.emit_to::<MessagingEmit>(
                                    "Messaging",
                                    "owlnest-messaging-emit",
                                    ev.try_into().unwrap(),
                                ) {
                                    warn!("{:?}", e)
                                };
                                state.push_history(from, msg.clone()).await;
                            }
                            OutEvent::InboundNegotiated(peer_id) => {
                                let mut guard = state.connected_peers.write().await;
                                if let Some(v) = guard.get_mut(peer_id) {
                                    v.0 = true;
                                } else {
                                    guard.insert(*peer_id, (true, false));
                                }
                            }
                            OutEvent::OutboundNegotiated(peer_id) => {
                                let mut guard = state.connected_peers.write().await;
                                if let Some(v) = guard.get_mut(peer_id) {
                                    v.1 = true;
                                } else {
                                    guard.insert(*peer_id, (false, true));
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    match ev.as_ref() {
                        swarm::SwarmEvent::ConnectionEstablished {
                            peer_id,
                            num_established,
                            ..
                        } => {
                            if *num_established > NonZeroU32::new(1).expect("1 > 0") {
                                continue;
                            }
                            if let Err(e) = app_handle.emit_to(
                                "Messaging",
                                "owlnest-messaging-emit",
                                MessagingEmit::Connected { peer: *peer_id },
                            ) {
                                warn!("{:?}", e)
                            };
                            continue;
                        }
                        swarm::SwarmEvent::ConnectionClosed {
                            peer_id,
                            num_established,
                            ..
                        } => {
                            if *num_established == 0 {
                                state.connected_peers.write().await.remove(peer_id);
                            }
                            if let Err(e) = app_handle.emit_to(
                                "Messaging",
                                "owlnest-messaging-emit",
                                MessagingEmit::Disconnected { peer: *peer_id },
                            ) {
                                warn!("{:?}", e)
                            };
                        }
                        _ => {}
                    }
                }
            });
            app.manage(state_clone);
            Ok(())
        })
        .invoke_handler(generate_handler![
            send_msg,
            get_connected_peers,
            get_peer_status,
            spawn_window,
            get_chat_history,
            clear_chat_history
        ])
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
    let message = Message::new(manager.identity().get_peer_id(), peer_id, msg);
    match manager
        .messaging()
        .send_message(peer_id, message.clone())
        .await
    {
        Ok(_dur) => {
            state.push_history(&peer_id, message).await;
            Ok(())
        }
        Err(e) => Err(format!("Failed to send message to {}: {}", peer_id, e)),
    }
}

#[tauri::command]
async fn get_connected_peers(
    state: tauri::State<'_, State>,
) -> Result<HashMap<PeerId, (bool, bool)>, String> {
    let map = state.connected_peers.read().await.clone();
    Ok(map)
}

#[tauri::command]
async fn get_chat_history(
    state: tauri::State<'_, State>,
    peer_id: String,
) -> Result<Vec<Message>, String> {
    let peer_id = PeerId::from_str(&peer_id).map_err(|e| e.to_string())?;
    match state.message_history_store.read().await.get(&peer_id) {
        Some(v) => Ok(v.clone()),
        None => Err("NotFound".into()),
    }
}

#[tauri::command]
async fn clear_chat_history(
    state: tauri::State<'_, State>,
    peer_id: Option<String>,
) -> Result<(), String> {
    if peer_id.is_some() {
        let peer_id = PeerId::from_str(&peer_id.unwrap()).map_err(|e| e.to_string())?;
        if let Some(v) = state.message_history_store.write().await.get_mut(&peer_id) {
            v.clear();
            v.shrink_to_fit();
        }
    } else {
        for (_, value) in state.message_history_store.write().await.iter_mut() {
            value.clear();
            value.shrink_to_fit();
        }
    }
    Ok(())
}

#[allow(unused)]
#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, State>,
    peer: Option<PeerId>,
) -> Result<(), String> {
    if let Some(window) = app.get_window("Messaging") {
        let _ = window.set_focus();
        return Ok(());
    }
    let url = if let Some(peer) = peer {
        format!("#/app/messaging/{}", peer)
    } else {
        "#/app/messaging".into()
    };
    tauri::WindowBuilder::new(&app, "Messaging", tauri::WindowUrl::App(url.into()))
        .focused(true)
        .title("Owlnest - Messaging")
        .build()
        .expect("New window to be created successfully");

    Ok(())
}

#[tauri::command]
async fn get_peer_status(
    state: tauri::State<'_, State>,
    peer: PeerId,
) -> Result<Option<(bool, bool)>, String> {
    Ok(state.connected_peers.read().await.get(&peer).copied())
}

#[derive(Debug, Clone, Serialize)]
pub enum MessagingEmit {
    IncomingMessage { from: PeerId, msg: Message },
    InboundNegotiated { peer: PeerId },
    OutboundNegotiated { peer: PeerId },
    Disconnected { peer: PeerId },
    Connected { peer: PeerId },
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
