use super::*;
use crate::event::popup_manager::{get_timestamp, DefaultPopupProps, Popup};
use owlnest::net::p2p::protocols::blob::{OutEvent, RecvInfo, SendInfo};
use std::{fs, str::FromStr};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("owlnest-blob-transfer")
        .setup(|app| {
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = app_handle
                    .state::<swarm::Manager>()
                    .event_subscriber()
                    .subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Blob(ev)) = ev.as_ref() {
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
                                bytes_received,
                                bytes_total,
                                ..
                            } => {
                                if *bytes_received == *bytes_total {
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
                                bytes_sent,
                                bytes_total,
                                ..
                            } => {
                                if *bytes_sent == *bytes_total {
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
                            _ => {}
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
async fn list_connected(state: tauri::State<'_, swarm::Manager>) -> Result<Vec<PeerId>, String> {
    Ok(state.blob().list_connected().await)
}

#[tauri::command]
async fn send(
    state: tauri::State<'_, swarm::Manager>,
    peer: String,
    file_path: String,
) -> Result<u64, String> {
    let peer = PeerId::from_str(&peer).map_err(|e| e.to_string())?;
    fs::OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(|e| e.to_string())?;
    match state.blob().send_file(peer, file_path).await {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
async fn recv(
    state: tauri::State<'_, swarm::Manager>,
    recv_id: u64,
    file_name: String,
) -> Result<(), String> {
    let file_path = format!("C:/Users/{}/Downloads/{}", whoami::username(), file_name);
    match state.blob().recv_file(recv_id, file_path).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
async fn cancel_send(state: tauri::State<'_, swarm::Manager>, send_id: u64) -> Result<(), String> {
    state
        .blob()
        .cancel_send(send_id)
        .await
        .map_err(|_| "ID not found or not acknowledged".to_string())
}

#[tauri::command]
async fn cancel_recv(state: tauri::State<'_, swarm::Manager>, recv_id: u64) -> Result<(), String> {
    state
        .blob()
        .cancel_recv(recv_id)
        .await
        .map_err(|_| "ID not found".to_string())
}

#[tauri::command]
async fn list_pending_send(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Vec<SendInfo>, String> {
    Ok(state.blob().list_pending_send().await)
}

#[tauri::command]
async fn list_pending_recv(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Vec<RecvInfo>, String> {
    Ok(state.blob().list_pending_recv().await)
}

#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
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
        bytes_received: u64,
        bytes_total: u64,
    },
    SendProgressed {
        local_send_id: u64,
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
    CancelledSend {
        local_send_id: u64,
    },
    CancelledRecv {
        local_recv_id: u64,
    },
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
                bytes_total,
            } => Self::IncomingFile {
                from: from.clone(),
                file_name: file_name.clone(),
                local_recv_id: *local_recv_id,
                bytes_total: *bytes_total,
            },
            RecvProgressed {
                local_recv_id,
                bytes_received,
                bytes_total,
            } => Self::RecvProgressed {
                local_recv_id: *local_recv_id,
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
                bytes_sent,
                bytes_total,
            } => Self::SendProgressed {
                local_send_id: *local_send_id,
                bytes_sent: *bytes_sent,
                bytes_total: *bytes_total,
            },
            CancelledSend(local_send_id) => Self::CancelledSend {
                local_send_id: *local_send_id,
            },
            OngoingSendError {
                local_send_id,
                error,
            } => Self::OngoingSendError {
                local_send_id: *local_send_id,
                error: error.clone(),
            },
            CancelledRecv(local_recv_id) => Self::CancelledRecv {
                local_recv_id: *local_recv_id,
            },
            Error(e) => Self::Error(e.to_string()),
            _ => return Err(()),
        };
        Ok(ev)
    }
}
