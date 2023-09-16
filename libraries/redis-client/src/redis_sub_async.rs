//! Реализация подписки на сообщения, асинхронная

use std::fmt::Debug;

use tokio::sync::mpsc::Sender;

use serde::de::DeserializeOwned;
use serde_json::from_str as deserialize;
use tracing::trace;

use futures_util::StreamExt as _;
use url::Url;

use crate::errors::Errors;

/// Подписка на сообщения из Redis. Вызывать из задачи Tokio.
pub async fn start_redis_subscription_async<V>(
    url: Url,
    channel: String,
    tx: Sender<V>,
) -> Result<(), Errors>
where
    V: Debug + DeserializeOwned,
{
    let client = redis::Client::open(url.to_string())?;
    let connection = client.get_async_connection().await?;
    let mut pubsub = connection.into_pubsub();
    pubsub.subscribe(channel).await?;
    let mut stream = pubsub.on_message();
    loop {
        let msg = stream.next().await;
        let msg = match msg {
            Some(value) => value,
            None => return Err(Errors::GetMessageError),
        };
        trace!("New message from Redis: {:?}", msg);
        let payload: String = msg.get_payload()?;
        let payload: V = match deserialize(&payload) {
            Ok(value) => value,
            Err(err) => return Err(Errors::DeserializeError(err.to_string())),
        };
        if let Err(err) = tx.send(payload).await {
            return Err(Errors::SendThreadChannleError(err.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::super::RedisPubAsync;
    use super::*;

    use tokio::{spawn, sync::mpsc};

    use ntest::timeout;

    /// Запустить - cargo test redis_sub_async::tests::test1
    #[tokio::test]
    #[timeout(1000)]
    async fn test1() {
        let url = Url::from_str("redis://127.0.0.1").expect("");
        let channel = "redis_sub_async";
        let msg_content = "test pub value";

        let (tx, mut rx) = mpsc::channel::<String>(32);

        // запускаем задачу с подпиской
        let url_clone = url.clone();
        let _ = spawn(async move {
            start_redis_subscription_async(url_clone, channel.to_string(), tx)
                .await
                .expect("");
        });

        let sp2 = spawn(async move {
            // отправляем сообщение
            let mut redis_hash =
                RedisPubAsync::new(&url, channel).await.expect("");
            redis_hash.set(channel, msg_content).await.expect("");

            // проверяем, что сообщение пришло
            while let Some(msg) = rx.recv().await {
                assert_eq!(msg_content, msg);
                break;
            }
        });

        sp2.await.expect("");
    }
}
