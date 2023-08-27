//! Пример реализации публикации сообщений, используя асинхронный клиент

use serde::{Deserialize, Serialize};
use tokio::main;

use redis_client::RedisPubAsync;

#[derive(Debug)]
enum Tags {
    Field1,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct SimpleValue<T> {
    value: T,
}

impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[main]
async fn main() {
    let mut hash = RedisPubAsync::new("redis://127.0.0.1/", "test_hash")
        .await
        .expect("Соединение не создано");

    let value = SimpleValue { value: 10 };

    hash.set(&Tags::Field1.to_string(), &value).await.unwrap();
    let read_field: SimpleValue<i32> =
        hash.get(&Tags::Field1.to_string()).await.unwrap();

    assert_eq!(read_field, value);
}
