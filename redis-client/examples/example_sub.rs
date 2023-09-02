//! Пример реализации подписки на новые сообщения

use std::sync::mpsc;
use std::thread;

use tokio::main;

use redis_client::{start_redis_subscription, RedisPubAsync};

#[main]
async fn main() {
    let url = "redis://127.0.0.1";
    let channel = "test_sub";
    let msg_content = "test sub value";

    let (tx, rx) = mpsc::channel::<String>();

    // запускаем поток с подпиской
    thread::spawn(|| {
        start_redis_subscription(url, channel, tx).unwrap();
    });

    // отправляем сообщение
    let mut redis_hash = RedisPubAsync::new(url, channel).await.unwrap();
    redis_hash.set(channel, msg_content).await.unwrap();

    // проверяем, что сообщение пришло
    for msg in rx {
        assert_eq!(msg_content, msg);
        break;
    }
}
