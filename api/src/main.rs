use api::app;
use redis_client::RedisHashAsync;

#[tokio::main]
async fn main() {
    let redis_hash = RedisHashAsync::new("redis://127.0.0.1/", "test_api")
        .await
        .unwrap();
    let app = app::App::new(redis_hash);
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.app.into_make_service())
        .await
        .unwrap();
}
