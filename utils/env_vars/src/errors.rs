use dotenvy::Error as DotenvyError;
use envy::Error as EnvyError;

#[derive(Debug)]
pub enum Errors {
    EnvFileLoadError(String),
    DeserializeError(String),
}

impl From<DotenvyError> for Errors {
    fn from(value: DotenvyError) -> Self {
        Self::EnvFileLoadError(value.to_string())
    }
}

impl From<EnvyError> for Errors {
    fn from(value: EnvyError) -> Self {
        Self::DeserializeError(value.to_string())
    }
}
