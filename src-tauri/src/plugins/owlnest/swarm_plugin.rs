use owlnest::net::p2p::{swarm, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

struct State {
    pub swarm_manager: swarm::manager::Manager,
}

use super::*;
pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    Builder::new("swarm")
        .setup(|app| {
            let app_handle = app.clone();
            let mut listener = manager.event_subscriber().subscribe();
            async_runtime::spawn(async move {
                loop {
                    if let Ok(ev) = listener.recv().await {
                        if let Ok(ev) = TryInto::<SwarmEventEmit>::try_into(ev.as_ref()) {
                            let _ = app_handle.emit_all("swarm-event", ev);
                        }
                    }
                }
            });
            app.manage(State {
                swarm_manager: manager,
            });
            Ok(())
        })
        .invoke_handler(generate_handler![dial])
        .build()
}

#[tauri::command]
async fn dial(state: tauri::State<'_, State>, dial_options: DialOptions) -> Result<(), String> {
    let addr = dial_options.address.parse().map_err(|e| format!("{}", e))?;
    state
        .swarm_manager
        .swarm()
        .dial(&addr)
        .map_err(|e| format!("{}", e))
}

#[derive(Debug, Deserialize)]
pub struct DialOptions {
    pub address: String,
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
                ..
            } => Self::ConnectionClosed {
                peer_id: *peer_id,
                endpoint: endpoint.into(),
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
            _ => return Err(()),
        };
        Ok(ev)
    }
}

#[allow(unused)]
mod serde_types {
    use libp2p::core::Endpoint;
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

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ConnectionId(u64);
    impl From<&owlnest::net::p2p::swarm::ConnectionId> for ConnectionId {
        fn from(value: &owlnest::net::p2p::swarm::ConnectionId) -> Self {
            Self(value.into_inner().try_into().unwrap())
        }
    }
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ListenerId(u64);
    impl From<&libp2p::core::transport::ListenerId> for ListenerId {
        fn from(value: &libp2p::core::transport::ListenerId) -> Self {
            Self(value.into_inner().try_into().unwrap())
        }
    }
}
