extern crate owlnest;
use libp2p::{Multiaddr, PeerId};
use owlnest::net::p2p::{identity::IdentityUnion, protocols::identify, swarm, SwarmConfig};
use serde::{Serialize,Deserialize};
use tauri::async_runtime;
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod messaging;
// pub mod statistics;
pub mod swarm_plugin;
pub mod mdns;

pub fn setup_peer() -> swarm::Manager {
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
