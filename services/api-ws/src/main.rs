use std::{net::SocketAddr, time::Duration};

use env_vars::Config;
use futures_util::SinkExt;
use tokio::{
    main,
    net::{TcpListener, TcpSocket, TcpStream},
    select, spawn,
    sync::{
        broadcast::{self, Receiver},
        mpsc,
    },
    task::JoinHandle,
    time::sleep,
};
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn, Level};
use url::Url;

use api_ws::{load_all_messages_from_hash, Errors};
use logging::configure_logging;
use messages::Messages;
use redis_client::start_redis_subscription_async;

#[main]
async fn main() {
    let config = env_vars::load().expect("Settings not loaded");

    configure_logging("api-ws", config.loki_url.as_str(), Level::DEBUG)
        .await
        .expect("Error in logger initialization");

    loop {
        let result = app(&config).await;
        match result {
            Ok(_) => todo!(),
            Err(error) => match error {
                Errors::BindToPortError(error) => {
                    error!("Stop program: {:?}", error);
                    // return;
                }
                _ => {
                    warn!("{:?}", error);
                }
            },
        };
        sleep(Duration::from_secs(5)).await;
        info!("Restarting...");
    }
}

async fn app(config: &Config) -> Result<(), Errors> {
    let (tx_from_redis, mut rx_from_redis) = mpsc::channel::<Messages>(128);

    // запускаем поток подписки на сообщения из Redis
    let config_clone = config.clone();
    let thread_redis = spawn(async move {
        let result = start_redis_subscription_async(
            &config_clone.redis_url,
            &config_clone.redis_channel,
            &tx_from_redis,
        )
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(Errors::from(error)),
        }
    });

    // запускаем поток для управления подключениями websocket
    let config_clone = config.clone();
    let thread_all_ws = spawn(async move {
        let addr = format!("0.0.0.0:{}", config_clone.api_ws_port);

        // let socket = TcpSocket::new_v4().unwrap();
        // socket.set_reuseaddr(true).unwrap();
        // socket.bind(addr.parse().unwrap()).unwrap();
        // let listener = socket.listen(1024).unwrap();
        let listener = TcpListener::bind(&addr).await;
        let listener = match listener {
            Ok(value) => value,
            Err(error) => {
                return Err(Errors::BindToPortError(error));
            }
        };
        info!("Listening on: {}", addr);

        let (tx, mut _rx) = broadcast::channel(128);

        // получаем данные из redis и рассылаем потокам websocket
        let tx_clone = tx.clone();
        spawn(async move {
            while let Some(msg) = rx_from_redis.recv().await {
                tx_clone.send(msg).unwrap();
            }
        });

        while let Ok((stream, addr)) = listener.accept().await {
            let mut rx_clone = tx.subscribe();
            let config_clone2 = config_clone.clone();
            spawn(async move {
                let connection = handle_ws_connection(
                    stream,
                    addr,
                    &mut rx_clone,
                    config_clone2.redis_url,
                    config_clone2.redis_channel,
                )
                .await;
                match connection {
                    Ok(_) => warn!("Unexpected end of thread"),
                    Err(error) => {
                        info!("Connection from {addr} closed: {error:?}");
                    }
                };
            });
        }
        Ok(())
    });

    // match try_join!(task_flatten(thread_redis), task_flatten(thread_all_ws)) {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(e),
    // }
    select! {
        res = thread_redis => {
            match res {
                Ok(res1) => match res1 {
                    Ok(_) => todo!(),
                    Err(e) => {
                        thread_all_ws.abort_handle();
                        Err(e)
                    }
                },
                Err(e) => todo!(),
            }
        },
        res = thread_all_ws => {
            match res {
                Ok(res1) => match res1 {
                    Ok(_) => todo!(),
                    Err(e) => Err(e),
                },
                Err(e) => todo!(),
            }
        },
    }
}

/// Ожидаем завершения задачи и упрощаем структуру результата
async fn task_flatten<T>(
    handle: JoinHandle<Result<T, Errors>>,
) -> Result<T, Errors> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(Errors::TokioTaskHandleError(err)),
    }
}

/// Создание и управление подключением websocket
async fn handle_ws_connection(
    raw_stream: TcpStream,
    addr: SocketAddr,
    rx: &mut Receiver<Messages>,
    redis_url: Url,
    redis_channel: String,
) -> Result<(), Errors> {
    info!("Incoming TCP connection from: {}", addr);
    let mut ws_stream = accept_async(raw_stream).await?;
    info!("WebSocket connection established: {:?}", addr);

    let msgs = load_all_messages_from_hash(redis_url, redis_channel).await?;
    for msg in msgs {
        let msg = msg.serialize()?;
        let result = ws_stream.send(msg.into()).await;
        match result {
            Ok(_) => (),
            Err(error) => {
                let error = error.to_string();
                return Err(Errors::SendToWsError(error));
            }
        };
    }

    while let Ok(msg) = rx.recv().await {
        let msg = msg.serialize()?;
        let result = ws_stream.send(msg.into()).await;
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
