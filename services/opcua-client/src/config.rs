use messages::{types, Messages};
use opcua::types::{Identifier, NodeId};

use opcua_client_lib::{convert, write, Errors, ValueFromOpcUa, ValueToOpcUa};

/// Перечисляем теги OPC UA, на которые будем подписываться
pub fn create_nodes_for_subscription() -> Vec<NodeId> {
    // namespace
    const NS: u16 = 4;
    vec![
        NodeId::new(NS, Identifier::Numeric(2)),
        NodeId::new(NS, Identifier::Numeric(5)),
        NodeId::new(NS, Identifier::Numeric(6)),
    ]
}

/// Подготавливаем полученные из OPC UA теги для отправки в Redis
pub fn msg_opcua_to_redis(
    msg: &ValueFromOpcUa,
) -> Result<Option<Messages>, Errors> {
    match msg.node_id.identifier {
        Identifier::Numeric(2) => {
            let value = convert::variant_to_i16(&msg.value)?;
            let ts = convert::datetime_to_chrono(&msg.source_timestamp)?;
            let msg_content = types::SingleValue::new(value, Some(ts));
            let msg = Messages::MotorState(msg_content);
            Ok(Some(msg))
        }
        Identifier::Numeric(5) => {
            let value = convert::variant_to_f64(&msg.value)?;
            let ts = convert::datetime_to_chrono(&msg.source_timestamp)?;
            let msg_content = types::SingleValue::new(value, Some(ts));
            let msg = Messages::SetpointRead(msg_content);
            Ok(Some(msg))
        }
        Identifier::Numeric(6) => {
            let value = convert::variant_to_f64(&msg.value)?;
            let ts = convert::datetime_to_chrono(&msg.source_timestamp)?;
            let msg_content = types::SingleValue::new(value, Some(ts));
            let msg = Messages::Temperature(msg_content);
            Ok(Some(msg))
        }
        _ => Ok(None),
    }
}

/// Подготавливаем полученные из Redis сообщения для отправки в OPC UA
pub fn msg_redis_to_opcua(
    opcua_url: &str,
    msg: &Messages,
) -> Result<(), Errors> {
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
        Messages::SetpointWrite(value) => {
            let value = ValueToOpcUa {
                node_id: NodeId::new(NS, 5),
                value: convert::f32_to_variant(value.value as f32),
            };
            write(opcua_url, value)?
        }
        Messages::MotorState(_) => (),
        Messages::SetpointRead(_) => (),
        Messages::Temperature(_) => (),
    };
    Ok(())
}
