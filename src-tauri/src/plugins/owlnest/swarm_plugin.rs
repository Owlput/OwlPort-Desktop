use super::*;
use libp2p::identify::Info;
use owlnest::net::p2p::swarm::{self, behaviour::BehaviourEvent};
use std::num::NonZeroU32;
use tracing::{error, warn};

#[derive(Clone)]
struct State {
    pub swarm_manager: swarm::manager::Manager,
    pub connected_peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
}

#[derive(Debug, Clone, Serialize)]
struct PeerInfo {
    supported_protocols: Vec<String>,
    protocol_version: String,
}
impl Default for PeerInfo {
    fn default() -> Self {
        Self {
            supported_protocols: Default::default(),
            protocol_version: "UNKNOWN".into(),
        }
    }
}
impl From<&Info> for PeerInfo {
    fn from(value: &Info) -> Self {
        PeerInfo {
            supported_protocols: value
                .protocols
                .iter()
                .map(|protocol| protocol.to_string())
                .collect(),
            protocol_version: value.protocol_version.clone(),
        }
    }
}

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-swarm")
        .setup(|app| {
            let app_handle = app.clone();
            let mut listener = manager.event_subscriber().subscribe();
            let state = State {
                swarm_manager: manager,
                connected_peers: Arc::new(RwLock::new(HashMap::new())),
            };
            let state_clone = state.clone();
            async_runtime::spawn(async move {
                while let Ok(ev) = listener.recv().await {
                    if let Ok(ev) = TryInto::<SwarmEventEmit>::try_into(ev.as_ref()) {
                        app_handle
                            .emit_all("swarm-emit", ev)
                            .expect("event emit to succeed");
                    }
                    use libp2p::swarm::SwarmEvent::*;
                    let mut guard = state.connected_peers.write().await;
                    match ev.as_ref() {
                        ConnectionEstablished {
                            peer_id,
                            num_established,
                            ..
                        } => {
                            if *num_established < NonZeroU32::new(2).unwrap() {
                                guard.insert(*peer_id, Default::default());
                            }
                        }
                        ConnectionClosed {
                            peer_id,
                            num_established,
                            ..
                        } => {
                            if *num_established < 1 {
                                guard.remove(peer_id);
                            }
                        }
                        IncomingConnection { .. } => {}
                        IncomingConnectionError { .. } => {}
                        OutgoingConnectionError { .. } => {}
                        NewListenAddr { .. } => {}
                        ExpiredListenAddr { .. } => {}
                        ListenerClosed { .. } => {}
                        ListenerError { .. } => {}
                        Dialing { .. } => {}
                        NewExternalAddrCandidate { .. } => {}
                        ExternalAddrConfirmed { .. } => {}
                        ExternalAddrExpired { .. } => {}
                        Behaviour(ev) => {
                            if let BehaviourEvent::Identify(identify::OutEvent::Received {
                                peer_id,
                                info,
                            }) = ev
                            {
                                if let Some(v) = guard.get_mut(peer_id) {
                                    *v = info.into()
                                } else {
                                    error!("Behaviour event handled before ConnectionEstablished")
                                }
                            }
                        }
                        NewExternalAddrOfPeer { .. } => {}
                        _ => warn!("New branch for SwarmEvent not covered"),
                    }
                    drop(guard)
                }
                error!("Swarm sender Dropped! Internal state corrupted!");
            });
            app.manage(state_clone);
            Ok(())
        })
        .invoke_handler(generate_handler![
            dial,
            listen,
            get_local_peer_id,
            list_listeners,
            list_connected,
            get_peer_info,
            disconnect_peer
        ])
        .build()
}

#[tauri::command]
async fn dial(state: tauri::State<'_, State>, dial_options: DialOptions) -> Result<(), String> {
    state
        .swarm_manager
        .swarm()
        .dial(&dial_options.address)
        .await
        .map_err(|e| format!("{}", e))
}

#[derive(Debug, Deserialize)]
pub struct DialOptions {
    pub address: Multiaddr,
}

#[tauri::command]
async fn listen(
    state: tauri::State<'_, State>,
    listen_options: ListenOptions,
) -> Result<(), String> {
    let addr = listen_options
        .addr
        .parse()
        .map_err(|e| format!("{:?}", e))?;
    state
        .swarm_manager
        .swarm()
        .listen(&addr)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn get_local_peer_id(state: tauri::State<'_, State>) -> Result<String, String> {
    Ok(state.swarm_manager.identity().get_peer_id().to_string())
}

#[tauri::command]
async fn list_listeners(state: tauri::State<'_, State>) -> Result<Vec<Multiaddr>, String> {
    Ok(state.swarm_manager.swarm().list_listeners().await)
}

#[tauri::command]
async fn list_connected(state: tauri::State<'_, State>) -> Result<Vec<PeerId>, String> {
    Ok(state.connected_peers.read().await.keys().cloned().collect())
}
#[tauri::command]
async fn get_peer_info(
    state: tauri::State<'_, State>,
    peer_id: PeerId,
) -> Result<Option<PeerInfo>, String> {
    Ok(state.connected_peers.read().await.get(&peer_id).cloned())
}

#[tauri::command]
async fn disconnect_peer(state: tauri::State<'_, State>, peer_id: PeerId) -> Result<(), String> {
    state
        .swarm_manager
        .swarm()
        .disconnect_peer_id(peer_id)
        .await
        .map_err(|_| "Cannot disconnect".into())
}

#[derive(Debug, Deserialize)]
pub struct ListenOptions {
    pub addr: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum SwarmEventEmit {
    ConnectionEstablished {
        /// A connection to the given peer has been opened.
        peer_id: PeerId,
        /// Endpoint of the connection that has been opened.
        endpoint: serde_types::ConnectedPoint,
        /// How long it took to establish this connection
        established_in: u64,
    },
    ConnectionClosed {
        /// Identity of the peer that we have connected to.
        peer_id: PeerId,
        /// Endpoint of the connection that has been closed.
        endpoint: serde_types::ConnectedPoint,
        /// Reason for the disconnection, if it was not a successful
        /// active close.
        cause: String,
        num_established: u64,
    },
    Dialing {
        /// Identity of the peer that we are connecting to.
        maybe_peer_id: Option<PeerId>,
    },
    IncomingConnection {
        /// Local connection address.
        /// This address has been earlier reported with a [`NewListenAddr`](SwarmEvent::NewListenAddr)
        /// event.
        local_addr: Multiaddr,
        /// Address used to send back data to the remote.
        send_back_addr: Multiaddr,
    },
    NewListenAddr {
        /// The new address that is being listened on.
        address: Multiaddr,
    },
    ExpiredListenAddr {
        /// The expired address.
        address: Multiaddr,
    },
    OutgoingConnectionError {
        error: serde_types::DialError,
    },
}
impl TryFrom<&swarm::SwarmEvent> for SwarmEventEmit {
    type Error = ();
    fn try_from(value: &swarm::SwarmEvent) -> Result<Self, Self::Error> {
        use swarm::SwarmEvent;
        let ev = match value {
            SwarmEvent::Dialing { peer_id, .. } => Self::Dialing {
                maybe_peer_id: peer_id.clone(),
            },
            SwarmEvent::ConnectionEstablished {
                peer_id,
                endpoint,
                established_in,
                ..
            } => Self::ConnectionEstablished {
                peer_id: *peer_id,
                endpoint: endpoint.into(),
                established_in: established_in.as_millis().try_into().unwrap_or(u64::MAX),
            },
            SwarmEvent::ConnectionClosed {
                peer_id,
                cause,
                endpoint,
                num_established,..
            } => Self::ConnectionClosed {
                peer_id: *peer_id,
                endpoint: endpoint.into(),
                num_established: *num_established as u64,
                cause: format!("{:?}", cause),
            },
            SwarmEvent::IncomingConnection {
                local_addr,
                send_back_addr,
                ..
            } => Self::IncomingConnection {
                local_addr: local_addr.clone(),
                send_back_addr: send_back_addr.clone(),
            },
            SwarmEvent::NewListenAddr { address, .. } => Self::NewListenAddr {
                address: address.clone(),
            },
            SwarmEvent::ExpiredListenAddr { address, .. } => Self::ExpiredListenAddr {
                address: address.clone(),
            },
            SwarmEvent::OutgoingConnectionError { error, .. } => Self::OutgoingConnectionError {
                error: error.into(),
            },
            _ => return Err(()),
        };
        Ok(ev)
    }
}

#[allow(unused)]
mod serde_types {
    use std::f32::consts::E;

    use libp2p::{core::Endpoint, PeerId, TransportError};
    use owlnest::net::p2p::{swarm, Multiaddr};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize)]
    pub enum ConnectedPoint {
        Dialer {
            address: Multiaddr,
            role_override: bool,
        },
        Listener {
            local_addr: Multiaddr,
            send_back_addr: Multiaddr,
        },
    }
    impl From<&swarm::ConnectedPoint> for ConnectedPoint {
        fn from(value: &swarm::ConnectedPoint) -> Self {
            match value {
                swarm::ConnectedPoint::Dialer {
                    address,
                    role_override,
                } => {
                    let is_override = if let Endpoint::Dialer = role_override {
                        false
                    } else {
                        true
                    };
                    Self::Dialer {
                        address: address.clone(),
                        role_override: is_override,
                    }
                }
                swarm::ConnectedPoint::Listener {
                    local_addr,
                    send_back_addr,
                } => Self::Listener {
                    local_addr: local_addr.clone(),
                    send_back_addr: send_back_addr.clone(),
                },
            }
        }
    }

    /// Possible errors when trying to establish or upgrade an outbound connection.
    #[derive(Debug, Serialize, Clone)]
    pub enum DialError {
        /// The peer identity obtained on the connection matches the local peer.
        LocalPeerId {
            endpoint: ConnectedPoint,
        },
        /// No addresses have been provided by [`NetworkBehaviour::handle_pending_outbound_connection`] and [`DialOpts`].
        /// Which basically never happens.
        NoAddresses,
        /// The provided [`dial_opts::PeerCondition`] evaluated to false and thus
        /// the dial was aborted.
        DialPeerConditionFalse,
        /// Pending connection attempt has been aborted.
        Aborted,
        /// The peer identity obtained on the connection did not match the one that was expected.
        WrongPeerId {
            obtained: PeerId,
            endpoint: ConnectedPoint,
        },
        Denied(String),
        /// An error occurred while negotiating the transport protocol(s) on a connection.
        Transport(Vec<(Multiaddr, String)>),
    }

    impl From<&libp2p::swarm::DialError> for DialError {
        fn from(value: &libp2p::swarm::DialError) -> Self {
            match value {
                libp2p::swarm::DialError::LocalPeerId { endpoint } => Self::LocalPeerId {
                    endpoint: endpoint.into(),
                },
                libp2p::swarm::DialError::NoAddresses => Self::NoAddresses,
                libp2p::swarm::DialError::DialPeerConditionFalse(_) => Self::DialPeerConditionFalse,
                libp2p::swarm::DialError::Aborted => Self::Aborted,
                libp2p::swarm::DialError::WrongPeerId { obtained, endpoint } => Self::WrongPeerId {
                    obtained: *obtained,
                    endpoint: endpoint.into(),
                },
                libp2p::swarm::DialError::Denied { cause } => Self::Denied(cause.to_string()),
                libp2p::swarm::DialError::Transport(e) => {
                    let closure =
                        |err: &(Multiaddr, libp2p::TransportError<std::io::Error>)| match &err.1 {
                            TransportError::MultiaddrNotSupported(addr) => {
                                (addr.clone(), "Multiaddr Not Supported".to_string())
                            }
                            TransportError::Other(e) => {
                                let err_string = format!("{:?}", value);
                                let regex =
                                    regex::Regex::new("kind: [^,]*, m").expect("valid regex");
                                if let Some(matched) = regex.find(&err_string) {
                                    let matched_str = matched.as_str();
                                    (
                                        err.0.clone(),
                                        matched_str[6..matched_str.len() - 3].to_owned(),
                                    )
                                } else {
                                    (err.0.clone(), format!("{:?}", e))
                                }
                            }
                        };
                    let info = e.iter().map(closure).collect::<Vec<(Multiaddr, String)>>();
                    Self::Transport(info)
                }
            }
        }
    }
}
