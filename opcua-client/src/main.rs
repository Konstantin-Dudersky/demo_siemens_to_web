use std::thread;
use std::{sync::mpsc, time::Duration};

use messages::{types, Messages};
use opcua::types::{Identifier, NodeId};
use tokio::main;
use tracing::{error, info, trace};

use logging::logging;
use opcua_client::{convert, create_session, publish, subscribe, ValueToOpcUa};
use redis_client::{start_redis_subscription, RedisPubSync};

#[main]
async fn main() {
    logging("opcua-client").await.unwrap();

    let redis_url = "redis://127.0.0.1/";
    let redis_channel = "opcua";
    const OPCUA_URL: &str = "opc.tcp://192.168.101.180:4840/";

    let mut redis_hash = RedisPubSync::new(redis_url, redis_channel).unwrap();

    let (channel_1_tx, channel_1_rx) = mpsc::channel();

    // Поток получения данных из OPC UA
    let thread1 = thread::spawn(move || loop {
        match subscribe(OPCUA_URL, channel_1_tx.clone()) {
            Ok(_) => (),
            Err(err) => {
                error!("OPC UA subscription end with error: {:?}", err);
            }
        };
        info!("Restarting OPC UA client...");
        std::thread::sleep(Duration::from_secs(2));
    });

    // Поток отправки новых данных в Redis
    let thread2 = thread::spawn(move || {
        for msg in channel_1_rx {
            match msg.node_id {
                NodeId {
                    identifier: Identifier::Numeric(2),
                    ..
                } => {
                    let value = convert::variant_to_i16(&msg.value).unwrap();
                    let ts = convert::datetime_to_chrono(&msg.source_timestamp)
                        .unwrap();
                    let msg = Messages::IntValueFromOpcUa(
                        types::SimpleValue::new(value, Some(ts)),
                    );
                    trace!("result msg: {}", msg);
                    redis_hash.set(&msg.key(), msg).unwrap();
                }
                _ => (),
            };
        }
    });

    let (channel_2_tx, channel_2_rx) = mpsc::channel::<Messages>();

    // Поток получения новых сообщений из Redis
    let _ = thread::spawn(|| {
        start_redis_subscription(redis_url, redis_channel, channel_2_tx);
    });

    // Поток отправки новых сообщений в OPC UA
    let _ = thread::spawn(|| {
        let session = create_session(OPCUA_URL).unwrap();
        for msg in channel_2_rx {
            match msg {
                Messages::IntValueToOpcUa(value) => {
                    let value = ValueToOpcUa {
                        node_id: NodeId::new(4, 2),
                        value: convert::i16_to_variant(value.value),
                    };
                    publish(session.clone(), value)
                }
                _ => (),
            }
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
