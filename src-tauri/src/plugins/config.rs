use serde::{Deserialize, Serialize};
use crate::impl_config;
use self::app_config::AppConfig;
use self::nest_config::NestConfig;
use super::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("configuration")
        .invoke_handler(tauri::generate_handler![read_config, write_config])
        .build()
}

#[tauri::command]
pub fn read_config(path: ConfigPath) -> Result<String, String> {
    match read_from_file(path) {
        Ok(v) => Ok(serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            match e {
                Error::Io(e) => Err(format!("error.io.file.read: {:?}", e)),
                Error::Deserialize(e) => Err(format!("error.parsing: {:?}", e)),
                Error::Serialize(e) => Err(format!("error.internal.bug: {:?}", e)), // Shouldn't happen
            }
        }
    }
}

#[tauri::command]
pub fn write_config(config: ConfigEnum, path: ConfigPath) -> Result<(), String> {
    match write_to_file(config, path) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e {
                Error::Io(e) => Err(format!("error.io.file.write: {:?}", e)),
                Error::Serialize(e) => Err(format!("error.plugin.config.parse: {:?}", e)),
                Error::Deserialize(e) => Err(format!("error.internal.bug: {:?}", e)), // Shouldn't happen
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
}

impl_config!(AppConfig:"conf/app_config.toml",NestConfig:"conf/nest_config.toml");

/// Read config from a specified path, or from the default location if no path is specified.
/// This function won't check whether the given config belongs to the given path.
pub fn read_from_file(path: ConfigPath) -> Result<ConfigEnum, Error> {
    let (path, config_type,_) = match_config(path,None);
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => return Err(Error::Io(e)),
    };
    matched_deserialize(config_type, bytes)
}
/// Write the config to a specified path, or to the default location if no path is specified.
/// This will overwrite the entire file if it exists, or create a new one when not.
/// This function won't check whether the given config belongs to the given path.
pub fn write_to_file(config: ConfigEnum, path: ConfigPath) -> Result<(), Error> {
    let (path, _, bytes) = match_config(path,Some(config));
    let bytes = match bytes.unwrap() {
        Ok(bytes) => bytes,
        Err(e) => return Err(Error::Serialize(e)),
    };
    match std::fs::write(path, bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Io(e)),
    }
}

pub mod app_config {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct AppConfig {
        pub nest: NestConenction,
    }
    impl Default for AppConfig {
        fn default() -> Self {
            Self {
                nest: NestConenction {
                    address: "http://127.0.0.1:20001".into(),
                    cert_path: None,
                },
            }
        }
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct NestConenction {
        pub address: String,
        #[serde(default)]
        pub cert_path: Option<String>,
    }
}
pub mod nest_config {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct NestConfig {
        dummy: Option<()>,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore = "file operation needed"]
    fn deserialize() {
        assert_eq!(
            read_from_file(ConfigPath::AppConfig(None)).unwrap(),
            ConfigEnum::AppConfig(AppConfig {
                nest: app_config::NestConenction {
                    address: "http://127.0.0.1:20001".into(),
                    cert_path: None
                },
            })
        );
    }
    #[test]
    #[ignore = "human verification needed"]
    fn serialize() {
        println!("serializing TOML style AppConfig:\n{}",toml::to_string_pretty(&AppConfig::default()).unwrap());
        println!("serializing JSON style AppConfig:\n{}",serde_json::to_string_pretty(&AppConfig::default()).unwrap());
        println!("serializing JSON style ConfigPath:\n{}",serde_json::to_string_pretty(&ConfigPath::AppConfig(None)).unwrap());
        println!("serializing JSON style ConfigEnum:\n{}",serde_json::to_string_pretty(&ConfigEnum::AppConfig(AppConfig::default())).unwrap());
    }
}