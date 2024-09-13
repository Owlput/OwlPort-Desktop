use super::*;
use owlnest::net::p2p::protocols::gossipsub::*;
use tracing::warn;

type TopicStore = Box<dyn store::TopicStore + Send + Sync>;
type MessageStore = Box<dyn store::MessageStore + Send + Sync>;

#[derive(Clone)]
struct State {
    pub manager: swarm::Manager,
    pub topic_store: Arc<TopicStore>,
    pub message_store: Arc<MessageStore>,
}
impl State {
    pub fn push_history(&self, msg: Message) {
        let _ = self.message_store.insert_message(msg);
    }
    pub fn on_new_participant(&self, peer_id: &PeerId, topic: &TopicHash) {
        self.topic_store.join_topic(peer_id, topic);
    }
    pub fn on_participant_left(&self, peer_id: &PeerId, topic: &TopicHash) {
        self.topic_store.leave_topic(peer_id, topic);
    }
}

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    let state = State {
        manager: manager.clone(),
        #[cfg(feature = "volatile")]
        message_store: Arc::new(Box::new(store::MemMessageStore::default())),
        #[cfg(feature = "volatile")]
        topic_store: Arc::new(Box::new(store::MemTopicStore::default())),
    };
    Builder::new("libp2p-gossipsub")
        .setup(move |app| {
            let app_handle = app.clone();
            let state_clone = state.clone();
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
                                state.push_history(message.clone());
                            }
                            Subscribed { peer_id, topic } => {
                                state.on_new_participant(peer_id, topic)
                            }
                            Unsubscribed { peer_id, topic } => {
                                state.on_participant_left(peer_id, topic)
                            }
                            _ => {}
                        }
                        continue;
                    }
                }
            });
            app.manage(state_clone);
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
            try_map_string_to_hash
        ])
        .build()
}

/// Publish the message. Text messages will be transformed into bytes.
#[tauri::command]
async fn publish_message(
    state: tauri::State<'_, State>,
    topic: ReadableTopic,
    message: String,
) -> Result<(), String> {
    let manager = &state.manager;
    let topic_hash: TopicHash = topic.get_hash().into();
    match manager
        .gossipsub()
        .publish_message(
            topic_hash.clone(),
            message.clone().into_bytes().into_boxed_slice(),
        )
        .await
    {
        Ok(_id) => {
            let _ = state.push_history(Message {
                source: Some(manager.identity().get_peer_id()),
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
    state: tauri::State<'_, State>,
    topic: ReadableTopic,
) -> Result<Option<Box<[serde_types::Message]>>, String> {
    match state.message_store.get_messages(&topic.get_hash().into()) {
        Some(list) => Ok(Some(
            list.to_vec().into_iter().map(|msg| msg.into()).collect(),
        )),
        None => Ok(None),
    }
}

/// List all topics, whether readable or not.
#[tauri::command]
async fn get_all_topics(state: tauri::State<'_, State>) -> Result<Box<[ReadableTopic]>, String> {
    Ok(state
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
    state: tauri::State<'_, State>,
    topic: ReadableTopic,
) -> Result<Option<Box<[PeerId]>>, String> {
    match state.topic_store.participants(&topic.get_hash().into()) {
        Some(list) => Ok(Some(
            list.to_vec().into_iter().map(|msg| msg.into()).collect(),
        )),
        None => Ok(None),
    }
}

/// Subscribe to
#[tauri::command]
async fn subscribe(state: tauri::State<'_, State>, topic: ReadableTopic) -> Result<bool, String> {
    if topic.get_string().is_none() {
        return Err("Cannot subscribe without topic string.".into());
    }
    let topic_string = topic.get_string().unwrap();
    state.topic_store.insert_string(topic_string.clone());
    let result = state
        .manager
        .gossipsub()
        .subscribe_topic(topic_string)
        .await
        .map_err(|e| e.to_string());
    if let Ok(true) = result {
        state
            .topic_store
            .subscribe_topic(topic_string.clone().into());
    }
    result
}

#[tauri::command]
async fn unsubscribe(state: tauri::State<'_, State>, topic: ReadableTopic) -> Result<bool, String> {
    if let Some(topic_string) = topic.get_string() {
        let result = state
            .manager
            .gossipsub()
            .unsubscribe_topic(topic_string)
            .await
            .map_err(|e| e.to_string());
        if let Ok(true) = result {
            state
                .topic_store
                .unsubscribe_topic(&Sha256Topic::new(topic_string).hash());
        }
        return result;
    }
    Err("Cannot unsubscribe without topic string.".into())
}

#[tauri::command]
async fn subscribed_topics(state: tauri::State<'_, State>) -> Result<Box<[ReadableTopic]>, String> {
    Ok(state.topic_store.subscribed_topics())
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
    state: tauri::State<'_, State>,
    topic: ReadableTopic,
) -> Result<(), String> {
    if let Some(topic_string) = topic.get_string() {
        return Ok(state.topic_store.insert_string(topic_string.clone()));
    }
    Err("Cannot insert record without topic string.".into())
}

#[tauri::command]
async fn try_map_topic_hash(
    state: tauri::State<'_, State>,
    topic: serde_types::TopicHash,
) -> Result<Option<String>, String> {
    Ok(state.topic_store.try_map(&topic.into()))
}
#[tauri::command]
async fn try_map_string_to_hash(
    state: tauri::State<'_, State>,
    topic: String,
) -> Result<serde_types::TopicHash, String> {
    Ok(Sha256Topic::new(topic).hash().into())
}

#[allow(unused)]
#[tauri::command]
async fn spawn_window<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, State>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReadableTopic {
    HashOnly(serde_types::TopicHash),
    StringOnly(String),
    Both {
        hash: serde_types::TopicHash,
        string: String,
    },
}
impl ReadableTopic {
    pub fn get_hash(&self) -> serde_types::TopicHash {
        match self {
            ReadableTopic::HashOnly(hash) => hash.clone(),
            ReadableTopic::StringOnly(string) => Sha256Topic::new(string).hash().into(),
            ReadableTopic::Both { hash, .. } => hash.clone(),
        }
    }
    pub fn get_string(&self) -> Option<&String> {
        match self {
            ReadableTopic::HashOnly(_) => None,
            ReadableTopic::StringOnly(string) => Some(string),
            ReadableTopic::Both { string, .. } => Some(string),
        }
    }
}
impl From<String> for ReadableTopic {
    fn from(value: String) -> Self {
        let topic = Sha256Topic::new(value.clone());
        Self::Both {
            hash: topic.hash().into(),
            string: value,
        }
    }
}
impl From<&str> for ReadableTopic {
    fn from(value: &str) -> Self {
        let topic = Sha256Topic::new(value);
        Self::Both {
            hash: topic.hash().into(),
            string: value.to_owned(),
        }
    }
}
impl From<TopicHash> for ReadableTopic {
    fn from(value: TopicHash) -> Self {
        Self::HashOnly(value.into())
    }
}
impl From<serde_types::TopicHash> for ReadableTopic {
    fn from(value: serde_types::TopicHash) -> Self {
        Self::HashOnly(value)
    }
}

mod serde_types {
    use std::convert::Infallible;

    use libp2p::{gossipsub::MessageId, PeerId};
    use owlnest::net::p2p::protocols::gossipsub;
    use serde::{Deserialize, Serialize};

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
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct TopicHash {
        /// The topic hash. Stored as a string to align with the protobuf API.
        hash: String,
    }
    impl TopicHash {
        pub fn from_raw(hash: impl Into<String>) -> TopicHash {
            TopicHash { hash: hash.into() }
        }

        pub fn into_string(self) -> String {
            self.hash
        }

        pub fn as_str(&self) -> &str {
            &self.hash
        }
    }
    impl From<gossipsub::TopicHash> for TopicHash {
        fn from(value: gossipsub::TopicHash) -> Self {
            Self {
                hash: value.into_string(),
            }
        }
    }
    impl From<TopicHash> for gossipsub::TopicHash {
        fn from(value: TopicHash) -> Self {
            gossipsub::TopicHash::from_raw(value.hash)
        }
    }
}

pub mod store {
    use dashmap::{DashMap, DashSet};
    use libp2p::{
        gossipsub::{Sha256Topic, Message, TopicHash},
        PeerId,
    };
    use owlnest::net::p2p::protocols::gossipsub;

    use super::ReadableTopic;

    pub trait TopicStore {
        fn insert_string(&self, topic_string: String);
        fn insert_hash(&self, topic_hash: TopicHash) -> bool;
        fn try_map(&self, topic_hash: &TopicHash) -> Option<String>;
        fn participants(&self, topic_hash: &TopicHash) -> Option<Box<[PeerId]>>;
        fn join_topic(&self, peer: &PeerId, topic_hash: &TopicHash) -> bool;
        fn leave_topic(&self, peer: &PeerId, topic_hash: &TopicHash) -> bool;
        fn subscribe_topic(&self, topic: ReadableTopic) -> bool;
        fn unsubscribe_topic(&self, topic: &TopicHash) -> bool;
        fn hash_topics(&self) -> Box<[TopicHash]>;
        fn readable_topics(&self) -> Box<[(TopicHash, String)]>;
        fn subscribed_topics(&self) -> Box<[ReadableTopic]>;
    }
    pub trait MessageStore {
        fn get_messages(&self, topic_hash: &TopicHash) -> Option<Box<[Message]>>;
        fn insert_message(&self, message: Message) -> Result<(), ()>;
    }

    #[derive(Debug, Default)]
    pub struct MemTopicStore {
        populated: DashMap<TopicHash, (String, DashSet<PeerId>)>,
        vacant: DashMap<TopicHash, DashSet<PeerId>>,
        subscribed: DashSet<TopicHash>,
    }
    impl TopicStore for MemTopicStore {
        fn insert_string(&self, topic_string: String) {
            let hash = Sha256Topic::new(topic_string.clone()).hash();
            if let Some((_, list)) = self.vacant.remove(&hash) {
                self.populated.insert(hash, (topic_string, list));
                return; // move the entry from vacant to populated if present.
            };
            if self.populated.get_mut(&hash).is_none() {
                // Only insert when not present.
                self.populated
                    .insert(hash, (topic_string, Default::default()));
            }
        }
        /// Returns true when the topic is not present anywhere(false when already present or readable).
        fn insert_hash(&self, topic_hash: TopicHash) -> bool {
            if self.populated.get(&topic_hash).is_some() {
                return false; // The topic is present
            }
            if self.vacant.get(&topic_hash).is_some() {
                return false; // The topic is present
            }
            self.vacant.insert(topic_hash.clone(), DashSet::default());
            true // The topic was not presnet
        }
        fn try_map(&self, topic_hash: &TopicHash) -> Option<String> {
            self.populated
                .get(topic_hash)
                .map(|entry| entry.value().0.clone())
        }
        fn readable_topics(&self) -> Box<[(TopicHash, String)]> {
            self.populated
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().0.clone()))
                .collect()
        }
        fn hash_topics(&self) -> Box<[TopicHash]> {
            self.vacant
                .iter()
                .map(|entry| entry.key().clone())
                .collect()
        }
        fn participants(&self, topic_hash: &TopicHash) -> Option<Box<[PeerId]>> {
            self.populated
                .get(topic_hash)
                .map(|entry| entry.value().1.iter().map(|entry| *entry.key()).collect())
                .or(self
                    .vacant
                    .get(topic_hash)
                    .map(|entry| entry.value().iter().map(|entry| *entry.key()).collect()))
        }
        fn join_topic(&self, peer: &PeerId, topic_hash: &TopicHash) -> bool {
            self.insert_hash(topic_hash.clone());
            if let Some(mut entry) = self.vacant.get_mut(topic_hash) {
                return entry.value_mut().insert(*peer);
            }
            if let Some(mut entry) = self.populated.get_mut(topic_hash) {
                return entry.value_mut().1.insert(*peer);
            }
            unreachable!()
        }
        fn leave_topic(&self, peer: &PeerId, topic_hash: &TopicHash) -> bool {
            if let Some(mut entry) = self.vacant.get_mut(topic_hash) {
                return entry.value_mut().remove(peer).is_some();
            }
            if let Some(mut entry) = self.populated.get_mut(topic_hash) {
                return entry.value_mut().1.remove(peer).is_some();
            }
            false
        }
        /// Returns `true` when we have not subscribed before.  
        /// This will add to existing store if not presnet.
        fn subscribe_topic(&self, topic: ReadableTopic) -> bool {
            let hash: gossipsub::TopicHash = topic.get_hash().into();
            let is_subscribed = self.subscribed.insert(hash.clone());
            if let ReadableTopic::HashOnly(_) = topic {
                if self.vacant.get(&hash).is_none() {
                    // insert if not present
                    self.vacant.insert(hash, Default::default());
                }
                return is_subscribed;
            }
            if self.populated.get(&hash).is_none() {
                // insert if not present
                self.populated.insert(
                    hash,
                    (topic.get_string().unwrap().to_owned(), Default::default()),
                );
            }
            is_subscribed
        }
        fn unsubscribe_topic(&self, topic_hash: &TopicHash) -> bool {
            self.subscribed.remove(&topic_hash).is_some()
        }
        fn subscribed_topics(&self) -> Box<[ReadableTopic]> {
            self.subscribed
                .iter()
                .map(|entry| {
                    let hash = entry.key().clone();
                    if let Some(string) = self
                        .populated
                        .get(&hash)
                        .map(|entry| entry.value().0.clone())
                    {
                        return ReadableTopic::Both {
                            hash: hash.into(),
                            string,
                        };
                    };
                    ReadableTopic::HashOnly(hash.into())
                })
                .collect()
        }
    }
    #[derive(Debug, Clone, Default)]
    pub struct MemMessageStore {
        inner: DashMap<TopicHash, Vec<super::Message>>,
    }
    impl MessageStore for MemMessageStore {
        fn get_messages(&self, topic_hash: &TopicHash) -> Option<Box<[Message]>> {
            self.inner
                .get(topic_hash)
                .map(|entry| entry.value().clone().into_boxed_slice())
        }

        fn insert_message(&self, message: Message) -> Result<(), ()> {
            match self.inner.get_mut(&message.topic) {
                Some(mut entry) => entry.value_mut().push(message),
                None => {
                    self.inner.insert(message.topic.clone(), vec![message]);
                }
            }
            Ok(())
        }
    }
}
