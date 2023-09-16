use std::sync::Arc;

use tokio::sync::Mutex;

use redis_client_lib::RedisPubAsync;

#[derive(Clone)]
pub struct AppState {
    pub redis_hash: Arc<Mutex<RedisPubAsync>>,
}
