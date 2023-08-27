//! Реализация подписки на сообщения

use std::sync::mpsc::Sender;

use serde::de::DeserializeOwned;
use serde_json::from_str as deserialize;

pub fn start_redis_subscription<V>(url: &str, channel: &str, tx: Sender<V>)
where
    V: DeserializeOwned + std::fmt::Debug,
{
    // TODO - обработка ошибок
    let client = redis::Client::open(url).unwrap();
    let mut connection = client.get_connection().unwrap();
    let mut pubsub = connection.as_pubsub();
    pubsub.subscribe(channel).unwrap();
    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        let payload: V = deserialize(&payload).unwrap();
        println!("channel '{}': {:?}", msg.get_channel_name(), payload);
        tx.send(payload).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::super::RedisPubAsync;
    use super::*;

    use std::sync::mpsc;
    use std::thread;

    use ntest::timeout;

    /// Запустить - cargo test redis_sub::tests::test1
    #[tokio::test]
    #[timeout(1000)]
    async fn test1() {
        let url = "redis://127.0.0.1";
        let channel = "test_pub";
        let msg_content = "test pub value";

        let (tx, rx) = mpsc::channel::<String>();

        // запускаем поток с подпиской
        thread::spawn(|| {
            start_redis_subscription(url, channel, tx);
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
}
