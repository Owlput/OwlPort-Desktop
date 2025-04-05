export class ConnectionInfo {
  connection_id: number;
  remote_address: string;
  constructor(connection_id:number, remote_address:string){
    this.connection_id = connection_id;
    this.remote_address = remote_address;
  }
}

export class PeerInfo {
  supported_protocols: Array<string>;
  protocol_version: string;
  connections: Array<ConnectionInfo>;
  constructor(supported_protocols: Array<string>, protocol_version: string, connections: Array<{connection_id:number, remote_address:string}>){
    this.supported_protocols = supported_protocols;
    this.protocol_version = protocol_version;
    this.connections = connections.map((v)=>new ConnectionInfo(v.connection_id, v.remote_address))
  }
}

export class SwarmEmit {
  ConnectionEstablished:
    | {
        peer_id: string;
        endpoint: ConnectedPoint;
      }
    | undefined = undefined;
  ConnectionClosed:
    | {
        peer_id: string;
        endpoint: ConnectedPoint;
        cause: string;
        num_established: Number;
      }
    | undefined = undefined;
  Dialing:
    | {
        maybe_peer_id: string | null;
      }
    | undefined = undefined;
  IncomingConnection:
    | {
        local_addr: string;
        send_back_addr: string;
      }
    | undefined = undefined;
  NewListenAddr:
    | {
        address: string;
      }
    | undefined = undefined;
  OutgoingConnectionError:
    | {
        error: DialError;
      }
    | undefined = undefined;
}

export class ConnectedPoint {
  Dialer:
    | {
        address: string;
        role_override: boolean;
      }
    | undefined = undefined;
  Listener:
    | {
        local_addr: string;
        send_back_addr: string;
      }
    | undefined = undefined;
}

export class DialError {
  LocalPeerId:
    | {
        endpoint: ConnectedPoint;
      }
    | undefined = undefined;
  NoAddresses: {} | undefined = undefined;
  DialPeerConditionFalse: {} | undefined = undefined;
  Aborted: {} | undefined = undefined;
  WrongPeerId: { obtained: string; endpoint: ConnectedPoint } | undefined =
    undefined;
  Denied: string | undefined = undefined;
  Transport: Array<Array<string>> | undefined = undefined;
}

export interface RelayInfo {
  listenable_address: Array<string>;
  listened_address: Array<string>;
  supports_ext: Boolean;
  latency: Number;
  advertised: Array<string> | null;
}

export interface RelayStub {
  peer_id: string;
  latency: Number;
}
