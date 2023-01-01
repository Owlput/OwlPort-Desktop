use serde::{Deserialize, Serialize};

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
pub fn write_config<T: Serialize>(config: T, path: ConfigPath) -> Result<(), String> {
    match write_to_file(&config, path) {
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

// TODO: utilize macros for automatically generate corresponding
// implementations for each config file 
#[derive(Debug, Deserialize)]
pub enum ConfigPath {
    NestConfig(Option<String>),
}
enum ConfigType {
    NestConfig,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ConfigEnum {
    NestConfig(NestConfig),
}

/// Read config from a specified path, or from the default location if no path is specified.
pub fn read_from_file(path: ConfigPath) -> Result<ConfigEnum, Error> {
    let (path, config_type) = match_config_path(path);
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => return Err(Error::Io(e)),
    };
    match config_type {
        ConfigType::NestConfig => match toml::from_slice(&bytes) {
            Ok(val) => Ok(ConfigEnum::NestConfig(val)),
            Err(e) => Err(Error::Deserialize(e)),
        },
    }
}
/// Write the config to a specified path, or to the default location if no path is specified.
/// This will overwrite the entire file if it exists, or create a new one when not.
pub fn write_to_file<T: Serialize>(config: &T, path: ConfigPath) -> Result<(), Error> {
    let (path, _) = match_config_path(path);
    let bytes = match toml::to_vec(config) {
        Ok(bytes) => bytes,
        Err(e) => return Err(Error::Serialize(e)),
    };
    match std::fs::write(path, bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Io(e)),
    }
}

pub mod nest_config {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct NestConfig {
        pub address: String,
        #[serde(default)]
        pub cert_path: Option<String>,
    }
    impl Default for NestConfig {
        fn default() -> Self {
            Self {
                address: "http://127.0.0.1:20001".into(),
                cert_path: None,
            }
        }
    }
}
pub mod app_config {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct AppConfig {
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
            read_from_file(ConfigPath::NestConfig(None)).unwrap(),
            ConfigEnum::NestConfig(NestConfig {
                address: r#"http://127.0.0.1:20001"#.into(),
                cert_path: None
            })
        );
    }
}

fn match_config_path(path: ConfigPath) -> (String, ConfigType) {
    match path {
        ConfigPath::NestConfig(path) => match path {
            Some(path) => (path, ConfigType::NestConfig),
            None => ("conf/nest_config.toml".into(), ConfigType::NestConfig),
        },
    }
}
