extern crate owlnest;
use libp2p::{Multiaddr, PeerId};
use owlnest::net::p2p::swarm::behaviour::BehaviourEvent;
use owlnest::net::p2p::{SwarmConfig, identity::IdentityUnion, protocols::identify, swarm};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tauri::async_runtime;
use tauri::{
    Manager, Runtime, generate_handler,
    plugin::{Builder, TauriPlugin},
};
use tokio::sync::RwLock;
use tracing::error;

pub mod messaging;
// pub mod statistics;
pub mod advertise;
pub mod autonat;
pub mod blob;
pub mod developer_options;
pub mod gossipsub;
pub mod kad;
pub mod mdns;
pub mod relay;
pub mod swarm_plugin;
pub mod upnp;

pub fn setup_peer() -> anyhow::Result<swarm::Manager> {
    let config = read_config("./owlnest_config.toml")?;
    let ident = if !config.swarm.identity_path.is_empty() {
        read_ident(&config.swarm.identity_path)?
    } else {
        IdentityUnion::generate()
    };
    Ok(swarm::Builder::new(config).build(ident, async_runtime::handle().inner().clone()))
}

fn read_ident(path: impl AsRef<Path>) -> Result<IdentityUnion, std::io::Error> {
    use tracing::warn;
    match IdentityUnion::from_file_protobuf_encoding(path.as_ref()) {
        Ok(ident) => Ok(ident),
        Err(e) => {
            warn!("Failed to read keypair: {:?}", e);
            let ident = IdentityUnion::generate();
            ident.export_keypair("./id.libp2pkp")?;
            Ok(ident)
        }
    }
}

fn read_config(path: impl AsRef<Path>) -> Result<SwarmConfig, std::io::Error> {
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .read(true)
        .create(false)
        .open(path.as_ref())
    {
        let mut config_text = String::new();
        if let Err(e) = f.read_to_string(&mut config_text) {
            error!("Cannot read config file: {}", e)
        };
        match toml::from_str(&config_text) {
            Ok(v) => return Ok(v),
            Err(e) => error!("Cannot parse config file: {}", e),
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Cannot read configuration file",
    ))
}
