use messages::{types, Messages};
use opcua::types::{Identifier, NodeId};
use std::{
    any::Any,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
use tokio::main;
use tracing::{error, info};
use url::Url;

use env_vars;
use logging::logging;
use opcua_client::{
    convert, subscribe, write, Errors, ValueFromOpcUa, ValueToOpcUa,
};
use redis_client::{start_redis_subscription, RedisPubSync};

#[main]
async fn main() {
    let config = env_vars::load().expect("Настройки не загружены");

    logging("opcua-client", config.loki_url.as_str())
        .await
        .expect("Error in logger initialization");

    // Запуск потоков OPC UA -> Redis
    let config_copy = config.clone();
    let threads_opcua_to_redis = thread::spawn(move || loop {
        let res = threads_opcua_to_redis(
            &config_copy.redis_url,
            &config_copy.redis_channel,
            &config_copy.opcua_url,
        );
        if let Err(err) = res {
            error!("Error in threads OPC UA -> Redis: {err:?}")
        };
        info!("Restarting threads OPC UA -> Redis...");
        std::thread::sleep(Duration::from_secs(2));
    });

    // Запуск потоков Redis -> OPC UA
    let config_copy = config.clone();
    let threads_redis_to_opcua = thread::spawn(move || loop {
        let res = threads_redis_to_opcua(
            &config_copy.redis_url,
            &config_copy.redis_channel,
            &config_copy.opcua_url,
        );
        if let Err(err) = res {
            error!("Error in threads Redis -> OPC UA: {err:?}")
        };
        info!("Restarting threads Redis -> OPC UA...");
        std::thread::sleep(Duration::from_secs(2));
    });
    threads_opcua_to_redis.join().unwrap();
    threads_redis_to_opcua.join().unwrap();
}

// -----------------------------------------------------------------------------

/// Запуск потоков OPC UA -> Redis
fn threads_opcua_to_redis(
    redis_url: &Url,
    redis_channel: &str,
    opcua_url: &Url,
) -> Result<(), Box<dyn Any + Send>> {
    let redis_url = redis_url.to_string();
    let redis_channel = redis_channel.to_string();
    let opcua_url = opcua_url.to_string();

    let (channel_tx, channel_rx) = mpsc::channel::<ValueFromOpcUa>();

    // Поток подписки на изменения из OPC UA
    let thread1 = thread::spawn(move || {
        let result = thread_from_opcua(&opcua_url, &channel_tx);
        if let Err(err) = result {
            error!("OPC UA subscription end with error: {:?}", err);
        };
    });

    // Поток отправки сообщений в Redis
    let thread2 = thread::spawn(move || {
        let result = thread_to_redis(&redis_url, &redis_channel, &channel_rx);
        if let Err(error) = result {
            error!("Error in Redis publisher: {error}")
        };
    });
    thread1.join()?;
    thread2.join()?;
    Ok(())
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
        let msg = msg_opcua_to_redis(&msg)?;
        if let Some(msg) = msg {
            redis_hash.set(&msg.key(), msg)?;
        }
    }
    Ok(())
}

// -----------------------------------------------------------------------------

/// Запуск потоков Redis -> OPC UA
fn threads_redis_to_opcua(
    redis_url: &Url,
    redis_channel: &str,
    opcua_url: &Url,
) -> Result<(), Box<dyn Any + Send>> {
    let redis_url = redis_url.to_string();
    let redis_channel = redis_channel.to_string();
    let opcua_url = opcua_url.to_string();

    let (channel_tx, channel_rx) = mpsc::channel::<Messages>();

    // Поток получения новых сообщений из Redis
    let thread1 = thread::spawn(move || {
        let result = thread_from_redis(&redis_url, &redis_channel, &channel_tx);
        if let Err(err) = result {
            error!("Error in Redis subscription: {:?}", err);
        }
    });

    // Поток отправки новых данных в OPC UA
    let thread2 = thread::spawn(move || {
        let result = thread_to_opcua(&opcua_url, &channel_rx);
        if let Err(err) = result {
            error!("Error in OPC UA write: {:?}", err);
        }
    });

    thread1.join()?;
    thread2.join()?;
    Ok(())
}

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
    for msg in channel_rx {
        msg_redis_to_opcua(opcua_url, &msg)?;
    }
    Ok(())
}

// -----------------------------------------------------------------------------

/// Перечисляем теги OPC UA, на которые будем подписываться
fn create_nodes_for_subscription() -> Vec<NodeId> {
    // namespace
    const NS: u16 = 4;
    vec![
        NodeId::new(NS, Identifier::Numeric(2)),
        NodeId::new(NS, Identifier::Numeric(5)),
    ]
}

/// Подготавливаем полученные из OPC UA теги для отправки в Redis
fn msg_opcua_to_redis(
    msg: &ValueFromOpcUa,
) -> Result<Option<Messages>, Errors> {
    match msg.node_id.identifier {
        Identifier::Numeric(2) => {
            let value = convert::variant_to_i16(&msg.value)?;
            let ts = convert::datetime_to_chrono(&msg.source_timestamp)?;
            let msg =
                Messages::MotorState(types::SingleValue::new(value, Some(ts)));
            Ok(Some(msg))
        }
        Identifier::Numeric(5) => {
            let value = convert::variant_to_f64(&msg.value)?;
            let ts = convert::datetime_to_chrono(&msg.source_timestamp)?;
            let msg = Messages::SetpointRead(types::SingleValue::new(
                value,
                Some(ts),
            ));
            Ok(Some(msg))
        }
        _ => Ok(None),
    }
}

/// Подготавливаем полученные из Redis сообщения для отправки в OPC UA
fn msg_redis_to_opcua(opcua_url: &str, msg: &Messages) -> Result<(), Errors> {
    const NS: u16 = 4;

    match msg {
        Messages::CommandStart(_) => {
            let value = ValueToOpcUa {
                node_id: NodeId::new(NS, 3),
                value: convert::bool_to_variant(true),
            };
            write(opcua_url, value)?
        }
        Messages::CommandStop(_) => {
            let value = ValueToOpcUa {
                node_id: NodeId::new(NS, 4),
                value: convert::bool_to_variant(true),
            };
            write(opcua_url, value)?
        }
        Messages::SetpointChange(value) => {
            let value = ValueToOpcUa {
                node_id: NodeId::new(NS, 5),
                value: convert::f32_to_variant(value.value as f32),
            };
            write(opcua_url, value)?
        }
        _ => (),
    };
    Ok(())
}
