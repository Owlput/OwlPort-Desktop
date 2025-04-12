use super::*;
use owlnest::net::p2p::protocols::gossipsub::*;
use store::{MessageRecord, TopicStore};
use tauri::{Emitter, EventTarget};
use tracing::warn;

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    Builder::new("owlnest-gossipsub")
        .setup(move |app, _api| {
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(ev)) = ev.as_ref()
                    {
                        use OutEvent::*;
                        match ev {
                            Message { message, .. } => {
                                if let Err(e) = app_handle
                                    .emit_to::<EventTarget, serde_types::GossipsubEmit>(
                                        EventTarget::labeled("owlnest-gossipsub"),
                                        "owlnest-gossipsub-emit",
                                        ev.try_into().unwrap(),
                                    )
                                {
                                    warn!("{:?}", e)
                                };
                                println!("new message {:?}", message);
                                let _ = &manager.gossipsub().message_store().insert_message(
                                    &message.topic,
                                    MessageRecord::Remote(message.clone()),
                                );
                            }
                            Subscribed { peer_id, topic } => {
                                manager.gossipsub().topic_store().join_topic(peer_id, topic);
                            }
                            Unsubscribed { peer_id, topic } => {
                                manager
                                    .gossipsub()
                                    .topic_store()
                                    .leave_topic(peer_id, topic);
                            }
                            _ => {}
                        }
                        continue;
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(generate_handler![
            publish_message,
            spawn_window,
            get_message_history,
            get_all_topics,
            list_participants,
            subscribe,
            unsubscribe,
            subscribed_topics,
            mesh_peers,
            insert_topic_hash_map,
            try_map_topic_hash,
            try_map_string_to_hash,
            clear_message,
        ])
        .build()
}

/// Publish the message. Text messages will be transformed into bytes.
#[tauri::command]
async fn publish_message(
    state: tauri::State<'_, swarm::Manager>,
    topic: Topic,
    message: String,
) -> Result<(), String> {
    let handle = state.gossipsub();
    let topic_hash: TopicHash = topic.get_hash().into();
    if let Err(e) = handle
        .publish_message(&topic_hash, message.clone().into_bytes().into_boxed_slice())
        .await
    {
        return Err(format!(
            "Failed to publish message on {}: {}",
            topic_hash, e
        ));
    }
    Ok(())
}

/// List all messages received about the given topic, including message from self.
#[tauri::command]
async fn get_message_history(
    state: tauri::State<'_, swarm::Manager>,
    topic: Topic,
) -> Result<Option<Box<[serde_types::MessageRecord]>>, String> {
    Ok(state
        .gossipsub()
        .message_store()
        .get_messages(&topic.get_hash().into())
        .map(|i| i.into_vec().into_iter().map(Into::into).collect()))
}

/// List all topics, whether readable or not.
#[tauri::command]
async fn get_all_topics(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Box<[TopicRecord]>, String> {
    Ok(state
        .gossipsub()
        .topic_store()
        .readable_topics()
        .to_vec()
        .into_iter()
        .map(|(hash, topic_string)| TopicRecord::WithString {
            topic_hash: hash.into(),
            topic_string,
        })
        .chain(
            state
                .gossipsub()
                .topic_store()
                .hash_topics()
                .to_vec()
                .into_iter()
                .map(|hash| hash.into()),
        )
        .collect())
}

/// List all participants of the given topic in peer ID.
#[tauri::command]
async fn list_participants(
    state: tauri::State<'_, swarm::Manager>,
    topic: Topic,
) -> Result<Option<Box<[PeerId]>>, String> {
    match state
        .gossipsub()
        .topic_store()
        .participants(&topic.get_hash().into())
    {
        Some(list) => Ok(Some(list.iter().copied().map(|msg| msg).collect())),
        None => Ok(None),
    }
}

/// Subscribe to
#[tauri::command]
async fn subscribe(state: tauri::State<'_, swarm::Manager>, topic: Topic) -> Result<bool, String> {
    let topic_hash = topic.get_hash().into();
    if topic.get_string().is_some() {
        state
            .gossipsub()
            .topic_store()
            .insert_topic(topic.get_string().unwrap().clone(), &topic_hash);
    } else {
        state.gossipsub().topic_store().insert_hash(&topic_hash);
    }
    let result = state
        .gossipsub()
        .subscribe_topic_hash(&topic_hash.clone())
        .await
        .map_err(|e| e.to_string());
    if let Ok(true) = result {
        state
            .gossipsub()
            .topic_store()
            .subscribe_topic(&topic_hash, None);
    }
    result
}

#[tauri::command]
async fn unsubscribe(
    state: tauri::State<'_, swarm::Manager>,
    topic: Topic,
) -> Result<bool, String> {
    let topic_hash = topic.get_hash();
    Ok(state
        .gossipsub()
        .unsubscribe_topic_hash(&topic_hash.into())
        .await)
}

#[tauri::command]
async fn subscribed_topics(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Box<[TopicRecord]>, String> {
    Ok(state
        .gossipsub()
        .topic_store()
        .subscribed_topics()
        .to_vec()
        .into_iter()
        .map(|hash| TopicRecord::from_mapped(hash, state.gossipsub().topic_store()))
        .collect())
}

#[tauri::command]
async fn mesh_peers(
    state: tauri::State<'_, swarm::Manager>,
    topic: Topic,
) -> Result<Box<[PeerId]>, String> {
    let topic_hash = topic.get_hash();
    return Ok(state
        .gossipsub()
        .mesh_peers_of_topic(&topic_hash.into())
        .await);
}

#[tauri::command]
async fn insert_topic_hash_map(
    state: tauri::State<'_, swarm::Manager>,
    topic: Topic,
) -> Result<bool, String> {
    if let Topic::StringOnly {
        topic_string,
        hash_type,
    } = topic
    {
        return Ok(state
            .gossipsub()
            .topic_store()
            .insert_topic(topic_string.clone(), &hash_type.hash(topic_string)));
    }
    Err("Cannot insert record without topic string.".into())
}

#[tauri::command]
async fn clear_message(
    state: tauri::State<'_, swarm::Manager>,
    topic: Option<Topic>,
) -> Result<(), String> {
    state
        .gossipsub()
        .message_store()
        .clear_message(topic.map(|topic| topic.get_hash().into()).as_ref());
    Ok(())
}

#[tauri::command]
async fn try_map_topic_hash(
    state: tauri::State<'_, swarm::Manager>,
    topic: serde_types::TopicHash,
) -> Result<Option<String>, String> {
    Ok(state.gossipsub().topic_store().try_map(&topic.into()))
}
#[tauri::command]
async fn try_map_string_to_hash(
    topic_string: String,
    hash_type: HashType,
) -> Result<serde_types::TopicHash, String> {
    Ok(hash_type.hash(topic_string).into())
}

#[allow(unused)]
#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, swarm::Manager>,
    peer: Option<PeerId>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("owlnest-gossipsub") {
        let _ = window.set_focus();
        return Ok(());
    }
    tauri::WebviewWindowBuilder::new(
        &app,
        "owlnest-gossipsub",
        tauri::WebviewUrl::App("#/app/gossipsub".into()),
    )
    .focused(true)
    .title("Owlnest - GossipSub")
    .build()
    .expect("New window to be created successfully");

    Ok(())
}

mod serde_types {
    use derive_more::From;
    pub use gossipsub::serde_types::TopicHash;
    use libp2p::{gossipsub::MessageId, PeerId};
    use owlnest::net::p2p::protocols::gossipsub;
    use serde::{Deserialize, Serialize};
    use std::convert::Infallible;

    /// The message sent to the user after a [`RawMessage`] has been transformed by a
    /// [`crate::DataTransform`].
    #[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
    pub struct Message {
        /// Id of the peer that published this message.
        pub source: Option<PeerId>,

        /// Content of the message.
        pub data: Vec<u8>,

        /// A random sequence number.
        pub sequence_number: Option<u64>,

        /// The topic this message belongs to
        pub topic: TopicHash,
    }
    impl From<gossipsub::Message> for Message {
        fn from(value: gossipsub::Message) -> Self {
            let gossipsub::Message {
                source,
                data,
                sequence_number,
                topic,
            } = value;
            Self {
                source,
                data,
                sequence_number,
                topic: topic.into(),
            }
        }
    }

    /// The kind of message.
    #[derive(Debug, Clone, Serialize)]
    pub enum MessageRecord {
        /// Message published by a remote node.
        Remote(Message),
        /// Message published by local node.
        Local(Box<[u8]>),
    }
    impl From<super::MessageRecord> for MessageRecord {
        fn from(value: super::MessageRecord) -> Self {
            match value {
                super::MessageRecord::Local(bytes) => Self::Local(bytes),
                super::MessageRecord::Remote(msg) => Self::Remote(msg.into()),
            }
        }
    }

    /// Messages that have expired while attempting to be sent to a peer.
    #[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, From)]
    pub struct FailedMessages {
        /// The number of publish messages that failed to be published in a heartbeat.
        pub publish: usize,
        /// The number of forward messages that failed to be published in a heartbeat.
        pub forward: usize,
        /// The number of messages that were failed to be sent to the priority queue as it was full.
        pub priority: usize,
        /// The number of messages that were failed to be sent to the non-priority queue as it was
        /// full.
        pub non_priority: usize,
        /// The number of messages that timed out and could not be sent.
        pub timeout: usize,
    }

    impl From<&libp2p::gossipsub::FailedMessages> for FailedMessages {
        fn from(value: &libp2p::gossipsub::FailedMessages) -> Self {
            let libp2p::gossipsub::FailedMessages {
                publish,
                forward,
                priority,
                non_priority,
                timeout,
            } = *value;
            Self {
                publish,
                forward,
                priority,
                non_priority,
                timeout,
            }
        }
    }

    #[derive(Debug, Clone, Serialize)]
    /// Event that can be emitted by the gossipsub behaviour.
    pub enum GossipsubEmit {
        /// A message has been received.
        Message {
            /// The peer that forwarded us this message.
            propagation_source: PeerId,
            /// The [`MessageId`] of the message. This should be referenced by the application when
            /// validating a message (if required).
            message_id: MessageId,
            /// The decompressed message itself.
            message: Message,
        },
        /// A remote subscribed to a topic.
        Subscribed {
            /// Remote that has subscribed.
            peer_id: PeerId,
            /// The topic it has subscribed to.
            topic: TopicHash,
        },
        /// A remote unsubscribed from a topic.
        Unsubscribed {
            /// Remote that has unsubscribed.
            peer_id: PeerId,
            /// The topic it has subscribed from.
            topic: TopicHash,
        },
        /// A peer that does not support gossipsub has connected.
        GossipsubNotSupported { peer_id: PeerId },
        SlowPeer {
            /// The peer_id
            peer_id: PeerId,
            /// The types and amounts of failed messages that are occurring for this peer.
            failed_messages: FailedMessages,
        },
    }

    impl TryFrom<&super::OutEvent> for GossipsubEmit {
        type Error = Infallible;
        fn try_from(value: &super::OutEvent) -> Result<Self, Self::Error> {
            use super::OutEvent::*;
            let ev = match value {
                Message {
                    propagation_source,
                    message_id,
                    message,
                } => Self::Message {
                    propagation_source: *propagation_source,
                    message_id: message_id.clone(),
                    message: message.clone().into(),
                },
                Subscribed { peer_id, topic } => Self::Subscribed {
                    peer_id: *peer_id,
                    topic: topic.clone().into(),
                },
                Unsubscribed { peer_id, topic } => Self::Unsubscribed {
                    peer_id: *peer_id,
                    topic: topic.clone().into(),
                },
                GossipsubNotSupported { peer_id } => {
                    Self::GossipsubNotSupported { peer_id: *peer_id }
                }
                SlowPeer {
                    peer_id,
                    failed_messages,
                } => Self::SlowPeer {
                    peer_id: *peer_id,
                    failed_messages: failed_messages.into(),
                },
            };
            Ok(ev)
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Topic {
    HashOnly {
        hash: serde_types::TopicHash,
    },
    StringOnly {
        topic_string: String,
        hash_type: HashType,
    },
}
impl Topic {
    pub fn get_hash(&self) -> serde_types::TopicHash {
        match self {
            Self::HashOnly { hash } => hash.clone(),
            Self::StringOnly {
                topic_string,
                hash_type,
            } => hash_type.hash(topic_string.clone()).into(),
        }
    }
    pub fn get_string(&self) -> Option<&String> {
        match self {
            Self::HashOnly { .. } => None,
            Self::StringOnly { topic_string, .. } => Some(topic_string),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopicRecord {
    HashOnly {
        hash: serde_types::TopicHash,
    },
    WithString {
        topic_hash: serde_types::TopicHash,
        topic_string: String,
    },
}
impl TopicRecord {
    pub fn from_mapped<S: TopicStore + ?Sized>(hash: TopicHash, store: &Box<S>) -> Self {
        if let Some(string) = store.as_ref().try_map(&hash) {
            return TopicRecord::WithString {
                topic_hash: hash.into(),
                topic_string: string,
            };
        }
        TopicRecord::HashOnly { hash: hash.into() }
    }
}
impl From<TopicHash> for TopicRecord {
    fn from(value: TopicHash) -> Self {
        Self::HashOnly { hash: value.into() }
    }
}
