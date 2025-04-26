pub mod ws {
    use std::{sync::Arc, thread::sleep, time::{Duration, SystemTime}};

    use futures_util::StreamExt;
    use log::{debug, info};
    use tokio::{net::{TcpListener, TcpStream}, sync::Mutex};
    use tokio_tungstenite::{accept_async, tungstenite::Message};
    use crate::{config, service::ConnectionManager};

    async fn handle_message(connection_id: String, message: Message) {
        let conn_manager = ConnectionManager::instance();
        let message_string_raw = message.into_text();
        if message_string_raw.is_err() { return; }
        let message_string = message_string_raw.unwrap();
        let send_message_res = conn_manager.send_message(&connection_id, message_string.to_string()).await;
        if send_message_res.is_err() { return; }
    }

    async fn handle_connection(stream: TcpStream) {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");

            debug!("🔌 New WebSocket connection");

            let connection_id = uuid::Uuid::new_v4().to_string();

            let (write, mut read) = ws_stream.split();
            let conn_manager = ConnectionManager::instance();
            conn_manager.add_connection(connection_id.clone(), write);

            let last_pong_time = Arc::new(Mutex::new(SystemTime::now()));

            ping_connection(connection_id.clone(), last_pong_time.clone()).await;

            while let Some(Ok(msg)) = read.next().await {
                loop {
                    if msg.is_pong() {
                        let mut time = last_pong_time.lock().await;
                        *time = SystemTime::now();
                        break;
                    }
                    debug!("📨 Received: {:?}", msg);
                    if msg.is_close() {
                        handle_connection_termination(connection_id.clone()).await;
                        break;
                    }
                    if msg.is_text() || msg.is_binary() {
                        handle_message(connection_id.clone(), msg).await;
                        break;
                    }
                    break;
                }
            }
        });
    }

    async fn ping_connection(connection_id: String, last_pong_time: Arc<Mutex<SystemTime>>) {
            tokio::spawn(async move {
                loop {
                    let conn_manager = ConnectionManager::instance();
                    conn_manager.ping_connection(&connection_id).await;
                    let last_pong_time_clone = last_pong_time.clone();
                    sleep(Duration::from_secs(*config::app::PING_INTERVAL_SECS));
                    let last_pong_time_lock = last_pong_time_clone.lock().await;
                    if let Ok(elapsed) = last_pong_time_lock.elapsed() {
                        if elapsed > Duration::from_secs(*config::app::PING_ELAPSED_SECS) {
                            handle_connection_termination(connection_id.clone()).await;
                            return;
                        }
                    }
                }
            });
    }

    async fn handle_connection_termination(connection_id: String) {
        let conn_manager = ConnectionManager::instance();
        conn_manager.remove_connection(&connection_id);
        info!("❌ Connection closed");
    }

    pub async fn init() {
        let _ = env_logger::try_init();
        let addr = format!("127.0.0.1:{}", *config::app::PORT).to_string();
        let listener = TcpListener::bind(&addr).await.unwrap();
        info!("🚀 Websocket server is listening at {}", &addr);
        
        while let Ok((stream, _)) = listener.accept().await {
            handle_connection(stream).await;
        }
    }
}

