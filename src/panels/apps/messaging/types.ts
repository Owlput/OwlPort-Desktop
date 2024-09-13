export class MessagingEmit {
  IncomingMessage:
    | {
        from: String;
        msg: Message;
      }
    | undefined = undefined;
  InboundNegotiated:
    | {
        peer: Message;
      }
    | undefined = undefined;
  OutboundNegotiated:
    | {
        peer: String;
      }
    | undefined = undefined;
  Disconnected:
    | {
        peer: String;
      }
    | undefined = undefined;
  Connected:
    | {
        peer: String;
      }
    | undefined = undefined;
}

export interface Message {
  time: Number;
  from: String;
  to: String;
  msg: String;
}
