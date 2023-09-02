//! Реализация подписки на сообщения

use std::sync::mpsc::Sender;

use serde::de::DeserializeOwned;
use serde_json::from_str as deserialize;
use tracing::trace;

use crate::errors::Errors;

pub fn start_redis_subscription<V>(
    url: &str,
    channel: &str,
    tx: &Sender<V>,
) -> Result<(), Errors>
where
    V: DeserializeOwned + std::fmt::Debug,
{
    let client = redis::Client::open(url)?;
    let mut connection = client.get_connection()?;
    let mut pubsub = connection.as_pubsub();
    pubsub.subscribe(channel)?;
    loop {
        let msg = pubsub.get_message()?;
        let payload: String = msg.get_payload()?;
        trace!("New message from Redis: {:?}", msg);
        let payload: V = match deserialize(&payload) {
            Ok(value) => value,
            Err(err) => return Err(Errors::DeserializeError(err.to_string())),
        };
        if let Err(err) = tx.send(payload) {
            return Err(Errors::SendThreadChannleError(err.to_string()));
        }
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
        thread::spawn(move || {
            start_redis_subscription(url, channel, &tx).unwrap();
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
