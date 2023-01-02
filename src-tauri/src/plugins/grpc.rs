use std::str::FromStr;

use tauri::{async_runtime, http::Uri, Manager};
use tracing::{debug, warn};

use super::{
    config::{self, ConfigPath, app_config::{AppConfig, NestConenction}},
    *,
};
use crate::net::grpc::nest_rpc::generated::*;
use async_runtime::RwLock;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

type Client = nest_rpc_client::NestRpcClient<Channel>;

struct MyState {
    grpc_client: RwLock<Result<Client, tonic::transport::Error>>, // `DerefMut` not implemented for State<_,S>, so lock is required.
    default_nest_address: String,
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let AppConfig { nest } = match config::read_from_file(ConfigPath::AppConfig(None)) {
        Ok(v) => match v{
            config::ConfigEnum::AppConfig(v)=>v,
            _ => AppConfig::default()
        },
        Err(_) => AppConfig::default(),
    };
    let NestConenction{address,cert_path} = nest;
    let client = match setup_client(address.parse().unwrap(), cert_path) {
        Ok(client) => RwLock::new(Ok(client)),
        Err(e) => RwLock::new(Err(e)),
    };

    Builder::new("grpc_hb")
        .invoke_handler(tauri::generate_handler![hb, reconnect])
        .setup(|app_handle| {
            app_handle.manage(MyState {
                grpc_client: client,
                default_nest_address: address,
            });
            Ok(())
        })
        .build()
}

#[tauri::command]
async fn hb(state: tauri::State<'_, MyState>) -> Result<String, String> {
    let mut client = state.grpc_client.write().await;
    match client
        .as_mut()
        .map_err(|e| e.to_string())?
        .heartbeat(HeartbeatRequest {
            client: "Dev Client".into(),
        })
        .await
    {
        Ok(res) => {
            debug!("Successfully completed heartbeat");
            Ok(res.into_inner().rand.to_string())
        }
        Err(e) => {
            warn!("Failed to perform heartbeat with error {}", &e);
            Err(format!("error.plugin.grpc.nest.heartbeat: {:?}", e))
        }
    }
}
#[tauri::command]
fn reconnect(state: tauri::State<'_, MyState>, nest_address: Option<String>) -> Result<(), String> {
    debug!("Trying to reconnect to nest");
    let uri = match Uri::from_str(&nest_address.unwrap_or(state.default_nest_address.clone())) {
        Ok(v) => v,
        Err(e) => return Err(format!("error.parsing: {:?}", e)),
    };
    match setup_client(uri, None) {
        Ok(client) => {
            debug!("Successfully reconnected");
            *state.grpc_client.blocking_write() = Ok(client);
            Ok(())
        }
        Err(e) => {
            let error_string = e.to_string();
            *state.grpc_client.blocking_write() = Err(e);
            Err(format!("error.plugin.grpc.connection: {:?}", error_string))
        }
    }
}

fn setup_client(
    nest_address: Uri,
    cert_path: Option<String>,
) -> Result<Client, tonic::transport::Error> {
    if let Some(path) = cert_path {
        let pem = async_runtime::block_on(tokio::fs::read(path)).unwrap();
        let ca = Certificate::from_pem(pem);
        let conf = ClientTlsConfig::new().ca_certificate(ca);
        let channel = match async_runtime::block_on(
            Channel::builder(nest_address.clone())
                .tls_config(conf)
                .unwrap()
                .connect(),
        ) {
            Ok(channel) => Ok(channel),
            Err(e) => Err(e),
        };
        match channel {
            Ok(channel) => Ok(Client::new(channel)),
            Err(e) => Err(e),
        }
    } else {
        let channel =
            match async_runtime::block_on(Channel::builder(nest_address.clone()).connect()) {
                Ok(channel) => Ok(channel),
                Err(e) => Err(e),
            };
        match channel {
            Ok(channel) => Ok(Client::new(channel)),
            Err(e) => Err(e),
        }
    }
}
