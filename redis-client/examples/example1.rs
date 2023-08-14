use redis_client::RedisHash;
use tokio::main;

#[derive(Debug)]
enum Fields {
    Field1,
    Field2,
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[main]
async fn main() {
    let mut hash = RedisHash::new("redis://127.0.0.1/", "test_hash")
        .await
        .expect("Соединение не создано");

    hash.set(&Fields::Field1.to_string(), 10).await.unwrap();
    let read_field: u32 = hash.get("example_1_field").await.unwrap();

    assert_eq!(read_field, 10);
}
