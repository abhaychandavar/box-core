use std::sync::Arc;

use dashmap::DashMap;
use futures::{stream::SplitSink, SinkExt};
use once_cell::sync::Lazy;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::{self, Message, Utf8Bytes}, WebSocketStream};

#[allow(dead_code)]
pub struct WsConnection {
    pub id: String,
    pub conn: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>
}

pub struct ConnectionManager {
    connections: DashMap<String, WsConnection>
}

impl ConnectionManager {
    fn new() -> Self {
        return Self {
            connections: DashMap::new()
        }
    }

    pub fn instance() -> &'static ConnectionManager {
        static INSTANCE: Lazy<ConnectionManager> = Lazy::new(ConnectionManager::new);
        return &INSTANCE;
    }

    pub fn add_connection(&self, id: String, conn: SplitSink<WebSocketStream<TcpStream>, Message>) {
        let connection = WsConnection {
            id: id.clone(),
            conn: Arc::new(Mutex::new(conn)),
        };
        self.connections.insert(id, connection);
    }

    pub fn remove_connection(&self, id: &String) {
        self.connections.remove(id);
    }

    pub async fn send_message(&self, id: &str, message: String) -> Result<(), tungstenite::Error> {
        if let Some(connection) = self.connections.get(id) {
            let conn_clone = connection.conn.clone();
            let mut conn = conn_clone.lock().await;
            
            conn.send(Message::Text(Utf8Bytes::from(message))).await?; 
    
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn ping_connection(&self, id: &str) {
        if let Some(connection) = self.connections.get(id) {
            let conn_clone = connection.conn.clone();
            let mut conn = conn_clone.lock().await;
            let _ = conn.send(Message::Ping("ping".into())).await;
        }
    }
}
