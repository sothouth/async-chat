use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::connection::Outbound;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { name, sender }
    }
    pub fn join(&self, member: Arc<Outbound>) {
        let receiver = self.sender.subscribe();
        task::spawn(handle_receiver(self.name.clone(), receiver, member));
    }
    pub fn post(&self, message: Arc<String>) {
        let _ = self.sender.send(message);
    }
}

use async_chat::FromServer;
use tokio::sync::broadcast::error::RecvError;

async fn handle_receiver(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop {
        let message = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => FromServer::Error(format!(
                "channel {}: {} messages were dropped.",
                group_name, n
            )),
            Err(RecvError::Closed) => {
                break;
            }
        };
        if outbound.send(message).await.is_err() {
            break;
        }
    }
}
