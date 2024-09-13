export function isBodylessHandler(e:any):Boolean {
  let is_tauri_error =
    e?.message?.includes("__TAURI_IPC__ is not a function");
  if (is_tauri_error && import.meta.env.VITE_WITH_RUST) {
    console.error(`ERR ${e.message}\nPlease check your tauri setup.`);
    return false;
  }
  if (!is_tauri_error) {
    console.error(`ERR ${e.message?e.message:e}`);
  }
  return isBodyless();
}

export function isBodyless():Boolean{
  return !import.meta.env.VITE_WITH_RUST
}

export enum Status{
  PendingInit = 0,
  NoBackend = -1,
  OK = 1,
}

export function is_ip_private(address:String):Boolean{
  const privateIpv4Regex = /^(10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(1[6-9]|2[0-9]|3[0-1])\.\d{1,3}\.\d{1,3}|192\.168\.\d{1,3}\.\d{1,3})$/;
  const privateIpv6Regex = /^(fc|fd)[0-9a-fA-F]{0,2}(:[0-9a-fA-F]{0,4}){0,7}$/;
  return privateIpv4Regex.test(address.valueOf()) ||privateIpv6Regex.test(address.valueOf())
}