//! Пример подписки на сообщения из Redis
//!
//! Запустить, после запуска можно отправлять сообщения. Сообщения должны
//! печататься. Отправлять можно простые строки, не забыть кавычки.

use std::str::FromStr;

use tokio::{join, main, spawn, sync::mpsc};
use url::Url;

use redis_client_lib::start_redis_subscription_async;

#[main]
async fn main() {
    let url = Url::from_str("redis://127.0.0.1").unwrap();
    let channel = "redis_sub_async";

    let (tx, mut rx) = mpsc::channel::<String>(32);

    let sp1 = spawn(async move {
        start_redis_subscription_async(url, channel.to_string(), tx)
            .await
            .unwrap();
    });

    let sp2 = spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("new message from redis: {}", msg);
        }
    });

    join!(sp1, sp2).0.unwrap();
}
