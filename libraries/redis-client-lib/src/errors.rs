use std::fmt;

use redis::RedisError;

#[derive(Debug)]
pub enum Errors {
    RedisConnectionError(String),
    /// Поле не найдено в хеше
    FieldNotFoundError(String),
    /// Ошибка сериализации
    SerializeError(String),
    /// Ошибка десериализации
    DeserializeError(String),
    /// Ошибка отправки соообщения в канал mpsc
    SendThreadChannleError(String),
    /// Ошибка получения собщения из асинхронной подписки PubSub
    GetMessageError,
}

impl From<RedisError> for Errors {
    fn from(value: RedisError) -> Self {
        Errors::RedisConnectionError(value.to_string())
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
