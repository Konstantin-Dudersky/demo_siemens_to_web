use tracing_loki::{url::ParseError, Error as LokiError};

#[derive(Debug)]
pub enum Errors {
    LokiError(String),
    ParseError(String),
}

impl From<LokiError> for Errors {
    fn from(value: LokiError) -> Self {
        Self::LokiError(value.to_string())
    }
}

impl From<ParseError> for Errors {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}
