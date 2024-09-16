use super::*;
use owlnest::net::p2p::protocols::gossipsub::*;
use tracing::warn;

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    Builder::new("libp2p-gossipsub")
        .setup(move |app| {
            let app_handle = app.clone();
            async_runtime::spawn(async move {
                let mut listener = manager.event_subscriber().subscribe();
                while let Ok(ev) = listener.recv().await {
                    if let swarm::SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(ev)) = ev.as_ref()
                    {
                        use OutEvent::*;
                        match ev {
                            Message { message, .. } => {
                                if let Err(e) = app_handle.emit_to::<serde_types::GossipsubEmit>(
                                    "GossipSub",
                                    "libp2p-gossipsub-emit",
                                    ev.try_into().unwrap(),
                                ) {
                                    warn!("{:?}", e)
                                };
                                let _ = manager
                                    .gossipsub()
                                    .message_store
                                    .insert_message(message.clone());
                            }
                            Subscribed { peer_id, topic } => {
                                manager.gossipsub().topic_store.join_topic(peer_id, topic);
                            }
                            Unsubscribed { peer_id, topic } => {
                                manager.gossipsub().topic_store.leave_topic(peer_id, topic);
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
    topic: ReadableTopic,
    message: String,
) -> Result<(), String> {
    let handle = state.gossipsub();
    let topic_hash: TopicHash = topic.get_hash().into();
    match handle
        .publish_message(
            topic_hash.clone(),
            message.clone().into_bytes().into_boxed_slice(),
        )
        .await
    {
        Ok(_id) => {
            let _ = handle.message_store.insert_message(Message {
                source: Some(state.identity().get_peer_id()),
                data: message.into_bytes(),
                sequence_number: None,
                topic: topic.get_hash().into(),
            });
            Ok(())
        }
        Err(e) => Err(format!(
            "Failed to publish message on {}: {}",
            topic_hash, e
        )),
    }
}

/// List all messages received about the given topic, including message from self.
#[tauri::command]
async fn get_message_history(
    state: tauri::State<'_, swarm::Manager>,
    topic: ReadableTopic,
) -> Result<Option<Box<[serde_types::Message]>>, String> {
    match state
        .gossipsub()
        .message_store
        .get_messages(&topic.get_hash().into())
    {
        Some(list) => Ok(Some(
            list.to_vec().into_iter().map(|msg| msg.into()).collect(),
        )),
        None => Ok(None),
    }
}

/// List all topics, whether readable or not.
#[tauri::command]
async fn get_all_topics(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Box<[ReadableTopic]>, String> {
    Ok(state
        .gossipsub()
        .topic_store
        .readable_topics()
        .to_vec()
        .into_iter()
        .map(|(hash, string)| ReadableTopic::Both {
            hash: hash.into(),
            string,
        })
        .chain(
            state
                .gossipsub()
                .topic_store
                .hash_topics()
                .to_vec()
                .into_iter()
                .map(|hash| ReadableTopic::HashOnly(hash.into())),
        )
        .collect())
}

/// List all participants of the given topic in peer ID.
#[tauri::command]
async fn list_participants(
    state: tauri::State<'_, swarm::Manager>,
    topic: ReadableTopic,
) -> Result<Option<Box<[PeerId]>>, String> {
    match state
        .gossipsub()
        .topic_store
        .participants(&topic.get_hash().into())
    {
        Some(list) => Ok(Some(
            list.to_vec().into_iter().map(|msg| msg.into()).collect(),
        )),
        None => Ok(None),
    }
}

/// Subscribe to
#[tauri::command]
async fn subscribe(
    state: tauri::State<'_, swarm::Manager>,
    topic: ReadableTopic,
) -> Result<bool, String> {
    if topic.get_string().is_none() {
        return Err("Cannot subscribe without topic string.".into());
    }
    let topic_string = topic.get_string().unwrap();
    state
        .gossipsub()
        .topic_store
        .insert_string(topic_string.clone());
    let result = state
        .gossipsub()
        .subscribe_topic(topic_string)
        .await
        .map_err(|e| e.to_string());
    if let Ok(true) = result {
        state
            .gossipsub()
            .topic_store
            .subscribe_topic(topic_string.clone().into());
    }
    result
}

#[tauri::command]
async fn unsubscribe(
    state: tauri::State<'_, swarm::Manager>,
    topic: ReadableTopic,
) -> Result<bool, String> {
    if let Some(topic_string) = topic.get_string() {
        let result = state
            .gossipsub()
            .unsubscribe_topic(topic_string)
            .await
            .map_err(|e| e.to_string());
        if let Ok(true) = result {
            state
                .gossipsub()
                .topic_store
                .unsubscribe_topic(&Sha256Topic::new(topic_string).hash());
        }
        return result;
    }
    Err("Cannot unsubscribe without topic string.".into())
}

#[tauri::command]
async fn subscribed_topics(
    state: tauri::State<'_, swarm::Manager>,
) -> Result<Box<[ReadableTopic]>, String> {
    Ok(state.gossipsub().topic_store.subscribed_topics())
}

#[tauri::command]
async fn mesh_peers(
    state: tauri::State<'_, swarm::Manager>,
    topic: ReadableTopic,
) -> Result<Box<[PeerId]>, String> {
    if let Some(topic_string) = topic.get_string() {
        return Ok(state.gossipsub().mesh_peers_of_topic(topic_string).await);
    }
    Err("Cannot get mesh peers without topic string.".into())
}

#[tauri::command]
async fn insert_topic_hash_map(
    state: tauri::State<'_, swarm::Manager>,
    topic: ReadableTopic,
) -> Result<(), String> {
    if let Some(topic_string) = topic.get_string() {
        return Ok(state
            .gossipsub()
            .topic_store
            .insert_string(topic_string.clone()));
    }
    Err("Cannot insert record without topic string.".into())
}

#[tauri::command]
async fn clear_message(
    state: tauri::State<'_, swarm::Manager>,
    topic: Option<ReadableTopic>,
) -> Result<(), String> {
    state
        .gossipsub()
        .message_store
        .clear_message(topic.map(|topic| topic.get_hash().into()).as_ref());
    Ok(())
}

#[tauri::command]
async fn try_map_topic_hash(
    state: tauri::State<'_, swarm::Manager>,
    topic: serde_types::TopicHash,
) -> Result<Option<String>, String> {
    Ok(state.gossipsub().topic_store.try_map(&topic.into()))
}
#[tauri::command]
async fn try_map_string_to_hash(topic: String) -> Result<serde_types::TopicHash, String> {
    Ok(Sha256Topic::new(topic).hash().into())
}

#[allow(unused)]
#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, swarm::Manager>,
    peer: Option<PeerId>,
) -> Result<(), String> {
    if let Some(window) = app.get_window("GossipSub") {
        let _ = window.set_focus();
        return Ok(());
    }
    tauri::WindowBuilder::new(
        &app,
        "GossipSub",
        tauri::WindowUrl::App("#/app/gossipsub".into()),
    )
    .focused(true)
    .title("Owlnest - GossipSub")
    .build()
    .expect("New window to be created successfully");

    Ok(())
}

mod serde_types {
    pub use gossipsub::serde_types::TopicHash;
    use libp2p::{gossipsub::MessageId, PeerId};
    use owlnest::net::p2p::protocols::gossipsub;
    use serde::Serialize;
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
            };

            Ok(ev)
        }
    }
}
