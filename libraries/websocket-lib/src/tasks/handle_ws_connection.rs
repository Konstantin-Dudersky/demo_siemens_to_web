use std::net::SocketAddr;

use futures_util::SinkExt;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::to_string as serialize;
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::info;
use url::Url;

use crate::{load_all_messages_from_hash, Errors};

/// Создание и управление подключением websocket
pub async fn handle_ws_connection<M>(
    raw_stream: TcpStream,
    addr: SocketAddr,
    mut rx: broadcast::Receiver<M>,
    redis_url: Url,
    redis_channel: String,
) -> Result<(), Errors>
where
    M: Clone + DeserializeOwned + Serialize,
{
    info!("Incoming TCP connection from: {}", addr);
    let mut ws_stream = accept_async(raw_stream).await?;
    info!("WebSocket connection established: {:?}", addr);

    let msgs: Vec<M> =
        load_all_messages_from_hash(redis_url, redis_channel).await?;
    for msg in msgs {
        let msg = serialize(&msg).unwrap();
        let result = ws_stream.send(Message::Text(msg)).await;
        match result {
            Ok(_) => (),
            Err(error) => {
                let error = error.to_string();
                return Err(Errors::SendToWsError(error));
            }
        };
    }

    while let Ok(msg) = rx.recv().await {
        let msg = serialize(&msg).unwrap();
        let result = ws_stream.send(Message::Text(msg)).await;
        match result {
            Ok(_) => (),
            Err(error) => {
                let error = error.to_string();
                return Err(Errors::SendToWsError(error));
            }
        };
    }
    Ok(())
}

// let mut rx_clone = tx.subscribe();
// spawn(async move {
//     let connection = task_handle_ws_connection(
//         stream,
//         addr,
//         &mut rx_clone,
//         config_clone2.redis_url,
//         config_clone2.redis_channel,
//     )
//     .await;
//     match connection {
//         Ok(_) => warn!("Unexpected end of thread"),
//         Err(error) => {
//             info!("Connection from {addr} closed: {error:?}");
//         }
//     };
// });
