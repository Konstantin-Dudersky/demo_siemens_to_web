use tokio::{spawn, sync::mpsc, try_join};
use tokio_util::sync::CancellationToken;

use env_vars::Config;
use messages::Messages;
use redis_client::start_redis_subscription_async;

use crate::{cancellable_task, flatten_task_result, Errors};

/// Основная задача для запуска
pub async fn task_main(
    config: &Config,
    cancel: CancellationToken,
) -> Result<(), Errors> {
    let (tx_from_redis, rx_from_redis) = mpsc::channel::<Messages>(128);

    // запускаем поток подписки на сообщения из Redis
    let future = start_redis_subscription_async(
        config.redis_url.clone(),
        config.redis_channel.clone(),
        tx_from_redis,
    );
    let task_redis = spawn(cancellable_task(future, cancel.clone()));

    // запускаем поток для управления подключениями websocket
    let future = super::listen_port(
        cancel.clone(),
        rx_from_redis,
        config.api_ws_port,
        config.redis_url.clone(),
        config.redis_channel.clone(),
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
