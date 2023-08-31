use std::sync::Arc;

use opcua::{client::prelude::*, sync::RwLock};

pub struct ValueToOpcUa {
    pub node_id: NodeId,
    pub value: Option<Variant>,
}

pub fn publish(session: Arc<RwLock<Session>>, value: ValueToOpcUa) {
    let session = session.read();
    let write_value = WriteValue {
        node_id: value.node_id,
        attribute_id: 13,
        index_range: UAString::default(),
        value: DataValue {
            value: value.value,
            status: None,
            source_timestamp: None,
            source_picoseconds: None,
            server_timestamp: None,
            server_picoseconds: None,
        },
    };
    session.write(&[write_value]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let (tx, rx) = std::sync::mpsc::channel::<String>();

        // publish("opc.tcp://192.168.101.180:4840/", tx);
    }
}
