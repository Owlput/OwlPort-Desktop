export interface ConnectionInfo {
  connection_id: Number;
  remote_address: String;
}

export interface PeerInfo {
  supported_protocols: Array<String>;
  protocol_version: String;
}

export class SwarmEmit {
  ConnectionEstablished: {
    peer_id: String;
    endpoint: ConnectedPoint;
  } | undefined = undefined;
  ConnectionClosed: {
    peer_id: String;
    endpoint: ConnectedPoint;
    cause: String;
    num_established: Number;
  } | undefined = undefined;
  Dialing: {
    maybe_peer_id: String | null;
  } | undefined = undefined;
  IncomingConnection: {
    local_addr: String;
    send_back_addr: String;
  } | undefined = undefined;
  NewListenAddr: {
    address: String;
  } | undefined = undefined;
  OutgoingConnectionError: {
    error: DialError;
  } | undefined = undefined;
}

export class ConnectedPoint {
  Dialer: {
    address: String;
    role_override: boolean;
  } | undefined = undefined;
  Listener: {
    local_addr: String;
    send_back_addr: String;
  } | undefined = undefined;
}

export class DialError {
  LocalPeerId: {
    endpoint: ConnectedPoint;
  } | undefined = undefined;
  NoAddresses: {} | undefined = undefined;
  DialPeerConditionFalse: {} | undefined = undefined;
  Aborted: {} | undefined = undefined;
  WrongPeerId: { obtained: String; endpoint: ConnectedPoint } | undefined = undefined;
  Denied: String | undefined = undefined;
  Transport: Array<Array<String>> | undefined = undefined;
}

export interface RelayInfo {
  address: Array<String>;
  listened_address: Array<String>;
  supports_ext: Boolean;
  latency: Number;
  advertised: Array<String> | null;
}
