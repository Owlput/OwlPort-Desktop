use super::*;
use dashmap::DashMap;
use owlnest::net::p2p::protocols::messaging::*;
use std::num::NonZeroU32;
use tracing::{info, warn};

#[derive(Clone)]
struct State {
    pub manager: swarm::Manager,
    pub message_store: Arc<DashMap<PeerId, Vec<Message>>>,
}
impl State {
    pub fn push_history(&self, peer: &PeerId, msg: Message) {
        if let Some(mut v) = self.message_store.get_mut(peer) {
            v.value_mut().push(msg);
        } else {
            self.message_store.insert(*peer, vec![msg]);
        }
    }
    pub fn clear_chat_history(&self, peer_id: Option<PeerId>) {
        if let Some(peer) = peer_id {
            if let Some(mut entry) = self.message_store.get_mut(&peer) {
                entry.value_mut().clear();
                entry.value_mut().shrink_to_fit();
            }
        } else {
            for mut entry in self.message_store.iter_mut() {
                entry.value_mut().clear();
                entry.value_mut().shrink_to_fit();
            }
        }
    }
}

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    let state = State {
        manager: manager.clone(),
        message_store: Default::default(),
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
                                state.push_history(from, msg.clone());
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
                        swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
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
            spawn_window,
            get_chat_history,
            clear_chat_history,
            list_connected,
            get_all_chats,
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
            state.push_history(&peer_id, message);
            Ok(())
        }
        Err(e) => Err(format!("Failed to send message to {}: {}", peer_id, e)),
    }
}

#[tauri::command]
async fn get_chat_history(
    state: tauri::State<'_, State>,
    peer_id: PeerId,
) -> Result<Vec<Message>, String> {
    match state.message_store.get(&peer_id) {
        Some(v) => Ok(v.value().clone()),
        None => Err("NotFound".into()),
    }
}

#[tauri::command]
async fn get_all_chats(state: tauri::State<'_, State>) -> Result<Vec<PeerId>, String> {
    Ok(state.message_store.iter().map(|kv| *kv.key()).collect())
}

#[tauri::command]
async fn clear_chat_history(
    state: tauri::State<'_, State>,
    peer_id: Option<PeerId>,
) -> Result<(), String> {
    state.clear_chat_history(peer_id);
    Ok(())
}

#[tauri::command]
async fn list_connected(state: tauri::State<'_, State>) -> Result<Box<[PeerId]>, ()> {
    Ok(state.manager.messaging().list_connected().await)
}

#[allow(unused)]
#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, State>,
    peer: Option<PeerId>,
) -> Result<(), String> {
    if let Some(peer) = peer {
        let store = &state.message_store;
        if let None = store.get(&peer) {
            store.insert(peer, Vec::new());
        }
    }
    if let Some(window) = app.get_window("Messaging") {
        window.emit("focusChat", peer);
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
                from: *from,
                msg: msg.clone(),
            },
            OutboundNegotiated(peer) => Self::OutboundNegotiated { peer: *peer },
            InboundNegotiated(peer) => Self::InboundNegotiated { peer: *peer },
            _ => return Err(()),
        };

        Ok(ev)
    }
}
