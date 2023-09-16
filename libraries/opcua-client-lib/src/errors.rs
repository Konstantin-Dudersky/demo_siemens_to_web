use std::fmt;

use chrono::ParseError;
use opcua::client::prelude::StatusCode;

use redis_client_lib::Errors as RedisErrors;

#[derive(Debug)]
pub enum Errors {
    ConvertDateTimeToChrono(String),
    ConvertFromVariantError(String),
    ClientNotCreated,
    SessionNotCreated(String),
    StatusCode(String),
    ThreadSendError(String),
    RedisError(String),
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

impl From<RedisErrors> for Errors {
    fn from(value: RedisErrors) -> Self {
        Self::RedisError(value.to_string())
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
