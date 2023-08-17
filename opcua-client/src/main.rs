// use opcua::client::prelude::{
//     ClientBuilder, EndpointDescription, IdentityToken, MessageSecurityMode,
//     UserTokenPolicy,
// };

use std::sync::Arc;

use opcua::client::prelude::*;
use opcua::sync::RwLock;

use messages::{Messages, SimpleValue};

fn main() {
    let mut client = ClientBuilder::new()
        .application_name("My First Client")
        .application_uri("urn:MyFirstClient")
        .create_sample_keypair(true)
        .trust_server_certs(true)
        .session_retry_limit(0)
        .client()
        .unwrap();

    let endpoint: EndpointDescription = (
        "opc.tcp://192.168.101.180:4840/",
        "None",
        MessageSecurityMode::None,
        UserTokenPolicy::anonymous(),
    )
        .into();

    let session = client
        .connect_to_endpoint(endpoint, IdentityToken::Anonymous)
        .unwrap();
    subscribe(session.clone()).unwrap();
    let _ = Session::run(session);
}

fn subscribe(session: Arc<RwLock<Session>>) -> Result<(), StatusCode> {
    let session = session.write();
    let subscription_id = session.create_subscription(
        1000.0,
        10,
        30,
        0,
        0,
        true,
        DataChangeCallback::new(|changed_monitored_items| {
            println!("Data change from server:");
            changed_monitored_items
                .iter()
                .for_each(|item| process_item(item));
        }),
    )?;
    println!("Created a subscription with id = {}", subscription_id);

    // Create some monitored items
    let items_to_create: Vec<MonitoredItemCreateRequest> =
        [2].iter().map(|v| NodeId::new(4, *v).into()).collect();
    let _ = session.create_monitored_items(
        subscription_id,
        TimestampsToReturn::Both,
        &items_to_create,
    )?;

    Ok(())
}

fn process_item(item: &MonitoredItem) {
    match item.id() {
        1 => {
            let value = item.last_value().value.as_ref().unwrap();
            let value = convert_opc_i16(value);
            println!("{:?}", value);
        }
        _ => (),
    }
}

fn convert_opc_i16(opc: &Variant) -> i16 {
    match opc {
        Variant::Empty => todo!(),
        Variant::Boolean(_) => todo!(),
        Variant::SByte(_) => todo!(),
        Variant::Byte(_) => todo!(),
        Variant::Int16(value) => *value,
        Variant::UInt16(value) => *value as i16,
        Variant::Int32(_) => todo!(),
        Variant::UInt32(_) => todo!(),
        Variant::Int64(_) => todo!(),
        Variant::UInt64(_) => todo!(),
        Variant::Float(_) => todo!(),
        Variant::Double(_) => todo!(),
        Variant::String(_) => todo!(),
        Variant::DateTime(_) => todo!(),
        Variant::Guid(_) => todo!(),
        Variant::StatusCode(_) => todo!(),
        Variant::ByteString(_) => todo!(),
        Variant::XmlElement(_) => todo!(),
        Variant::QualifiedName(_) => todo!(),
        Variant::LocalizedText(_) => todo!(),
        Variant::NodeId(_) => todo!(),
        Variant::ExpandedNodeId(_) => todo!(),
        Variant::ExtensionObject(_) => todo!(),
        Variant::Variant(_) => todo!(),
        Variant::DataValue(_) => todo!(),
        Variant::Diagnostics(_) => todo!(),
        Variant::Array(_) => todo!(),
    }
}

// fn main1() {
//     let test_arr = Messages::Msg2(SimpleValue { value: 32f64 });

//     let str = serde_json::to_string(&test_arr).unwrap();

//     let deser: Messages = serde_json::from_str(&str).unwrap();

//     match &deser {
//         Messages::IntValueFromOpcUa(value) => println!("{}", value.value),
//     }

//     println!("{:?}", str);
//     println!("{:?}", deser);
// }

use std::time::Duration;

async fn test_async() {
    std::thread::sleep(Duration::from_millis(2000));
}
