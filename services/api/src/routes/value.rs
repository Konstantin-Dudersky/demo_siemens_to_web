use axum::extract;
use serde_json::to_string as serialize;

use messages;

use crate::state;

pub async fn get(
    extract::Path(id): extract::Path<String>,
    extract::State(state): extract::State<state::AppState>,
) -> String {
    println!("id: {id}");
    let mut redis_hash = state.redis_hash.lock().await;
    let msg: messages::Messages = redis_hash.get(&id).await.unwrap();
    println!("{msg}");
    serialize(&msg).unwrap()
}

pub async fn put(
    extract::Path(id): extract::Path<String>,
    extract::State(state): extract::State<state::AppState>,
    extract::Json(payload): extract::Json<messages::Messages>,
) -> String {
    let mut redis_hash = state.redis_hash.lock().await;
    redis_hash.set(&id, &payload).await.unwrap();
    let msg: messages::Messages = redis_hash.get(&id).await.unwrap();
    serialize(&msg).unwrap()
}
