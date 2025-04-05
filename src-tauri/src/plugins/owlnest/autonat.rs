use super::*;
use autonat::OutEvent;
use owlnest::net::p2p::protocols::autonat;
use std::str::FromStr;
use tauri::Emitter;

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-autonat")
        .setup(|app, _api| {
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::AutoNat(ev)) = ev.as_ref() {
                        if let Ok(ev) = ev.try_into() {
                            let _ = app_handle.emit::<AutoNatEmit>("owlnest-autonat-emit", ev);
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            add_server,
            remove_server,
            probe,
            get_nat_status
        ])
        .build()
}

#[tauri::command]
async fn add_server(
    state: tauri::State<'_, swarm::Manager>,
    peer: String,
    address: Option<String>,
) -> Result<(), String> {
    let address = if let Some(v) = address {
        Some(Multiaddr::from_str(&v).map_err(|e| e.to_string())?)
    } else {
        None
    };
    let peer_id = PeerId::from_str(&peer).map_err(|e| e.to_string())?;
    let _ = state.autonat().add_server(&peer_id, address).await;
    Ok(())
}
#[tauri::command]
async fn remove_server(
    state: tauri::State<'_, swarm::Manager>,
    peer: String,
) -> Result<(), String> {
    let peer = PeerId::from_str(&peer).map_err(|e| e.to_string())?;
    let _ = state.autonat().remove_server(&peer).await;
    Ok(())
}
#[tauri::command]
async fn probe(state: tauri::State<'_, swarm::Manager>, address: String) -> Result<(), String> {
    let address = Multiaddr::from_str(&address).map_err(|e| e.to_string())?;
    let _ = state.autonat().probe(&address).await;
    Ok(())
}

#[tauri::command]
async fn get_nat_status(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<serde_types::NatStatusWithConfidence, String> {
    Ok(state.autonat().get_nat_status().await.into())
}

#[derive(Debug, Serialize, Clone)]
enum AutoNatEmit {
    StatusChanged {
        old: serde_types::NatStatus,
        new: serde_types::NatStatus,
    },
    OutboundProbe(serde_types::OutboundProbeEvent),
}
impl TryFrom<&OutEvent> for AutoNatEmit {
    type Error = ();
    fn try_from(value: &OutEvent) -> Result<Self, ()> {
        let ev = match value {
            OutEvent::StatusChanged { old, new } => Self::StatusChanged {
                old: old.clone().into(),
                new: new.clone().into(),
            },
            OutEvent::OutboundProbe(ev) => Self::OutboundProbe(match ev {
                libp2p::autonat::OutboundProbeEvent::Request { .. } => return Err(()),
                libp2p::autonat::OutboundProbeEvent::Response { peer, address, .. } => {
                    serde_types::OutboundProbeEvent::Response {
                        peer: *peer,
                        address: address.clone(),
                    }
                }
                libp2p::autonat::OutboundProbeEvent::Error { peer, error, .. } => {
                    serde_types::OutboundProbeEvent::Error {
                        peer: *peer,
                        error: format!("{:?}", error),
                    }
                }
            }),
            _ => return Err(()),
        };
        Ok(ev)
    }
}
mod serde_types {
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    pub enum OutboundProbeEvent {
        /// The remote successfully dialed one of our addresses.
        Response {
            /// Id of the peer that sent the response.
            peer: PeerId,
            /// The address at which the remote succeeded to dial us.
            address: Multiaddr,
        },
        /// The outbound request failed, was rejected, or the remote could dial
        /// none of our addresses.
        Error { peer: Option<PeerId>, error: String },
    }

    #[derive(Debug, Serialize, Clone)]
    pub enum NatStatus {
        Public { address: Multiaddr },
        Private,
        Unknown,
    }
    impl From<autonat::NatStatus> for NatStatus {
        fn from(value: autonat::NatStatus) -> Self {
            match value {
                autonat::NatStatus::Public(address) => Self::Public { address },
                autonat::NatStatus::Private => Self::Private,
                autonat::NatStatus::Unknown => Self::Unknown,
            }
        }
    }
    #[derive(Debug, Clone, Serialize)]
    pub struct NatStatusWithConfidence {
        pub status: NatStatus,
        pub confidence: usize,
    }
    impl From<(autonat::NatStatus, usize)> for NatStatusWithConfidence {
        fn from(value: (autonat::NatStatus, usize)) -> Self {
            Self {
                status: value.0.into(),
                confidence: value.1,
            }
        }
    }
}
