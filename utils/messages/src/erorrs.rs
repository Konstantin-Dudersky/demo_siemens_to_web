#[derive(Debug)]
pub enum Errors {
    DeserializationError(String),
    SerializationError(String),
}
