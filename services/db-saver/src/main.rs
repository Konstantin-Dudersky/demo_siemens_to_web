use sqlx::postgres::PgPoolOptions;
use tokio::{main, spawn, sync::mpsc};
use tracing::Level;

use db_saver_lib::save_row_in_db;
use env_vars;
use logging::configure_logging;
use messages::Messages;
use redis_client_lib::start_redis_subscription_async;

mod config;

#[main]
async fn main() {
    let config = env_vars::load().expect("Settings not loaded");

    configure_logging("db-saver", config.loki_url.as_str(), Level::INFO)
        .await
        .expect("Error in logger initialization");

    let db_url = format!(
        "postgres://{}:{}@{}:{}/db_data",
        config.db_user, config.db_password, config.db_host, config.db_port
    );

    let (tx, mut rx) = mpsc::channel::<Messages>(32);

    let config_clone = config.clone();
    let sp1 = spawn(async move {
        start_redis_subscription_async(
            config_clone.redis_url,
            config_clone.redis_channel,
            tx,
        )
        .await
        .unwrap();
    });

    let _ = spawn(async move {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        while let Some(msg) = rx.recv().await {
            let row = config::prepare_msg_from_redis_to_db(msg);
            if let Some(row) = row {
                save_row_in_db(&row, &pool).await.unwrap();
            };
        }
    });

    sp1.await.unwrap();
}
