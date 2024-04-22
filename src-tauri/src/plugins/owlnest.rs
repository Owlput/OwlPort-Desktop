extern crate owlnest;
use libp2p::{Multiaddr, PeerId};
use owlnest::net::p2p::swarm::behaviour::BehaviourEvent;
use owlnest::net::p2p::{identity::IdentityUnion, protocols::identify, swarm, SwarmConfig};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tauri::async_runtime;
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tokio::sync::RwLock;

pub mod messaging;
// pub mod statistics;
pub mod advertise;
pub mod autonat;
pub mod blob;
pub mod kad;
pub mod mdns;
pub mod relay;
pub mod swarm_plugin;
pub mod upnp;

pub fn setup_peer() -> swarm::Manager {
    #[cfg(not(feature = "debug"))]
    let identity = read_ident();
    #[cfg(feature = "debug")]
    let identity = IdentityUnion::generate();
    let swarm_config = SwarmConfig {
        local_ident: identity.clone(),
        kad: Default::default(),
        identify: identify::Config::new("/owlnest/0.0.1".into(), identity.clone().get_pubkey()),
        mdns: Default::default(),
        messaging: Default::default(),
        relay_server: Default::default(),
    };
    swarm::Builder::new(swarm_config).build(16, async_runtime::handle().inner().clone())
}

#[cfg(not(feature = "debug"))]
fn read_ident() -> IdentityUnion {
    use tracing::warn;
    match IdentityUnion::from_file_protobuf_encoding("./id.libp2pkeypair") {
        Ok(ident) => ident,
        Err(e) => {
            warn!("Failed to read keypair: {:?}", e);
            let ident = IdentityUnion::generate();
            ident.export_keypair(".", "id").unwrap();
            ident
        }
    }
}
