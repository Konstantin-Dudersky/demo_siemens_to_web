use std::sync::Arc;

use tokio::sync::Mutex;

use redis_client::RedisHashAsync;

#[derive(Clone)]
pub struct AppState {
    pub redis_hash: Arc<Mutex<RedisHashAsync>>,
}
