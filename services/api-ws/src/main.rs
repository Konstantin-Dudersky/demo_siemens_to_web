use std::{net::SocketAddr, str::FromStr};

use futures_util::SinkExt;
use tokio::{
    main,
    net::{TcpListener, TcpStream},
    spawn,
    sync::{
        broadcast::{self, Receiver},
        mpsc::{self},
    },
};
use tracing::info;

use logging::logging;
use messages::Messages;
use redis_client::{start_redis_subscription_async, RedisPubAsync};
use url::Url;

#[main]
async fn main() {
    let config = env_vars::load().expect("Settings not loaded");

    logging("api-ws", config.loki_url.as_str())
        .await
        .expect("Error in logger initialization");

    let (tx, mut rx) = mpsc::channel::<Messages>(128);

    let config_clone = config.clone();
    let sp1 = spawn(async move {
        start_redis_subscription_async(
            &config_clone.redis_url,
            &config_clone.redis_channel,
            &tx,
        )
        .await
        .unwrap();
    });

    let _ = spawn(async move {
        let addr = "0.0.0.0:8081";

        let try_socket = TcpListener::bind(addr).await;
        let listener = try_socket.expect("Failed to bind");
        info!("Listening on: {}", addr);

        let (tx, mut rx1) = broadcast::channel(16);

        let tx_clone = tx.clone();
        spawn(async move {
            while let Some(msg) = rx.recv().await {
                tx_clone.send(msg).unwrap();
            }
        });

        while let Ok((stream, addr)) = listener.accept().await {
            let mut rx_clone = tx.subscribe();
            let config_clone = config.clone();
            tokio::spawn(async move {
                handle_connection(
                    stream,
                    addr,
                    &mut rx_clone,
                    config_clone.redis_url,
                    config_clone.redis_channel,
                )
                .await;
            });
        }
    });

    sp1.await.unwrap();
}

async fn handle_connection(
    raw_stream: TcpStream,
    addr: SocketAddr,
    rx: &mut Receiver<Messages>,
    redis_url: Url,
    redis_channel: String,
) {
    info!("Incoming TCP connection from: {}", addr);
    let mut ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {:?}", addr);
    let mut redis = RedisPubAsync::new(&redis_url, &redis_channel)
        .await
        .unwrap();
    let msgs: Vec<Messages> = redis.hvals().await.unwrap();
    for msg in msgs {
        let msg = msg.serialize().unwrap();
        ws_stream.send(msg.into()).await.unwrap();
    }
    while let Ok(msg) = rx.recv().await {
        let msg = msg.serialize().unwrap();
        ws_stream.send(msg.into()).await.unwrap();
    }
}
