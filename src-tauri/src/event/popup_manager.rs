use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Popup {
    pub timeout: u32,
    pub stamp: String,
    pub component: String,
    pub props: String,
}

pub fn get_timestamp() -> u128 {
    use std::time;
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Serialize)]
pub struct DefaultPopupProps {
    pub message: String,
    pub title: Option<String>,
}
