use crate::event::popup_manager::{get_timestamp, DefaultPopupProps, Popup};
use super::*;
use owlnest::net::p2p::protocols::blob_transfer::{OutEvent, RecvInfo, SendInfo};
use std::time::Duration;
use std::{fs, str::FromStr};

#[derive(Clone)]
struct State {
    connected_peers: Arc<RwLock<HashSet<PeerId>>>,
    manager: swarm::Manager,
}
impl State {
    fn new(manager: swarm::Manager) -> Self {
        Self {
            connected_peers: Default::default(),
            manager,
        }
    }
}

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-blob-transfer")
        .setup(|app| {
            let state = State::new(peer_manager.clone());
            app.manage(state.clone());
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::BlobTransfer(ev)) =
                        ev.as_ref()
                    {
                        if let Ok(ev) = ev.try_into() {
                            let _ = app_handle
                                .emit_all::<BlobTransferEmit>("owlnest-blob-transfer-emit", ev);
                        }
                        match ev {
                            OutEvent::IncomingFile {
                                from, file_name, ..
                            } => {
                                let _ = app_handle.emit_to(
                                    "BlobTransfer",
                                    "newPopup",
                                    Popup {
                                        timeout: 5000,
                                        stamp: format!("file-transfer-popup-{}", get_timestamp()),
                                        component: "DefaultPopup".into(),
                                        props: serde_json::to_string(&DefaultPopupProps {
                                            message: format!(
                                                "Peer {} want to send you file {}",
                                                from, file_name
                                            ),
                                            title: Some("Incoming file".into()),
                                        })
                                        .unwrap(),
                                    },
                                );
                            }
                            OutEvent::RecvProgressed {
                                local_recv_id,
                                finished,
                                ..
                            } => {
                                if *finished {
                                    let _ = app_handle.emit_to(
                                        "BlobTransfer",
                                        "newPopup",
                                        Popup {
                                            timeout: 5000,
                                            stamp: format!(
                                                "file-transfer-popup-{}",
                                                get_timestamp()
                                            ),
                                            component: "DefaultPopup".into(),
                                            props: serde_json::to_string(&DefaultPopupProps {
                                                message: format!(
                                                    "Recv ID {} completed",
                                                    local_recv_id
                                                ),
                                                title: Some("File recv completed".into()),
                                            })
                                            .unwrap(),
                                        },
                                    );
                                }
                            }
                            OutEvent::SendProgressed {
                                local_send_id,
                                finished,
                                ..
                            } => {
                                if *finished {
                                    let _ = app_handle.emit_to(
                                        "BlobTransfer",
                                        "newPopup",
                                        Popup {
                                            timeout: 5000,
                                            stamp: format!(
                                                "file-transfer-popup-{}",
                                                get_timestamp()
                                            ),
                                            component: "DefaultPopup".into(),
                                            props: serde_json::to_string(&DefaultPopupProps {
                                                message: format!(
                                                    "Send ID {} completed",
                                                    local_send_id
                                                ),
                                                title: Some("File send completed".into()),
                                            })
                                            .unwrap(),
                                        },
                                    );
                                }
                            }
                            OutEvent::CancelledSend(local_send_id) => {
                                let _ = app_handle.emit_to(
                                    "BlobTransfer",
                                    "newPopup",
                                    Popup {
                                        timeout: 5000,
                                        stamp: format!("file-transfer-popup-{}", get_timestamp()),
                                        component: "DefaultPopup".into(),
                                        props: serde_json::to_string(&DefaultPopupProps {
                                            message: format!(
                                                "Send ID {} cancelled by receiver",
                                                local_send_id
                                            ),
                                            title: Some("File send cancelled".into()),
                                        })
                                        .unwrap(),
                                    },
                                );
                            }
                            OutEvent::CancelledRecv(local_recv_id) => {
                                let _ = app_handle.emit_to(
                                    "BlobTransfer",
                                    "newPopup",
                                    Popup {
                                        timeout: 5000,
                                        stamp: format!("file-transfer-popup-{}", get_timestamp()),
                                        component: "DefaultPopup".into(),
                                        props: serde_json::to_string(&DefaultPopupProps {
                                            message: format!(
                                                "Recv ID {} cancelled by sender",
                                                local_recv_id
                                            ),
                                            title: Some("File recv cancelled".into()),
                                        })
                                        .unwrap(),
                                    },
                                );
                            }
                            OutEvent::OutboundNegotiated(peer) => {
                                state.connected_peers.write().await.insert(*peer);
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
                            state.connected_peers.write().await.remove(peer_id);
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            send,
            recv,
            cancel_send,
            cancel_recv,
            list_pending_send,
            list_pending_recv,
            list_connected,
            spawn_window
        ])
        .build()
}

#[tauri::command]
async fn list_connected(state: tauri::State<'_, State>) -> Result<Vec<PeerId>, String> {
    Ok(state.connected_peers.read().await.iter().copied().collect())
}

#[tauri::command]
async fn send(
    state: tauri::State<'_, State>,
    peer: String,
    file_path: String,
) -> Result<u64, String> {
    let peer = PeerId::from_str(&peer).map_err(|e| e.to_string())?;
    fs::OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(|e| e.to_string())?;
    match state
        .manager
        .blob_transfer()
        .send_file(peer, file_path)
        .await
    {
        Ok(v) => Ok(v.0),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
async fn recv(
    state: tauri::State<'_, State>,
    recv_id: u64,
    file_name: String,
) -> Result<(), String> {
    let file_path = format!("C:/Users/{}/Downloads/{}", whoami::username(), file_name);
    match state
        .manager
        .blob_transfer()
        .recv_file(recv_id, file_path)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
async fn cancel_send(state: tauri::State<'_, State>, send_id: u64) -> Result<(), String> {
    state
        .manager
        .blob_transfer()
        .cancel_send(send_id)
        .await
        .map_err(|_| "ID not found or not acknowledged".to_string())
}

#[tauri::command]
async fn cancel_recv(state: tauri::State<'_, State>, recv_id: u64) -> Result<(), String> {
    state
        .manager
        .blob_transfer()
        .cancel_recv(recv_id)
        .await
        .map_err(|_| "ID not found".to_string())
}

#[tauri::command]
async fn list_pending_send(state: tauri::State<'_, State>) -> Result<Vec<SendInfo>, String> {
    Ok(state.manager.blob_transfer().list_pending_send().await)
}

#[tauri::command]
async fn list_pending_recv(state: tauri::State<'_, State>) -> Result<Vec<RecvInfo>, String> {
    Ok(state.manager.blob_transfer().list_pending_recv().await)
}

#[allow(unused_variables)]
#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, State>,
    peer: Option<PeerId>,
) -> Result<(), String> {
    if let Some(window) = app.get_window("BlobTransfer") {
        let _ = window.set_focus();
        return Ok(());
    }
    let url = if let Some(peer) = peer {
        format!("#/app/blob-transfer?remote={}", peer)
    } else {
        "#/app/blob-transfer".into()
    };
    tauri::WindowBuilder::new(&app, "BlobTransfer", tauri::WindowUrl::App(url.into()))
        .focused(true)
        .title("Owlnest - File Transfer")
        .build()
        .expect("New window to be created successfully");

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub enum BlobTransferEmit {
    IncomingFile {
        from: PeerId,
        file_name: String,
        local_recv_id: u64,
        bytes_total: u64,
    },
    RecvProgressed {
        local_recv_id: u64,
        finished: bool,
        bytes_received: u64,
        bytes_total: u64,
    },
    SendProgressed {
        local_send_id: u64,
        finished: bool,
        time_taken: Option<Duration>,
        bytes_sent: u64,
        bytes_total: u64,
    },
    OngoingRecvError {
        local_recv_id: u64,
        error: String,
    },

    OngoingSendError {
        local_send_id: u64,
        error: String,
    },
    CancelledSend(u64),
    CancelledRecv(u64),
    Error(String),
}

impl TryFrom<&OutEvent> for BlobTransferEmit {
    type Error = ();
    fn try_from(value: &OutEvent) -> Result<Self, ()> {
        use OutEvent::*;
        let ev = match value {
            IncomingFile {
                from,
                file_name,
                local_recv_id,
                bytes_total
            } => Self::IncomingFile {
                from: from.clone(),
                file_name: file_name.clone(),
                local_recv_id: *local_recv_id,
                bytes_total: *bytes_total
            },
            RecvProgressed {
                local_recv_id,
                finished,
                bytes_received,
                bytes_total,
            } => Self::RecvProgressed {
                local_recv_id: *local_recv_id,
                finished: *finished,
                bytes_received: *bytes_received,
                bytes_total: *bytes_total,
            },
            OngoingRecvError {
                local_recv_id,
                error,
            } => Self::OngoingRecvError {
                local_recv_id: *local_recv_id,
                error: error.clone(),
            },
            SendProgressed {
                local_send_id,
                finished,
                time_taken,
                bytes_sent,
                bytes_total,
            } => Self::SendProgressed {
                local_send_id: *local_send_id,
                finished: *finished,
                time_taken: *time_taken,
                bytes_sent: *bytes_sent,
                bytes_total: *bytes_total
            },
            CancelledSend(id) => Self::CancelledSend(*id),
            OngoingSendError {
                local_send_id,
                error,
            } => Self::OngoingSendError {
                local_send_id: *local_send_id,
                error: error.clone(),
            },
            CancelledRecv(id) => Self::CancelledRecv(*id),
            Error(e) => Self::Error(e.to_string()),
            _ => return Err(()),
        };
        Ok(ev)
    }
}
