use std::sync::{mpsc::Sender, Arc};

use opcua::{
    client::prelude::{
        DataChangeCallback, MonitoredItem, MonitoredItemService, Session,
        SubscriptionService,
    },
    sync::RwLock,
    types::{
        DateTime, MonitoredItemCreateRequest, NodeId, TimestampsToReturn,
        Variant,
    },
};
use tracing::{error, info, trace};

use crate::create_session::create_session;
use crate::errors::Errors;

#[derive(Debug)]
pub struct ValueFromOpcUa {
    pub node_id: NodeId,
    pub value: Option<Variant>,
    pub source_timestamp: Option<DateTime>,
    pub server_timestamp: Option<DateTime>,
}

pub fn subscribe(
    opcua_url: &str,
    channel_tx: Sender<ValueFromOpcUa>,
    nodes: Vec<NodeId>,
) -> Result<(), Errors> {
    let session = create_session(opcua_url)?;
    subscribe_(session.clone(), channel_tx, nodes)?;
    Session::run(session.clone());
    Ok(())
}

fn subscribe_(
    session: Arc<RwLock<Session>>,
    channel_tx: Sender<ValueFromOpcUa>,
    nodes: Vec<NodeId>,
) -> Result<(), Errors> {
    let session = session.write();
    let subscription_id = session.create_subscription(
        500.0,
        10,
        30,
        0,
        0,
        true,
        DataChangeCallback::new(move |changed_monitored_items| {
            for item in changed_monitored_items {
                trace!("new value from OPC UA subscription: {:?}", item);
                let val = prepare_item(item);
                for v in val {
                    match channel_tx.send(v) {
                        Ok(_) => (),
                        Err(err) => {
                            error!("{}", err.to_string());
                        }
                    }
                }
            }
        }),
    )?;
    info!("Created a subscription with id = {}", subscription_id);

    let items_to_create: Vec<MonitoredItemCreateRequest> =
        nodes.iter().map(|v| v.clone().into()).collect();
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
            source_timestamp: value.source_timestamp,
            server_timestamp: value.server_timestamp,
        });
    }
    res
}
