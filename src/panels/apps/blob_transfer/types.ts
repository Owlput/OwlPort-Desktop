export class PendingRecv extends Object {
  source_peer: String;
  file_name: String;
  recv_id: Number;
  bytes_total: Number;

  constructor(
    source_peer: String,
    file_name: String,
    recv_id: Number,
    bytes_total: Number
  ) {
    super();
    this.source_peer = source_peer;
    this.file_name = file_name;
    this.recv_id = recv_id;
    this.bytes_total = bytes_total;
  }
}
export class PendingSend extends Object {
  send_id: Number;
  file_path: String;
  destination: String;

  constructor(send_id: Number, file_path: String, dest: String) {
    super();
    this.send_id = send_id;
    this.file_path = file_path;
    this.destination = dest;
  }
}

export interface RsSendInfo {
  local_send_id: Number;
  bytes_sent: Number;
  bytes_total: Number;
  started: boolean;
  remote: String;
  file_path: String;
}

export interface RsRecvInfo {
  local_recv_id: Number;
  bytes_total: Number;
  bytes_received: Number;
  file_name: String;
  remote: String;
  timestamp: BigInt;
}

export class BlobTransferEmit {
  IncomingFile:
    | {
        from: string;
        file_name: string;
        local_recv_id: number;
        bytes_total: number;
      }
    | undefined = undefined;
  RecvProgressed:
    | {
        local_recv_id: number;
        bytes_received: number;
        bytes_total: number;
      }
    | undefined = undefined;
  SendProgressed:
    | {
        local_send_id: number;
        bytes_sent: number;
        bytes_total: number;
      }
    | undefined = undefined;
  OngoingRecvError:
    | {
        local_recv_id: number;
        error: string;
      }
    | undefined = undefined;
  OngoingSendError:
    | {
        local_send_id: number;
        error: String;
      }
    | undefined = undefined;
  CancelledSend:
    | {
        local_send_id: number;
      }
    | undefined = undefined;
  CancelledRecv:
    | {
        local_recv_id: number;
      }
    | undefined = undefined;
  Error: String | undefined = undefined;
}
