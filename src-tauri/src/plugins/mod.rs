pub mod openssl;

use std::error::Error;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
pub mod grpc {
    use tauri::{async_runtime, Manager};

    use super::*;
    use async_runtime::RwLock;
    use tonic::transport::{Certificate, Channel, ClientTlsConfig};

    use crate::net::grpc::hello_world::{greeter_client::GreeterClient, HbRequest};

    struct MyState {
        grpc_client: RwLock<Result<GreeterClient<Channel>, tonic::transport::Error>>, // `DerefMut` not implemented for State<_,S>, so lock is required.
    }

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        let pem = async_runtime::block_on(tokio::fs::read("../certs/owlnest_test_cert.pem")).unwrap();
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
        let client = match channel {
            Ok(channel) => RwLock::new(Ok(GreeterClient::new(channel))),
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

    // remember to call `.manage(MyState::default())`
    #[tauri::command]
    async fn hb(state: tauri::State<'_, MyState>) -> Result<String, String> {
        match &*state.grpc_client.read().await {
            Ok(client) => {
                let mut client = client.clone();
                match client
                    .hb(HbRequest {
                        rand: rand::random::<i64>(),
                    })
                    .await
                {
                    Ok(res) => Ok(res.into_inner().rand.to_string()),
                    Err(e) => Err(e.message().to_string()),
                }
            }
            Err(e) => Err(format!("{:#?}", e.source().unwrap())),
        }
    }

    #[tauri::command]
    async fn reconnect(state: tauri::State<'_, MyState>) -> Result<(), String> {
        match GreeterClient::connect("http://127.0.0.1:20001").await {
            Ok(client) => {
                *state.grpc_client.write().await = Ok(client);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
