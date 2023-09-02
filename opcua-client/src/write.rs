use std::sync::Arc;

use opcua::{client::prelude::*, sync::RwLock};

use crate::Errors;

pub struct ValueToOpcUa {
    pub node_id: NodeId,
    pub value: Option<Variant>,
}

pub fn write(
    session: Arc<RwLock<Session>>,
    value: ValueToOpcUa,
) -> Result<(), Errors> {
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
    session.write(&[write_value])?;
    Ok(())
}
