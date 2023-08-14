use redis::RedisError;

#[derive(Debug)]
pub enum Errors {
    RedisConnectionError(String),
    /// Поле не найдено в хеше
    FieldNotFoundError(String),
}

impl From<RedisError> for Errors {
    fn from(value: RedisError) -> Self {
        Errors::RedisConnectionError(value.to_string())
    }
}
