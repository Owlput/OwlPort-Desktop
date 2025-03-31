export function isBodylessHandler(e: Error): Boolean {
  let is_tauri_error = e?.message?.includes("__TAURI_IPC__ is not a function");
  if (is_tauri_error && import.meta.env.VITE_WITH_RUST) {
    console.error(`ERR ${e.message}\nPlease check your tauri setup.`);
    return false;
  }
  if (!is_tauri_error) {
    console.error(`ERR ${e.message ? e.message : JSON.stringify(e)} from ${e.stack?.split("src")[1].split(".vue")[0]}`);
  }
  return isBodyless();
}

export function isBodyless(): Boolean {
  return !import.meta.env.VITE_WITH_RUST;
}

export enum Status {
  PendingInit = 0,
  NoBackend = -1,
  OK = 1,
  Error = 2
}

export function is_ip_private(address: String): Boolean {
  const privateIpv4Regex =
    /(10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(1[6-9]|2[0-9]|3[0-1])\.\d{1,3}\.\d{1,3}|192\.168\.\d{1,3}\.\d{1,3})|127\.\d{1,3}\.\d{1,3}\.\d{1,3}|169\.254\.\d{1,3}\.\d{1,3}/;
  const privateIpv6Regex = /(fc|fd)[0-9a-fA-F]{0,2}(:[0-9a-fA-F]{0,4}){0,7}/;
  return (
    privateIpv4Regex.test(address.valueOf()) ||
    privateIpv6Regex.test(address.valueOf())
  );
}

// https://stackoverflow.com/questions/4460586/javascript-regular-expression-to-check-for-ip-addresses
export const REG_VALIDATE_IPV4 =
  /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
export const REG_VALIDATE_IPV6 =
  /^(?:[a-fA-F0-9]{1,4}:){7}[a-fA-F0-9]{1,4}$|^((?:[a-fA-F0-9]{1,4}:){0,5}[a-fA-F0-9]{1,4})?::((?:[a-fA-F0-9]{1,4}:){0,5}[a-fA-F0-9]{1,4})?$/;
