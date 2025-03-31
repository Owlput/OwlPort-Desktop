export interface TopicMessage {
  topic_hash: String;
  topic_literal: String | null;
  source: String | null;
  verified: Boolean;
  trusted: Boolean;
  message: String;
}

export class TopicHash {
  hash: string;
  constructor(hash: string) {
    this.hash = hash;
  }
  includes(other: string):boolean{
    return this.hash.includes(other)
  }
}

export class TopicRecord {
  HashOnly: { hash: TopicHash } | undefined = undefined;
  WithString: { topic_hash: TopicHash; topic_string: string } | undefined =
    undefined;
  constructor(value: any) {
    if (value.HashOnly){
      this.HashOnly = { hash: new TopicHash(value.HashOnly.hash.hash) };
    }
    if (value.WithString) this.WithString = value.WithString;
  }

  get_hash() {
    if (this.WithString) return this.WithString.topic_hash;
    return this.HashOnly!.hash;
  }
  get_string() {
    if (this.WithString) return this.WithString.topic_string;
    return null;
  }
}

export interface TopicInfo {
  hash: string;
  topic_literal: string | null;
  first_seen: string | null;
  last_seen: string;
  subscription_time: Number | null;
  number_of_message: Number;
}

export class GossipsubEmit {
  Message:
    | undefined
    | {
        propagation_source: string;
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

export class Topic {
  HashOnly: { hash: TopicHash } | undefined = undefined;
  StringOnly: { topic_string: string; hash_type: string } | undefined =
    undefined;
  private constructor(hash: TopicHash | null, str1?: string, str2?: string) {
    if (hash !== null) {
      this.HashOnly = { hash: hash };
    } else {
      this.StringOnly = { topic_string: str1!, hash_type: str2! };
    }
  }
  static hash_only(hash: TopicHash) {
    return new Topic(hash);
  }
  static identity_string(topic_string: string) {
    return new Topic(null, topic_string, "Identity");
  }
  static sha256_string(topic_string: string) {
    return new Topic(null, topic_string, "Sha256");
  }
}

export class MessageRecord {
  Local: null | Uint8Array = null;
  Remote: null | Message = null;
  private constructor(
    data: Array<number>,
    topic?: string,
    sequence_number?: number,
    source?: string
  ) {
    if (!topic) {
      this.Local = new Uint8Array(data);
      return;
    }
    this.Remote = new Message(
      data,
      new TopicHash(topic!),
      sequence_number!,
      source!
    );
  }

  static deserialize(value: any) {
    if (value.Local) {
      return new MessageRecord(value.Local);
    }
    if (value.Remote) {
      return new MessageRecord(
        value.Remote.data,
        value.Remote.topic,
        value.Remote.sequence_number,
        value.Remote.source
      );
    }
    throw new Error("Cannot deserialize");
  }
}
