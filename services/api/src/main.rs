use tracing::Level;

use logging::configure_logging;
use redis_client_lib::RedisPubAsync;

use api::app;

#[tokio::main]
async fn main() {
    let config = env_vars::load().expect("Setting not loaded");

    configure_logging("api", config.loki_url.as_str(), Level::INFO)
        .await
        .expect("Error in logger initialization");

    let redis_hash =
        RedisPubAsync::new(&config.redis_url, &config.redis_channel)
            .await
            .unwrap();
    let app = app::App::new(redis_hash);
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.app.into_make_service())
        .await
        .unwrap();
}
