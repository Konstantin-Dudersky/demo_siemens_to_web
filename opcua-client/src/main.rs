use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::{sync::mpsc, time::Duration};

use messages::{types, Messages};
use opcua::types::{Identifier, NodeId};
use tokio::main;
use tracing::{error, info, trace};

use logging::logging;
use opcua_client::{
    convert, create_session, subscribe, write, Errors, ValueFromOpcUa,
    ValueToOpcUa,
};
use redis_client::{start_redis_subscription, RedisPubSync};

#[main]
async fn main() {
    logging("opcua-client", "http://localhost:3100")
        .await
        .expect("Error in logger initialization");
    let redis_url = "redis://127.0.0.1/";
    let redis_channel = "opcua";
    const OPCUA_URL: &str = "opc.tcp://192.168.101.180:4840/";

    let (channel_1_tx, channel_1_rx) = mpsc::channel::<ValueFromOpcUa>();

    // Поток подписки на изменения из OPC UA
    let thread1 = thread::spawn(move || loop {
        let result = thread_from_opcua(OPCUA_URL, &channel_1_tx);
        if let Err(err) = result {
            error!("OPC UA subscription end with error: {:?}", err);
        };
        info!("Restarting OPC UA subscription...");
        std::thread::sleep(Duration::from_secs(2));
    });

    // Поток отправки сообщений в Redis
    let thread2 = thread::spawn(move || loop {
        let result = thread_to_redis(redis_url, redis_channel, &channel_1_rx);
        if let Err(error) = result {
            error!("Error in Redis publisher: {error}")
        };
        info!("Restarting Redis publisher...");
        std::thread::sleep(Duration::from_secs(2));
    });

    let (channel_2_tx, channel_2_rx) = mpsc::channel::<Messages>();

    // Поток получения новых сообщений из Redis
    let thread3 = thread::spawn(move || {
        let result = thread_from_redis(redis_url, redis_channel, &channel_2_tx);
        if let Err(err) = result {
            error!("Error in Redis subscription: {:?}", err);
        }
        info!("Restarting Redis subscription...");
        std::thread::sleep(Duration::from_secs(2));
    });

    // Поток отправки новых данных в OPC UA
    let thread4 = thread::spawn(move || {
        let ch = channel_2_rx;
        loop {
            let result = thread_to_opcua(OPCUA_URL, &ch);
            if let Err(err) = result {
                error!("Error in OPC UA write: {:?}", err);
            }
            info!("Restarting OPC UA write...");
            std::thread::sleep(Duration::from_secs(2));
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
}

/// Поток подписки на изменения из OPC UA
fn thread_from_opcua(
    opcua_url: &str,
    channel_tx: &Sender<ValueFromOpcUa>,
) -> Result<(), Errors> {
    let nodes = create_nodes_for_subscription();
    subscribe(opcua_url, channel_tx.clone(), nodes)?;
    Ok(())
}

/// Поток отправки сообщений в Redis
fn thread_to_redis(
    redis_url: &str,
    redis_channel: &str,
    channel_rx: &Receiver<ValueFromOpcUa>,
) -> Result<(), Errors> {
    let mut redis_hash = RedisPubSync::new(redis_url, redis_channel)?;
    for msg in channel_rx {
        let msg = opcua_to_redis(&msg)?;
        if let Some(msg) = msg {
            redis_hash.set(&msg.key(), msg)?;
        }
    }
    Ok(())
}

/// Запуск потоков Redis -> OPC UA
fn from_redis_to_opcua(redis_url: &str, redis_channel: &str, opcua_url: &str) {}

/// Поток получения новых сообщений из Redis
fn thread_from_redis(
    redis_url: &str,
    redis_channel: &str,
    channel_tx: &Sender<Messages>,
) -> Result<(), Errors> {
    start_redis_subscription(redis_url, redis_channel, channel_tx)?;
    Ok(())
}

/// Поток отправки новых данных в OPC UA
fn thread_to_opcua(
    opcua_url: &str,
    channel_rx: &Receiver<Messages>,
) -> Result<(), Errors> {
    let session = create_session(opcua_url)?;
    for msg in channel_rx {
        trace!("Waiting for messages from Redis");
        match msg {
            Messages::IntValueToOpcUa(value) => {
                let value = ValueToOpcUa {
                    node_id: NodeId::new(4, 2),
                    value: convert::i16_to_variant(value.value),
                };
                write(session.clone(), value)?
            }
            _ => (),
        }
    }
    Ok(())
}

/// Перечисляем теги OPC UA, на которые будем подписываться
fn create_nodes_for_subscription() -> Vec<NodeId> {
    // namespace
    const NS: u16 = 4;
    vec![NodeId::new(NS, Identifier::Numeric(2))]
}

/// Подготавливаем полученные из OPC UA теги для отправки в Redis
fn opcua_to_redis(msg: &ValueFromOpcUa) -> Result<Option<Messages>, Errors> {
    match msg.node_id.identifier {
        Identifier::Numeric(2) => {
            let value = convert::variant_to_i16(&msg.value)?;
            let ts = convert::datetime_to_chrono(&msg.source_timestamp)?;
            let msg = Messages::IntValueFromOpcUa(types::SimpleValue::new(
                value,
                Some(ts),
            ));
            Ok(Some(msg))
        }
        _ => Ok(None),
    }
}
