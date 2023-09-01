use chrono::ParseError;
use opcua::client::prelude::StatusCode;

#[derive(Debug)]
pub enum Errors {
    ConvertDateTimeToChrono(String),
    ConvertFromVariant(String),
    ClientNotCreated,
    SessionNotCreated(String),
    StatusCode(String),
    ThreadSendError(String),
}

impl From<ParseError> for Errors {
    fn from(value: ParseError) -> Self {
        Self::ConvertDateTimeToChrono(value.to_string())
    }
}

impl From<StatusCode> for Errors {
    fn from(value: StatusCode) -> Self {
        Self::StatusCode(value.to_string())
    }
}
