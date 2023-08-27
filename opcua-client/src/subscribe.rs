use std::sync::{mpsc::Sender, Arc};

use chrono::{DateTime, FixedOffset};
use opcua::{
    client::prelude::{
        ClientBuilder, DataChangeCallback, IdentityToken, MonitoredItem,
        MonitoredItemService, Session, SubscriptionService,
    },
    sync::RwLock,
    types::{
        EndpointDescription, MessageSecurityMode, MonitoredItemCreateRequest,
        NodeId, StatusCode, TimestampsToReturn, UserTokenPolicy, Variant,
    },
};

use crate::convert;

#[derive(Debug)]
pub struct ValueFromOpcUa {
    pub node_id: NodeId,
    pub value: Option<Variant>,
    pub source_timestamp: Option<DateTime<FixedOffset>>,
    pub server_timestamp: Option<DateTime<FixedOffset>>,
}

pub fn subscribe(opcua_url: &str, channel_tx: Sender<ValueFromOpcUa>) {
    let mut client = ClientBuilder::new()
        .application_name("My First Client")
        .application_uri("urn:MyFirstClient")
        .create_sample_keypair(true)
        .trust_server_certs(true)
        .session_retry_limit(0)
        .client()
        .unwrap();

    let endpoint: EndpointDescription = (
        opcua_url,
        "None",
        MessageSecurityMode::None,
        UserTokenPolicy::anonymous(),
    )
        .into();

    let session = client
        .connect_to_endpoint(endpoint, IdentityToken::Anonymous)
        .unwrap();
    subscribe_(session.clone(), channel_tx).unwrap();
    Session::run(session.clone());
}

fn subscribe_(
    session: Arc<RwLock<Session>>,
    tx: Sender<ValueFromOpcUa>,
) -> Result<(), StatusCode> {
    let session = session.write();
    let subscription_id = session.create_subscription(
        1000.0,
        10,
        30,
        0,
        0,
        true,
        DataChangeCallback::new(move |changed_monitored_items| {
            for item in changed_monitored_items {
                let val = prepare_item(item);
                for v in val {
                    tx.send(v).unwrap()
                }
            }
        }),
    )?;
    println!("Created a subscription with id = {}", subscription_id);

    let items_to_create: Vec<MonitoredItemCreateRequest> =
        [2].iter().map(|v| NodeId::new(4, *v).into()).collect();
    let _ = session.create_monitored_items(
        subscription_id,
        TimestampsToReturn::Both,
        &items_to_create,
    )?;

    Ok(())
}

fn prepare_item(item: &MonitoredItem) -> Vec<ValueFromOpcUa> {
    let node_id = item.item_to_monitor().node_id.clone();
    let mut res = vec![];
    for value in item.values() {
        res.push(ValueFromOpcUa {
            node_id: node_id.clone(),
            value: value.value.clone(),
            source_timestamp: convert::datetime_to_chrono(
                value.source_timestamp,
            ),
            server_timestamp: convert::datetime_to_chrono(
                value.server_timestamp,
            ),
        });
    }
    res
}
