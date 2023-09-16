//! Пример реализации подписки на новые сообщения

use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

use tokio::main;

use redis_client_lib::{start_redis_subscription, RedisPubAsync};
use url::Url;

#[main]
async fn main() {
    let url = Url::from_str("redis://127.0.0.1").expect("");
    let channel = "test_sub";
    let msg_content = "test sub value";

    let (tx, rx) = mpsc::channel::<String>();

    // запускаем поток с подпиской
    let url_clone = url.clone();
    thread::spawn(move || {
        start_redis_subscription(&url_clone, channel, &tx).unwrap();
    });

    // отправляем сообщение
    let mut redis_hash = RedisPubAsync::new(&url, channel).await.unwrap();
    redis_hash.set(channel, msg_content).await.unwrap();

    // проверяем, что сообщение пришло
    for msg in rx {
        assert_eq!(msg_content, msg);
        break;
    }
}
