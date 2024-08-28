export function isBodylessHandler(e) {
  let is_tauri_error =
    e.message.includes("__TAURI_IPC__ is not a function");
  if (is_tauri_error && import.meta.env.VITE_WITH_RUST) {
    console.error(`ERR ${e.message}\nPlease check your tauri setup.`);
    return false;
  }
  if (!is_tauri_error) {
    console.error(`ERR ${e.message}`);
  }
  return import.meta.env.VITE_WITH_RUST ? false : true;
}
