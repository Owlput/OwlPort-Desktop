export class ConnectionInfo {
  connection_id: number;
  remote_address: string;
  constructor(connection_id: number, remote_address: string) {
    this.connection_id = connection_id;
    this.remote_address = remote_address;
  }
}

export class PeerInfo {
  supported_protocols: Array<Protocol>;
  protocol_version: string;
  connections: Array<ConnectionInfo>;
  constructor(
    supported_protocols: Array<string>,
    protocol_version: string,
    connections: Array<{ connection_id: number; remote_address: string }>
  ) {
    this.protocol_version = protocol_version;
    this.connections = connections.map(
      (v) => new ConnectionInfo(v.connection_id, v.remote_address)
    );
    let supported_protocols_parsed: Array<Protocol> = [];
    for (const proto of supported_protocols) {
      let parsed_proto = new Protocol(proto);
      let same_proto = supported_protocols_parsed.filter((v) =>
        parsed_proto.eq(v)
      );
      if (same_proto[0]) {
        if (!parsed_proto.sub_protocols) continue;
        if (!same_proto[0].sub_protocols) same_proto[0].sub_protocols = [];
        same_proto[0].sub_protocols = same_proto[0].sub_protocols.concat(
          parsed_proto.sub_protocols
        );
        continue;
      }
      supported_protocols_parsed.push(parsed_proto);
    }
    this.supported_protocols = supported_protocols_parsed;
  }
}

export class Protocol {
  name: string;
  major_version: number = 0;
  minor_version: number = 0;
  patch_version: number = 0;
  sub_protocols: Array<string> | null = null;
  description: {
    alias: string | null;
    desc: string;
    remote_ref: string | null;
  } | null = null;

  private static SEMVER_REGEX = /\d+\.\d+\.\d+\b/;

  constructor(proto: string) {
    let semver = Protocol.SEMVER_REGEX.exec(proto);
    if (!semver) {
      this.name = proto;
      return;
    }
    let components = proto.split(semver[0]);
    let name = components[0];
    this.name = name.slice(0, name.length - 1);
    let versions = semver[0].split(".");
    this.major_version = Number.parseInt(versions[0]);
    this.minor_version = Number.parseInt(versions[1]);
    this.patch_version = Number.parseInt(versions[2]);
    this.parse_known_proto(this.name);
    if (!components[1]) return;
    this.sub_protocols = [components[1]];
  }
  private parse_known_proto(proto_name: string) {
    switch (proto_name) {
      case "/ipfs/ping":
        this.description = {
          alias: "IPFS Ping",
          desc: "Sends pings to all connected peers to test for latencey.",
          remote_ref:
            "https://github.com/libp2p/specs/blob/master/ping/ping.md",
        };
        break;
      case "/ipfs/id":
        this.description = {
          alias: "IPFS Identify",
          desc: "Exchange identity information between peers.",
          remote_ref:
            "https://github.com/libp2p/specs/blob/master/identify/README.md",
        };
        break;
        case "/ipfs/id/push":
        this.description = {
          alias: "IPFS Identify-Push",
          desc: "Push changes on peer at runtime.",
          remote_ref:
            "https://github.com/libp2p/specs/blob/master/identify/README.md",
        };
        break;
      case "/libp2p/autonat":
        this.description = {
          alias: "Libp2p AutoNAT",
          desc: "Send pings to a configured set of peers to test for reachability on the Internet.",
          remote_ref:
            "https://github.com/libp2p/specs/blob/master/autonat/autonat-v1.md",
        };
        break;
      case "/meshsub":
        this.description = {
          alias: "Libp2p Gossipsub",
          desc: "Receive and share message broadcasts under topics of interest.",
          remote_ref:
            "https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/gossipsub-v1.2.md",
        };
        break;
      case "/libp2p/circuit/relay":
        this.description = {
          alias: "Libp2p Relay",
          desc: "Provide relayed connection to improve reachability.",
          remote_ref:
            "https://github.com/libp2p/specs/blob/master/relay/circuit-v2.md",
        };
        break;
      case "/owlnest/messaging":
        this.description = {
          alias: "OwlNest Messaging",
          desc: "Direct messaging between peers.",
          remote_ref: null,
        };
        break;
      case "/owlnest/blob":
        this.description = {
          alias: "OwlNest Blob Transfer",
          desc: "Direct transfer of large amount of binary data.",
          remote_ref: null,
        };
        break;
      case "/owlnest/advertise":
        this.description = {
          alias: "OwlNest Advertise",
          desc: "Show advertisement of peers.",
          remote_ref: null,
        };
    }
  }
  eq(other: Protocol): boolean {
    return (
      this.name === other.name &&
      this.major_version === other.major_version &&
      this.minor_version === other.minor_version &&
      this.patch_version === other.patch_version
    );
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
