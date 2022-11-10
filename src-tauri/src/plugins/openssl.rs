use openssl::rsa::Rsa;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use tokio::io::AsyncWriteExt;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("openssl_binding")
        .invoke_handler(tauri::generate_handler![generate_rsa_keypair])
        .build()
}

/// Expecting whitespace separated parameters: `<length> [filename]`
#[tauri::command]
async fn generate_rsa_keypair(arg: String) -> Result<String, String> {
    let mut args = arg.split_ascii_whitespace();
    let size = match args.next() {
        Some(v) => match v.parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to parse RSA key size: {}", e)),
        },
        None => return Err("RSA key size not supplied".into()),
    };
    let keypair = match Rsa::generate(size) {
        Ok(keypair) => keypair,
        Err(e) => return Err(format!("Failed to generate RSA key: {}", e)),
    };
    let pem = keypair.private_key_to_pem().unwrap();
    let filepath = match args.next() {
        Some(v) => format!("certs/{}.pem", v),
        None => return Ok(String::from_utf8(pem).unwrap()),
    };
    let e = match tokio::fs::File::open(&filepath).await {
        Ok(_) => return Err("File already exist.".into()),
        Err(e) => e,
    };
    if e.kind() == std::io::ErrorKind::NotFound {
        match tokio::fs::File::create(format!("certs")).await {
            Ok(mut handle) => match handle.write_all(&pem).await {
                Ok(_) => Ok(format!(
                    "Successfully saved RSA private key to {}",
                    filepath
                )),
                Err(e) => Err(format!(
                    "Failed to create file {}, caused by {} |{}", 
                    // always spit out generated key when this fails and display to user when needed.
                    filepath,
                    e,
                    String::from_utf8(pem).unwrap()
                )),
            },
            Err(e) => Err(format!("Failed to create file: {}", e)),
        }
    } else {
        Err(format!("I/O Error: {:#?}", e))
    }
}
