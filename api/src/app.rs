use std::sync::Arc;

use axum::routing;
use tokio::sync::Mutex;

use redis_client::RedisHashAsync;

use crate::{routes, state};

pub struct App {
    pub app: routing::Router,
}

impl App {
    pub fn new(redis_hash: RedisHashAsync) -> Self {
        let shared_state = state::AppState {
            redis_hash: Arc::new(Mutex::new(redis_hash)),
        };
        let app = routing::Router::new()
            .route("/value/:id", routing::get(routes::value::get))
            .route("/value/:id", routing::put(routes::value::replace))
            .with_state(shared_state);
        Self { app }
    }
}