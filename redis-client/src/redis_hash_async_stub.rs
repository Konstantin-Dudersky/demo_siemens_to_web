//! Реализация хеша redis для тестирования

use std::collections::HashMap;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str as deserialize, to_string as serialize};

use super::IRedisHashAsync;
use crate::errors::Errors;

pub struct RedisHashAsyncStub {
    collection: HashMap<String, String>,
}

impl RedisHashAsyncStub {
    pub fn new() -> Self {
        Self {
            collection: HashMap::new(),
        }
    }
}

#[async_trait]
impl IRedisHashAsync for RedisHashAsyncStub {
    async fn set<V>(&mut self, field: &str, value: V) -> Result<(), Errors>
    where
        V: Serialize + std::marker::Send,
    {
        let json = match serialize(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Errors::SerializeError(error.to_string()))
            }
        };
        self.collection.insert(field.to_string(), json);
        Ok(())
    }

    async fn get<V>(&mut self, field: &str) -> Result<V, Errors>
    where
        V: DeserializeOwned,
    {
        let json = self.collection.get(field);
        let json = match json {
            Some(value) => value,
            None => {
                let error = format!("Не найден ключ: {}", field);
                return Err(Errors::FieldNotFoundError(error.to_string()));
            }
        };
        match deserialize::<V>(&json) {
            Ok(value) => Ok(value),
            Err(error) => Err(Errors::DeserializeError(error.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        let mut redis_hash = RedisHashAsyncStub::new();

        redis_hash.set("field1", "field value").await.unwrap();
        let res: String = redis_hash.get("field1").await.unwrap();

        assert_eq!(res, "field value");
    }
}
