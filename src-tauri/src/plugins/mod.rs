pub mod openssl;
pub mod grpc;
pub mod config;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

