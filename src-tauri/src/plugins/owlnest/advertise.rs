use super::*;
use owlnest::net::p2p::protocols::advertise;
use std::sync::atomic::AtomicBool;

#[derive(Debug, Clone)]
struct State {
    /// The state of the provider
    /// `true` for active, `false` for inactive.
    is_providing: Arc<AtomicBool>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            is_providing: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-advertise")
        .setup(|app| {
            let app_handle = app.clone();
            let state = State::default();
            app.manage(state.clone());
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Advertise(ev)) = ev.as_ref()
                    {
                        if let Ok(ev) = ev.try_into() {
                            let _ =
                                app_handle.emit_all::<AdvertiseEmit>("owlnest-advertise-emit", ev);
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            get_provider_state,
            set_provider_state,
            query_advertised,
            set_remote_advertisement
        ])
        .build()
}

#[tauri::command]
async fn get_provider_state(state: tauri::State<'_, swarm::Manager>) -> Result<bool, ()> {
    Ok(state.advertise().provider_state().await)
}

#[tauri::command]
async fn set_provider_state(
    state: tauri::State<'_, swarm::Manager>,
    provider_state: bool,
) -> Result<bool, ()> {
    Ok(state.advertise().set_provider_state(provider_state).await)
}

#[tauri::command]
async fn query_advertised(
    state: tauri::State<'_, swarm::Manager>,
    peer_id: PeerId,
) -> Result<Vec<PeerId>, advertise::Error> {
    state.advertise().query_advertised_peer(peer_id).await
}

#[tauri::command]
async fn set_remote_advertisement(
    state: tauri::State<'_, swarm::Manager>,
    remote: PeerId,
    advertisement_state: bool,
) -> Result<(), ()> {
    Ok(state
        .advertise()
        .set_remote_advertisement(remote, advertisement_state)
        .await)
}

#[derive(Debug, Serialize, Clone)]
enum AdvertiseEmit {
    QueryAnswered { from: PeerId, list: Vec<PeerId> },
    RemoteAdvertisementResult { from: PeerId, result: bool },
    ProviderState { state: bool },
    AdvertisedPeerChanged { peer: PeerId, state: bool },
    Error(String),
}
impl TryFrom<&advertise::OutEvent> for AdvertiseEmit {
    type Error = ();
    fn try_from(value: &advertise::OutEvent) -> Result<Self, ()> {
        use advertise::OutEvent::*;
        let result = match value {
            QueryAnswered { from, list } => Self::QueryAnswered {
                from: *from,
                list: list.clone(),
            },
            RemoteAdvertisementResult { from, result } => Self::RemoteAdvertisementResult {
                from: *from,
                result: result.is_ok(),
            },
            ProviderState(state, _) => Self::ProviderState { state: *state },
            AdvertisedPeerChanged(peer_id, state) => Self::AdvertisedPeerChanged {
                peer: *peer_id,
                state: *state,
            },
            Error(e) => Self::Error(format!("{:?}", e.to_string())),
        };
        Ok(result)
    }
}
