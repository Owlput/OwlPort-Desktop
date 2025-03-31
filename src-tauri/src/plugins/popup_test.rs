use super::*;
use crate::event::popup_manager::{DefaultPopupProps, Popup};

macro_rules! get_timestamp {
    () => {{
        use std::time;
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }};
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("popup-tester")
        .invoke_handler(tauri::generate_handler![emit_test_event])
        .build()
}

#[tauri::command]
async fn emit_test_event<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    let _ = app.emit(
        "newPopup",
        Popup {
            timeout: 10000,
            stamp: format!("test-popup-{}", get_timestamp!()),
            component: "DefaultPopup".into(),
            props: serde_json::to_string(&DefaultPopupProps {
                message: "This is a test message from backend".into(),
                title: Some("Popup tester".into()),
            })
            .unwrap(),
        },
    );
    Ok(())
}
