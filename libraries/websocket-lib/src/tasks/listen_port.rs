use std::fmt;

use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    net::TcpListener,
    spawn,
    sync::{broadcast, mpsc},
};
use tokio_util::sync::CancellationToken;
use tracing::info;
use url::Url;

use crate::{cancellable_task, Errors};

pub async fn listen_port<M>(
    cancel: CancellationToken,
    rx_from_redis: mpsc::Receiver<M>,
    api_ws_port: u16,
    redis_url: Url,
    redis_channel: String,
) -> Result<(), Errors>
where
    M: Clone + fmt::Debug + DeserializeOwned + Send + Serialize + 'static,
{
    let addr = format!("0.0.0.0:{}", api_ws_port);

    let listener = create_tcp_listener(addr).await?;

    let (tx_broadcast, mut _rx_broadcast) = broadcast::channel(128);

    // получаем данные из redis и рассылаем потокам websocket
    let future = super::redis_broadcast(rx_from_redis, tx_broadcast.clone());
    spawn(cancellable_task(future, cancel.clone()));

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok((stream, addr)) = listener.accept().await {
        // let config_clone2 = config_clone.clone();
        let future = super::handle_ws_connection(
            stream,
            addr,
            tx_broadcast.subscribe(),
            redis_url.clone(),
            redis_channel.clone(),
        );
        spawn(cancellable_task(future, cancel.clone()));
    }
    Ok(())
}

async fn create_tcp_listener(addr: String) -> Result<TcpListener, Errors> {
    let listener = TcpListener::bind(&addr).await;
    let listener = match listener {
        Ok(value) => value,
        Err(error) => {
            return Err(Errors::BindToPortError(error));
        }
    };
    info!("Listening on: {}", addr);
    Ok(listener)
}
