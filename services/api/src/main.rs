use std::str::FromStr;

use api::app;
use redis_client::RedisPubAsync;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::from_str("redis://127.0.0.1").expect("");
    let redis_hash = RedisPubAsync::new(&url, "opcua").await.unwrap();
    let app = app::App::new(redis_hash);
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.app.into_make_service())
        .await
        .unwrap();
}
