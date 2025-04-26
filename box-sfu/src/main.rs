use dotenv::dotenv;
use log::debug;
mod service;
use crate::service::ws::ws;
mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_env_logger().await;
    ws::init().await;
}

async fn init_env_logger () {
    env_logger::init();
    let log_level = format!("{}", *config::app::LOG_LEVEL).to_string();
    debug!("Log Level: {}", &log_level);
}