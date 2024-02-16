use super::*;
use owlnest::net::p2p::{protocols::upnp, swarm::behaviour::BehaviourEvent};
use std::{collections::HashSet, sync::atomic::AtomicI8};

///
/// Gateway state:
/// - -1: Gateway not found or doesn't support Upnp;
/// - 0: Gateway supports Upnp but not publicly reachable;
/// - 1: Gateway supports Upnp and publicly reachable.
#[derive(Debug, Clone, Default)]
struct State {
    gateway_state: Arc<AtomicI8>,
    external_addr: Arc<RwLock<HashSet<Multiaddr>>>,
}

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-upnp")
        .setup(|app| {
            let state = State::default();
            let app_handle = app.clone();
            let state_clone = state.clone();
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Upnp(ev)) = ev.as_ref() {
                        let _ = app_handle.emit_all::<UpnpEmit>("owlnest-upnp-emit", ev.into());
                        match ev {
                            upnp::OutEvent::NewExternalAddr(addr) => {
                                state
                                    .gateway_state
                                    .store(1, std::sync::atomic::Ordering::SeqCst);
                                state.external_addr.write().await.insert(addr.clone());
                            }
                            upnp::OutEvent::ExpiredExternalAddr(addr) => {
                                state.external_addr.write().await.remove(addr);
                            }
                            upnp::OutEvent::GatewayNotFound => state
                                .gateway_state
                                .store(-1, std::sync::atomic::Ordering::SeqCst),
                            upnp::OutEvent::NonRoutableGateway => state
                                .gateway_state
                                .store(0, std::sync::atomic::Ordering::SeqCst),
                        }
                    }
                }
            });
            app.manage(state_clone);
            Ok(())
        })
        .invoke_handler(generate_handler![
            list_available_external_addr,
            get_gateway_status
        ])
        .build()
}

#[tauri::command]
async fn list_available_external_addr(
    state: tauri::State<'_, State>,
) -> Result<Vec<Multiaddr>, String> {
    Ok(state.external_addr.read().await.iter().cloned().collect())
}

#[tauri::command]
async fn get_gateway_status(state: tauri::State<'_, State>) -> Result<i8, String> {
    Ok(state
        .gateway_state
        .load(std::sync::atomic::Ordering::SeqCst))
}

#[derive(Debug, Clone, Serialize)]
enum UpnpEmit {
    NewExternalAddr(Multiaddr),
    ExpiredExternalAddr(Multiaddr),
    GatewayNotFound,
    NonRoutableGateway,
}
impl From<&upnp::OutEvent> for UpnpEmit {
    fn from(value: &upnp::OutEvent) -> Self {
        match value {
            upnp::OutEvent::NewExternalAddr(addr) => Self::NewExternalAddr(addr.clone()),
            upnp::OutEvent::ExpiredExternalAddr(addr) => Self::ExpiredExternalAddr(addr.clone()),
            upnp::OutEvent::GatewayNotFound => Self::GatewayNotFound,
            upnp::OutEvent::NonRoutableGateway => Self::NonRoutableGateway,
        }
    }
}
