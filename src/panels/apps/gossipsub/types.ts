export interface TopicMessage {
  topic_hash: String;
  topic_literal: String | null;
  source: String | null;
  verified: Boolean;
  trusted: Boolean;
  message: String;
}

export class TopicHash {
  hash: String;
  constructor(hash: String) {
    this.hash = hash;
  }
}

export class ReadableTopic {
  HashOnly: String | undefined = undefined;
  StringOnly: String | undefined = undefined;
  Both:
    | {
        hash: { hash: String };
        string: String;
      }
    | undefined = undefined;
  constructor(value: any) {
    if (value.Both) this.Both = value.Both;
    if (value.HashOnly) this.HashOnly = value.HashOnly;
    if (value.StringOnly) this.StringOnly = value.StringOnly;
  }

  get_hash() {
    if (this.StringOnly) return null;
    if (this.Both) return this.Both.hash.hash;
    return this.HashOnly;
  }
  get_string() {
    if (this.HashOnly) return null;
    if (this.Both) return this.Both.string;
    return this.StringOnly;
  }
}

export interface TopicInfo {
  hash: String;
  topic_literal: String | null;
  first_seen: String | null;
  last_seen: String;
  subscription_time: Number | null;
  number_of_message: Number;
}

export class GossipsubEmit {
  Message:
    | undefined
    | {
        propagation_source: String;
        message_id: Array<Number>;
        message: Message;
      };
}

export class Message {
  source: null | string = null;
  data: Uint8Array;
  sequence_number: null | number = null;
  topic: TopicHash;

  constructor(
    data: Array<number>,
    topic: TopicHash,
    sequence_number?: number | null,
    source?: string | null
  ) {
    this.source = source as string | null;
    this.sequence_number = sequence_number as number | null;
    this.data = new Uint8Array(data);
    this.topic = topic;
  }
}
