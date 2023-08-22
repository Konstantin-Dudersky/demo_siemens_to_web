use std::sync::Arc;

use axum::{extract, routing};
use serde_json::to_string as serialize;
use tokio::sync::Mutex;

use messages;
use redis_client::RedisHashAsync;

struct App {
    app: routing::Router,
}

#[derive(Clone)]
struct AppState {
    redis_hash: Arc<Mutex<RedisHashAsync>>,
}

impl App {
    pub fn new(redis_hash: RedisHashAsync) -> Self {
        let shared_state = AppState {
            redis_hash: Arc::new(Mutex::new(redis_hash)),
        };
        let app = routing::Router::new()
            .route("/", routing::get(root))
            .route("/value/:id", routing::get(get_value))
            .route("/value/:id", routing::put(replace_value))
            .with_state(shared_state);
        Self { app }
    }
}

#[tokio::main]
async fn main() {
    let redis_hash = RedisHashAsync::new("redis://127.0.0.1/", "hash_key")
        .await
        .unwrap();
    let app = App::new(redis_hash);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_value(
    extract::Path(id): extract::Path<String>,
    extract::State(state): extract::State<AppState>,
) -> String {
    println!("id: {id}");
    let mut redis_hash = state.redis_hash.lock().await;
    redis_hash.get(&id).await.unwrap()
}

async fn replace_value(
    extract::Path(id): extract::Path<String>,
    extract::State(state): extract::State<AppState>,
    extract::Json(payload): extract::Json<messages::Messages>,
) -> String {
    let mut redis_hash = state.redis_hash.lock().await;
    redis_hash.set(&id, &payload).await.unwrap();
    let msg: messages::Messages = redis_hash.get(&id).await.unwrap();
    serialize(&msg).unwrap()
}

// test ------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use serde_json::to_string as serialize;

    use super::*;

    async fn create_test_server() -> TestServer {
        let redis_hash = RedisHashAsync::new("redis://127.0.0.1/", "test_api")
            .await
            .unwrap();
        let app = App::new(redis_hash);
        TestServer::new(app.app.into_make_service()).unwrap()
    }

    #[tokio::test]
    async fn test1() {
        let msg1 =
            messages::Messages::IntValueFromOpcUa(messages::SimpleValue {
                value: 15,
            });

        let test_server = create_test_server().await;

        let put_response = test_server.put("/value/test_msg").json(&msg1).await;
        put_response.assert_status_ok();
        put_response.assert_text(serialize(&msg1).unwrap());
    }
}
