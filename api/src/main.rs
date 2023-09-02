use api::app;
use redis_client::RedisPubAsync;

#[tokio::main]
async fn main() {
    let redis_hash = RedisPubAsync::new("redis://127.0.0.1/", "opcua")
        .await
        .unwrap();
    let app = app::App::new(redis_hash);
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.app.into_make_service())
        .await
        .unwrap();
}
