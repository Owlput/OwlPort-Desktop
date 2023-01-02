#[macro_export]
macro_rules! impl_config {
    ($($name:ident:$path:expr),+) => {
        #[derive(Debug,Serialize, Deserialize)]
        pub enum ConfigPath {
            $($name(Option<String>),)*
        }
        enum ConfigType {
            $($name,)*
        }

        #[derive(Debug, PartialEq, Eq, Serialize,Deserialize)]
        pub enum ConfigEnum {
            $($name($name),)*
        }
        fn match_config(path: ConfigPath,config:Option<ConfigEnum>) -> (String, ConfigType, Option<Result<Vec<u8>,toml::ser::Error>>) {
            let (path,config_type) = match path {
                $(
                ConfigPath::$name(path) => match path {
                    Some(path) => (path, ConfigType::$name),
                    None => ($path.into(), ConfigType::$name),
                },)*
            };
            match config{
                Some(config_enum)=>{
                    match config_enum{
                        $(ConfigEnum::$name(conf) => (path,config_type,Some(toml::to_vec(&conf))),)*
                    }
                }
                None => (path,config_type,None),
            }
        }
        fn matched_deserialize(config_type:ConfigType,bytes:Vec<u8>)->Result<ConfigEnum, Error>{
            match config_type{
                $(  ConfigType::$name => match toml::from_slice(&bytes) {
                    Ok(val) => Ok(ConfigEnum::$name(val)),
                    Err(e) => Err(Error::Deserialize(e)),
                },)*
            }
        }
    };
}
