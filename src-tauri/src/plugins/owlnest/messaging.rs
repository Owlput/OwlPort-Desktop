use std::{collections::HashMap, sync::Arc};
use owlnest::net::p2p::protocols::messaging::OutEvent;
use super::*;
use owlnest::net::p2p::{
    protocols::messaging::Message,
    swarm::{self, behaviour::BehaviourEvent},
};
use tokio::sync::RwLock;
use tracing::info;

#[derive(Clone)]
struct State{
    pub manager:swarm::Manager,
    pub connected_peers:Arc<RwLock<HashMap<PeerId,(bool,bool)>>>
}

pub fn init<R: Runtime>(manager: swarm::manager::Manager) -> TauriPlugin<R> {
    let state = State{
        manager:manager.clone(),
        connected_peers:Arc::new(RwLock::new(HashMap::new()))
    };
    Builder::new("owlnest-messaging")
        .setup(|app| {
            let app_handle = app.clone();
            let mut listener = manager.event_subscriber().subscribe();
            let state = state.clone();
            async_runtime::spawn(async move {
                while let Ok(ev) = listener.recv().await {
                    if let Ok(ev) = TryInto::<MessagingEmit>::try_into(ev.as_ref()) {
                        let _ = app_handle.emit_all("owlnest-messaging-emit", ev);
                        continue;
                    }
                    
                }
            });
            app.manage(state);
            Ok(())
        })
        .invoke_handler(generate_handler![send_msg])
        .build()
}

#[tauri::command]
async fn send_msg(
    state: tauri::State<'_, swarm::Manager>,
    target: String,
    msg: String,
) -> Result<(), String> {
    let peer_id = match target.parse::<PeerId>() {
        Ok(v) => v,
        Err(e) => {
            info!("Failed to parse peer ID for input {}: {}", target, e);
            return Err("Invalid PeerId".into());
        }
    };
    match state
        .messaging()
        .send_message(
            peer_id,
            Message::new(state.identity().get_peer_id(), peer_id, msg),
        )
        .await
    {
        Ok(_dur) => Ok(()),
        Err(e) => Err(format!("Failed to send message to {}: {}", peer_id, e)),
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum MessagingEmit {
    IncomingMessage { from: PeerId, msg: Message },
}
impl TryFrom<&> for MessagingEmit {
    type Error = ();
    fn try_from(value: &swarm::SwarmEvent) -> Result<Self, Self::Error> {
        let ev = match value {
            swarm::SwarmEvent::Behaviour(BehaviourEvent::Messaging(ev)) => {
                match ev {
                    owlnest::net::p2p::protocols::messaging::OutEvent::IncomingMessage {
                        from,
                        msg,
                    } => Self::IncomingMessage {
                        from: from.clone(),
                        msg: msg.clone(),
                    },
                    // owlnest::net::p2p::protocols::messaging::OutEvent::SendResult(_, _) => todo!(),
                    _ => return Err(()),
                }
            }
            _ => {
                return Err(());
            }
        };
        Ok(ev)
    }
}
