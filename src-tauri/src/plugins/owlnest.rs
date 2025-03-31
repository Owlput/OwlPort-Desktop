extern crate owlnest;
use libp2p::{Multiaddr, PeerId};
use owlnest::net::p2p::swarm::behaviour::BehaviourEvent;
use owlnest::net::p2p::{identity::IdentityUnion, protocols::identify, swarm, SwarmConfig};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::path::Path;
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
            println!("Cannot read config file: {}", e)
        };
        match toml::from_str(&config_text) {
            Ok(v) => return Ok(v),
            Err(e) => println!("Cannot parse config file: {}", e),
        }
    }
    println!("Cannot load config, trying to generate example config file...");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./owlnest_config.toml.example")
    {
        let default_config = SwarmConfig::default();
        if let Err(e) = f.write(
            toml::to_string_pretty(&default_config)
                .expect("Serialization to succeed")
                .as_bytes(),
        ) {
            println!("Cannot write example config: {}", e);
        }
        let _ = f.flush();
        drop(f);
        println!("Example config file has been generated.")
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Cannot read configuration file",
    ))
}
