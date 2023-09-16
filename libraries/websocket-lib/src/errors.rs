use std::io::Error as StdIoError;
use tokio_tungstenite::tungstenite::Error as WsError;

// use messages::Errors as MessagesError;
use redis_client_lib::Errors as RedisErrors;

#[derive(Debug)]
pub enum Errors {
    GetAllFromRedis(String),
    SendToWsError(String),
    MessagesError(String),
    WsError(WsError),
    BindToPortError(StdIoError),
    RedisError(RedisErrors),
    TokioTaskHandleError,
}

// impl From<MessagesError> for Errors {
//     fn from(value: MessagesError) -> Self {
//         let value = format!("{:?}", value);
//         Self::MessagesError(value)
//     }
// }

impl From<WsError> for Errors {
    fn from(value: WsError) -> Self {
        Self::WsError(value)
    }
}

impl From<RedisErrors> for Errors {
    fn from(value: RedisErrors) -> Self {
        Self::RedisError(value)
    }
}
