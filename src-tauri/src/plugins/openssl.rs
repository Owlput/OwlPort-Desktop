use openssl::rsa::Rsa;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use tokio::io::AsyncWriteExt;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("openssl_binding")
        .invoke_handler(tauri::generate_handler![generate_rsa_private_key])
        .build()
}

/// Expecting whitespace separated parameters: `<length> [filename]`
#[tauri::command]
async fn generate_rsa_private_key(length:u32,filename:Option<String>) -> Result<String, String> {
    let keypair = match Rsa::generate(length) {
        Ok(keypair) => keypair,
        Err(e) => return Err(format!("error.plugin.openssl.privateKey.generation: {:?}", e)),
    };
    let pem = keypair.private_key_to_pem().unwrap();
    let filepath = match filename {
        Some(v) => format!("certs/{}.pem", v),
        None => return Ok(String::from_utf8(pem).unwrap()),
    };
    let e = match tokio::fs::File::open(&filepath).await {
        Ok(_) => return Err("error.io.file.open.alreadyExist".into()),
        Err(e) => e,
    };
    if e.kind() == std::io::ErrorKind::NotFound {
        match tokio::fs::File::create(format!("certs")).await {
            Ok(mut handle) => match handle.write_all(&pem).await {
                Ok(_) => Ok(format!(
                    "ok.plugin.openssl.privateKey.write: {}",
                    filepath
                )),
                Err(e) => Err(format!(
                    "error.io.file.write: {}, {} |{}",
                    // always spit out generated key when this fails and display to user when needed.
                    e,
                    filepath,
                    String::from_utf8(pem).unwrap()
                )),
            },
            Err(e) => Err(format!("error.io.file.touch: {}", e)),
        }
    } else {
        Err(format!("error.io.file.unknown: {:#?}", e))
    }
}
