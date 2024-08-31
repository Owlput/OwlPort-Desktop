export interface TopicMessage {
  topic_hash: String;
  topic_literal: String | null;
  source_peer: String;
  verified: Boolean;
  trusted: Boolean;
  message: String;
}

export interface Topic {
  hash: String;
  topic_literal: String | null;
  first_seen: String | null;
  last_seen: String;
}
