use std::sync::Arc;

use opcua::{
    client::prelude::{ClientBuilder, IdentityToken, Session},
    sync::RwLock,
    types::{EndpointDescription, MessageSecurityMode, UserTokenPolicy},
};
use tracing::info;

use crate::errors::Errors;

pub fn create_session(opcua_url: &str) -> Result<Arc<RwLock<Session>>, Errors> {
    let client = ClientBuilder::new()
        .application_name("My First Client")
        .application_uri("urn:MyFirstClient")
        .create_sample_keypair(true)
        .trust_server_certs(true)
        .session_retry_limit(0)
        .client();
    let mut client = match client {
        Some(value) => value,
        None => return Err(Errors::ClientNotCreated),
    };
    let endpoint: EndpointDescription = (
        opcua_url,
        "None",
        MessageSecurityMode::None,
        UserTokenPolicy::anonymous(),
    )
        .into();
    let session =
        client.connect_to_endpoint(endpoint, IdentityToken::Anonymous);
    let session = match session {
        Ok(value) => value,
        Err(err) => {
            let msg = err.to_string();
            return Err(Errors::SessionNotCreated(msg));
        }
    };
    info!("OPC UA session created");
    Ok(session)
}
