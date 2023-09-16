use std::sync::Arc;

use axum::routing;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use redis_client_lib::RedisPubAsync;

use crate::{routes, state};

pub struct App {
    pub app: routing::Router,
}

impl App {
    pub fn new(redis_hash: RedisPubAsync) -> Self {
        let shared_state = state::AppState {
            redis_hash: Arc::new(Mutex::new(redis_hash)),
        };
        let app = routing::Router::new()
            .route("/value/:id", routing::get(routes::value::get))
            .route("/value/:id", routing::put(routes::value::put))
            .with_state(shared_state)
            .layer(CorsLayer::permissive());
        Self { app }
    }
}
