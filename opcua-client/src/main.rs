// use opcua::client::prelude::{
//     ClientBuilder, EndpointDescription, IdentityToken, MessageSecurityMode,
//     UserTokenPolicy,
// };
use opcua::client::prelude::*;

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
use opcua::sync::RwLock;
use std::sync::Arc;
fn subscribe(session: Arc<RwLock<Session>>) -> Result<(), StatusCode> {
    let session = session.read();
    // Creates a subscription with a data change callback
    let subscription_id = session.create_subscription(
        2000.0,
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
    let value = item.last_value().value.as_ref().unwrap();
    println!("{:?}", value);
    match value {
        Variant::Empty => todo!(),
        Variant::Boolean(_) => todo!(),
        Variant::SByte(_) => todo!(),
        Variant::Byte(_) => todo!(),
        Variant::Int16(_) => todo!(),
        Variant::UInt16(_) => todo!(),
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
    };
}
