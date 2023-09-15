use tokio::sync::{broadcast, mpsc};

use crate::Errors;

/// Задача для пересылки сообщений из Redis в broadcast для всех потоков WS
pub async fn redis_broadcast<M>(
    mut rx_from_redis: mpsc::Receiver<M>,
    tx: broadcast::Sender<M>,
) -> Result<(), Errors>
where
    M: std::fmt::Debug + std::marker::Send,
{
    while let Some(msg) = rx_from_redis.recv().await {
        tx.send(msg).unwrap();
    }
    Ok(())
}
