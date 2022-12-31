use tauri::{async_runtime, Manager};
use tracing::{debug, warn};

use super::*;
use crate::net::grpc::nest_rpc::generated::*;
use async_runtime::RwLock;
// use std::path::Path;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

type Client = nest_rpc_client::NestRpcClient<Channel>;

struct MyState {
    grpc_client: RwLock<Result<Client, tonic::transport::Error>>, // `DerefMut` not implemented for State<_,S>, so lock is required.
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let client = match setup_client(None /*&Path::new("../certs/owlnest_test_cert.pem")*/) {
        Ok(client) => RwLock::new(Ok(client)),
        Err(e) => RwLock::new(Err(e)),
    };

    Builder::new("grpc_hb")
        .invoke_handler(tauri::generate_handler![hb, reconnect])
        .setup(|app_handle| {
            app_handle.manage(MyState {
                grpc_client: client,
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
            Err(e.message().to_string())
        }
    }
}
#[tauri::command]
fn reconnect(state: tauri::State<'_, MyState>) -> Result<(), String> {
    debug!("Trying to reconnect to nest");
    match setup_client(/* &Path::new("../certs/owlnest_test_cert.pem" */ None) {
        Ok(client) => {
            debug!("Successfully reconnected");
            *state.grpc_client.blocking_write() = Ok(client);
            Ok(())
        }
        Err(e) => {
            let error_string = e.to_string();
            *state.grpc_client.blocking_write() = Err(e);
            Err(error_string)
        }
    }
}

fn setup_client(path: Option<&std::path::Path>) -> Result<Client, tonic::transport::Error> {
    if let Some(path) = path {
        let pem = async_runtime::block_on(tokio::fs::read(path)).unwrap();
        let ca = Certificate::from_pem(pem);
        let conf = ClientTlsConfig::new().ca_certificate(ca);
        let channel = match async_runtime::block_on(
            Channel::from_static("https://127.0.0.1:20001")
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
            match async_runtime::block_on(Channel::from_static("http://127.0.0.1:20001").connect())
            {
                Ok(channel) => Ok(channel),
                Err(e) => Err(e),
            };
        match channel {
            Ok(channel) => Ok(Client::new(channel)),
            Err(e) => Err(e),
        }
    }
}
