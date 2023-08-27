use std::sync::mpsc;
use std::thread;

use messages::{types, Messages};
use opcua::types::{Identifier, NodeId};

use opcua_client::{convert, subscribe};
use redis_client::RedisPubSync;

fn main() {
    let mut redis_hash =
        RedisPubSync::new("redis://127.0.0.1/", "opcua").unwrap();

    let (channel_tx, channel_rx) = mpsc::channel();

    let _ = thread::spawn(|| {
        subscribe("opc.tcp://192.168.101.180:4840/", channel_tx);
    });

    let thread2 = thread::spawn(move || {
        for msg in channel_rx {
            match msg.node_id {
                NodeId {
                    identifier: Identifier::Numeric(2),
                    ..
                } => {
                    let value = msg.value.as_ref().unwrap();
                    let value = convert::variant_to_i16(&value);
                    let ts = msg.source_timestamp.unwrap();
                    let ts = convert::datetime_to_chrono(ts).unwrap();
                    println!("{:?}", &msg);
                    let msg = Messages::IntValueFromOpcUa(
                        types::SimpleValue::new(value, Some(ts)),
                    );
                    println!("result msg: {}", msg);
                    redis_hash.set(&msg.key(), msg).unwrap();
                }
                _ => (),
            };
        }
    });

    thread2.join().unwrap();
}
