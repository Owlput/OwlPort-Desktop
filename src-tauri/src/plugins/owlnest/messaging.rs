use super::*;
use owlnest::net::p2p::protocols::messaging::*;
use std::num::NonZeroU32;
use tauri::{Emitter, EventTarget};
use tracing::{info, warn};

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-messaging")
        .setup(move |app, _api| {
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Messaging(ev)) = ev.as_ref()
                    {
                        match ev {
                            OutEvent::IncomingMessage { .. } => {
                                if let Err(e) = app_handle.emit_to::<EventTarget, MessagingEmit>(
                                    EventTarget::labeled("owlnest-messaging"),
                                    "owlnest-messaging-emit",
                                    ev.try_into().unwrap(),
                                ) {
                                    warn!("{:?}", e)
                                };
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
                                "owlnest-messaging",
                                "owlnest-messaging-emit",
                                MessagingEmit::Connected { peer: *peer_id },
                            ) {
                                warn!("{:?}", e)
                            };
                            continue;
                        }
                        swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            if let Err(e) = app_handle.emit_to(
                                "owlnest-messaging",
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
    manager: tauri::State<'_, swarm::Manager>,
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
    let message = Message::new(manager.identity().get_peer_id(), peer_id, msg);
    match manager
        .messaging()
        .send_message(peer_id, message.clone())
        .await
    {
        Ok(_dur) => {
            manager
                .messaging()
                .message_store()
                .push_message(&peer_id, message);
            Ok(())
        }
        Err(e) => Err(format!("Failed to send message to {}: {}", peer_id, e)),
    }
}

#[tauri::command]
async fn get_chat_history(
    manager: tauri::State<'_, swarm::Manager>,
    peer_id: PeerId,
) -> Result<Box<[Message]>, String> {
    match manager.messaging().message_store().get_messages(&peer_id) {
        Some(v) => Ok(v),
        None => Err("NotFound".into()),
    }
}

#[tauri::command]
async fn get_all_chats(manager: tauri::State<'_, swarm::Manager>) -> Result<Box<[PeerId]>, String> {
    Ok(manager.messaging().message_store().list_all_peers())
}

#[tauri::command]
async fn clear_chat_history(
    manager: tauri::State<'_, swarm::Manager>,
    peer_id: Option<PeerId>,
) -> Result<(), String> {
    manager
        .messaging()
        .message_store()
        .clear_message(peer_id.as_ref());
    Ok(())
}

#[tauri::command]
async fn list_connected(manager: tauri::State<'_, swarm::Manager>) -> Result<Box<[PeerId]>, ()> {
    Ok(manager.messaging().list_connected().await)
}

#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    manager: tauri::State<'_, swarm::Manager>,
    peer: Option<PeerId>,
) -> Result<(), String> {
    if let Some(peer) = peer {
        let store = manager.messaging().message_store();
        if let None = store.get_messages(&peer) {
            store.insert_empty_record(&peer);
        }
    }
    if let Some(window) = app.get_webview_window("owlnest-messaging") {
        let _ = window.emit("focusChat", peer);
        let _ = window.set_focus();
        return Ok(());
    }
    let url = if let Some(peer) = peer {
        format!("#/app/messaging/{}", peer)
    } else {
        "#/app/messaging".into()
    };
    tauri::WebviewWindowBuilder::new(&app, "owlnest-messaging", tauri::WebviewUrl::App(url.into()))
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
