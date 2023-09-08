use opcua::client::prelude::*;

use crate::create_session;
use crate::Errors;

pub struct ValueToOpcUa {
    pub node_id: NodeId,
    pub value: Option<Variant>,
}

pub fn write(opcua_url: &str, value: ValueToOpcUa) -> Result<(), Errors> {
    // TODO - если передавать в функцию сессию, вместо создания новой, через несколько циклов записи возникает ошибка, см ниже
    let session = create_session(opcua_url)?;
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

// {"message":"session:4 write() failed ServiceFault(ServiceFault { response_header: ResponseHeader { timestamp: DateTime { date_time: 2023-09-01T21:51:38.070951800Z }, request_handle: 11, service_result: IS_ERROR | BadUnexpectedError | BadResourceUnavailable | BadCommunicationError | BadIdentityTokenInvalid | BadIdentityTokenRejected | BadNonceInvalid | BadSessionIdInvalid, service_diagnostics: DiagnosticInfo { symbolic_id: None, namespace_uri: None, locale: None, localized_text: None, additional_info: None, inner_status_code: None, inner_diagnostic_info: None }, string_table: Some([]), additional_header: ExtensionObject { node_id: NodeId { namespace: 0, identifier: Numeric(0) }, body: None } } })","_spans":[],"_target":"opcua::client::session::session","_module_path":"opcua::client::session::session","_file":"/home/konstantin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/opcua-0.11.0/src/client/session/session.rs","_line":2380}
