use std::{cmp::Ordering, time::Duration};

use super::*;
use dashmap::DashMap;
use libp2p::StreamProtocol;
use owlnest::net::p2p::protocols::{
    ping,
    relay_server::{HOP_PROTOCOL_NAME, STOP_PROTOCOL_NAME},
};
use swarm::SwarmEvent;

const ADVERTISE_PROTOCOL: StreamProtocol =
    StreamProtocol::new(owlnest::net::p2p::protocols::advertise::PROTOCOL_NAME);

#[derive(Clone)]
struct State {
    connected: Arc<DashMap<PeerId, RelayInfo>>,
    manager: swarm::Manager,
}
impl State {
    /// Remove closed listeners from listened address on relays
    pub(crate) fn on_self_listener_closed(&self, closed_listeners: &Vec<Multiaddr>) {
        for address in closed_listeners {
            for mut relay in self.connected.iter_mut() {
                if relay
                    .value_mut()
                    .listened_address
                    .extract_if(|v| address.to_string().contains(&v.to_string()))
                    .count()
                    > 0
                {
                    break;
                }
            }
        }
    }
    pub(crate) fn on_identified(&self, peer_id: &PeerId, info: &identify::Info) {
        if !(info.protocols.contains(&HOP_PROTOCOL_NAME)
            || info.protocols.contains(&STOP_PROTOCOL_NAME))
        {
            return;
        }
        let connected_list = &self.connected;
        if let None = connected_list.get(peer_id) {
            connected_list.insert(
                *peer_id,
                RelayInfo {
                    listenable_address: HashSet::from_iter(info.listen_addrs.iter().cloned()),
                    listened_address: HashSet::new(),
                    supports_ext: info.protocols.contains(&ADVERTISE_PROTOCOL),
                    latency: -1,
                },
            );
            return;
        };

        let mut entry_to_update = connected_list.get_mut(peer_id).unwrap();
        entry_to_update.value_mut().listened_address.retain(|v| {
            info.listen_addrs
                .iter()
                .filter(|new_addr| v.to_string().contains(&new_addr.to_string()))
                .next()
                .is_some()
        });
        entry_to_update.value_mut().listenable_address = HashSet::from_iter(
            info.listen_addrs
                .iter()
                .filter(|v| {
                    !entry_to_update
                        .listened_address
                        .iter()
                        .find(|addr| addr.to_string().contains(&v.to_string()))
                        .is_some()
                })
                .cloned(),
        );
    }
    pub(crate) fn on_ping_update(&self, ev: &ping::OutEvent) {
        let maybe_entry = self.connected.get_mut(&ev.peer);
        if maybe_entry.is_none() {
            return;
        }
        let mut entry = maybe_entry.unwrap();
        if ev.result.is_err() {
            entry.value_mut().latency = -1;
            return;
        }
        entry.value_mut().latency = ev
            .result
            .as_ref()
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap_or(isize::MAX);
    }
    pub(crate) fn on_new_listener(&self, address: &Multiaddr) {
        let address_string = address.to_string();
        if let Some(mut entry) = self
            .connected
            .iter_mut()
            .filter(|entry| {
                let (k, v) = entry.pair();
                // get the one that has the address that is part of the newly listened address
                v.listenable_address
                    .iter()
                    .filter(|_| address_string.contains(&format!("{}/p2p-circuit", k)))
                    .next()
                    .is_some()
            })
            .next()
        {
            let v = entry.value_mut();
            v.listenable_address
                .retain(|v| !address_string.contains(&v.to_string()));
            v.listened_address.insert(address.clone());
        };
    }
}

pub fn init<R: Runtime>(peer_manager: swarm::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-relay")
        .setup(|app, _api| {
            let state = State {
                connected: Default::default(),
                manager: peer_manager.clone(),
            };
            app.manage(state.clone());
            let _app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = peer_manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Identify(ev)) = ev.as_ref()
                    {
                        match ev {
                            identify::OutEvent::Received { peer_id, info, .. } => {
                                state.on_identified(peer_id, info)
                            }
                            _ => {}
                        }
                        continue;
                    }
                    match ev.as_ref() {
                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                            state.on_new_listener(address)
                        }
                        libp2p::swarm::SwarmEvent::ListenerClosed { addresses, .. } => {
                            state.on_self_listener_closed(addresses)
                        }
                        _ => {}
                    }
                    if let swarm::SwarmEvent::ConnectionClosed {
                        peer_id,
                        num_established,
                        ..
                    } = ev.as_ref()
                    {
                        if *num_established < 1 {
                            state.connected.remove(peer_id);
                        }
                        continue;
                    }
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Ping(ev)) = ev.as_ref() {
                        state.on_ping_update(ev)
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            list_relays,
            query_advertised,
            set_remote_advertisement,
            set_local_provider_state,
            get_relay_info,
            listen_relay
        ])
        .build()
}

#[tauri::command]
async fn list_relays(state: tauri::State<'_, State>) -> Result<Vec<RelayStub>, String> {
    let relays = &state.connected;
    let mut list = relays
        .iter()
        .map(|entry| (*entry.key(), entry.value().latency).into())
        .collect::<Vec<RelayStub>>();
    list.sort_unstable_by(|a, b| {
        if a.latency >= 0 {
            a.latency.cmp(&b.latency)
        } else {
            Ordering::Greater
        }
    });
    Ok(list)
}

#[tauri::command]
async fn get_relay_info(
    state: tauri::State<'_, State>,
    relay: PeerId,
) -> Result<Option<RelayInfo>, String> {
    Ok(state
        .connected
        .get(&relay)
        .map(|entry| entry.value().clone()))
}

#[tauri::command]
async fn query_advertised(
    state: tauri::State<'_, swarm::Manager>,
    relay: PeerId,
) -> Result<Option<Box<[PeerId]>>, String> {
    state
        .advertise()
        .query_advertised_peer(relay)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_remote_advertisement(
    state: tauri::State<'_, swarm::Manager>,
    relay: PeerId,
    advertisement_state: bool,
) -> Result<(), String> {
    Ok(state
        .advertise()
        .set_remote_advertisement(&relay, advertisement_state)
        .await)
}

#[tauri::command]
async fn set_local_provider_state(
    state: tauri::State<'_, swarm::Manager>,
    provider_state: bool,
) -> Result<(), String> {
    state.advertise().set_provider_state(provider_state).await;
    Ok(())
}

#[tauri::command]
async fn listen_relay(
    state: tauri::State<'_, State>,
    relay_address: Multiaddr,
) -> Result<(), String> {
    let manager = state.manager.clone();
    let handle = async move {
        let mut listener = manager.event_subscriber().subscribe();
        if let Err(e) = manager.swarm().listen(&relay_address).await {
            return Err(e);
        };
        while let Ok(ev) = listener.recv().await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev.as_ref() {
                if address.to_string().contains(&relay_address.to_string()) {
                    return Ok(());
                }
            }
        }
        unreachable!()
    };
    let result = tokio::time::timeout(Duration::from_secs(10), handle).await;
    if result.is_err() {
        return Err("Timeout waiting for future to complete".into());
    }
    let result = result.unwrap();
    result.map_err(|e| format!("{:?}", e))
}

#[derive(Debug, Clone, Serialize)]
struct RelayInfo {
    pub listenable_address: HashSet<Multiaddr>,
    pub listened_address: HashSet<Multiaddr>,
    pub supports_ext: bool,
    pub latency: isize,
}

#[derive(Debug, Clone, Serialize)]
struct RelayStub {
    peer_id: PeerId,
    latency: isize,
}
impl From<(PeerId, isize)> for RelayStub {
    fn from(value: (PeerId, isize)) -> Self {
        RelayStub {
            peer_id: value.0,
            latency: value.1,
        }
    }
}
