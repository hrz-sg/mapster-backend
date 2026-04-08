use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, broadcast::{self, Sender}};

use crate::ws::notification::IntoWsNotification;

pub mod notification;
mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsChatMessage {
    pub chat_id: String,
    pub user_id: String,
    pub text: Option<String>,
}

impl IntoWsNotification for WsChatMessage {
    const METHOD: &'static str = "chat.message";
}

type ConnId = usize;

#[derive(Clone)]
pub struct Ws {
    chats: Arc<RwLock<HashMap<String, Sender<WsChatMessage>>>>, // chat_id -> broadcast sender
    user_sockets: Arc<RwLock<HashMap<String, Vec<ConnId>>>>,     // user_id -> conn_ids
    socket_channels: Arc<RwLock<HashMap<ConnId, broadcast::Receiver<WsChatMessage>>>>, // conn_id -> receiver
    next_conn_id: Arc<RwLock<ConnId>>,
}

impl Ws {
    pub fn new() -> Self {
        Self {
            chats: Arc::new(RwLock::new(HashMap::new())),
            user_sockets: Arc::new(RwLock::new(HashMap::new())),
            socket_channels: Arc::new(RwLock::new(HashMap::new())),
            next_conn_id: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn subscribe(&self, chat_id: &str) -> broadcast::Receiver<WsChatMessage> {
        let mut chats = self.chats.write().await;
        let tx = chats
            .entry(chat_id.to_string())
            .or_insert_with(|| broadcast::channel(100).0); // capacity 100
        tx.subscribe()
    }

    pub async fn broadcast(&self, chat_id: &str, msg: WsChatMessage) {
        if let Some(tx) = self.chats.read().await.get(chat_id) {
            let _ = tx.send(msg);
        }
    }

    pub async fn add_socket(&self, user_id: &str, rx: broadcast::Receiver<WsChatMessage>) -> ConnId {
        let mut id_lock = self.next_conn_id.write().await;
        let conn_id = *id_lock;
        *id_lock += 1;

        self.socket_channels.write().await.insert(conn_id, rx);
        self.user_sockets.write().await
            .entry(user_id.to_string())
            .or_default()
            .push(conn_id);

        conn_id
    }

    pub async fn remove_socket(&self, user_id: &str, conn_id: ConnId) {
        self.socket_channels.write().await.remove(&conn_id);

        if let Some(vec) = self.user_sockets.write().await.get_mut(user_id) {
            vec.retain(|&id| id != conn_id);
            if vec.is_empty() {
                self.user_sockets.write().await.remove(user_id);
            }
        }
    }
}