use std::fmt;

use serde::{de::DeserializeOwned, Serialize};
use tokio::{spawn, sync::mpsc, try_join};
use tokio_util::sync::CancellationToken;

use redis_client_lib::start_redis_subscription_async;
use url::Url;

use crate::{cancellable_task, flatten_task_result, Errors};

/// Основная задача для запуска
pub async fn task_main<M>(
    redis_url: Url,
    redis_channel: String,
    api_ws_port: u16,
    cancel: CancellationToken,
) -> Result<(), Errors>
where
    M: fmt::Debug + DeserializeOwned + Clone + Send + Serialize + 'static,
{
    let (tx_from_redis, rx_from_redis) = mpsc::channel::<M>(128);

    // запускаем поток подписки на сообщения из Redis
    let future = start_redis_subscription_async(
        redis_url.clone(),
        redis_channel.clone(),
        tx_from_redis,
    );
    let task_redis = spawn(cancellable_task(future, cancel.clone()));

    // запускаем поток для управления подключениями websocket
    let future = super::listen_port(
        cancel.clone(),
        rx_from_redis,
        api_ws_port,
        redis_url.clone(),
        redis_channel.clone(),
    );
    let task_listen_port = spawn(cancellable_task(future, cancel.clone()));

    match try_join!(
        flatten_task_result(task_redis, Errors::TokioTaskHandleError),
        flatten_task_result(task_listen_port, Errors::TokioTaskHandleError)
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
