use std::sync::Arc;

use opcua::{
    client::prelude::{ClientBuilder, IdentityToken, Session},
    sync::RwLock,
    types::{EndpointDescription, MessageSecurityMode, UserTokenPolicy},
};

pub fn create_session(opcua_url: &str) -> Arc<RwLock<Session>> {
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
    session
}
