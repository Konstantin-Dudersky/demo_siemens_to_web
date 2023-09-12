//! Реализация асинхронной публикации сообщений redis
//!
//! Значения публикуются в канале PubSub, и дополнительно сохраняются в хеше

use redis::aio::Connection;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str as deserialize, to_string as serialize};
use url::Url;

use crate::errors::Errors;

pub struct RedisPubAsync {
    connection: Connection,
    channel: String,
}

impl RedisPubAsync {
    pub async fn new(url: &Url, channel: &str) -> Result<Self, Errors> {
        let client = redis::Client::open(url.to_string())?;
        let connection = client.get_async_connection().await?;
        Ok(Self {
            connection,
            channel: channel.to_string(),
        })
    }
    pub async fn set<V>(&mut self, field: &str, value: V) -> Result<(), Errors>
    where
        V: Serialize + std::marker::Send,
    {
        let json = match serialize(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Errors::SerializeError(error.to_string()))
            }
        };
        self.connection.hset(&self.channel, field, &json).await?;
        self.connection.publish(&self.channel, &json).await?;
        Ok(())
    }

    /// Читаем поле из хеша
    /// Если хеша не существует, или поля в хеше нет, возвращается ошибка с
    /// kind() == TypeError
    pub async fn get<V>(&mut self, field: &str) -> Result<V, Errors>
    where
        V: DeserializeOwned,
    {
        let json: Result<String, redis::RedisError> =
            self.connection.hget(&self.channel, field).await;
        let json = match json {
            Ok(value) => value,
            Err(error) => match error.kind() {
                redis::ErrorKind::TypeError => {
                    return Err(Errors::FieldNotFoundError(error.to_string()))
                }
                _ => {
                    return Err(Errors::RedisConnectionError(error.to_string()))
                }
            },
        };
        match deserialize::<V>(&json) {
            Ok(value) => Ok(value),
            Err(error) => Err(Errors::DeserializeError(error.to_string())),
        }
    }

    /// Получение всех значений из хеша
    pub async fn hvals<V>(&mut self) -> Result<Vec<V>, Errors>
    where
        V: DeserializeOwned,
    {
        let values: Result<Vec<String>, redis::RedisError> =
            self.connection.hvals(&self.channel).await;
        let values = match values {
            Ok(values) => values,
            Err(error) => match error.kind() {
                redis::ErrorKind::TypeError => {
                    return Err(Errors::FieldNotFoundError(error.to_string()))
                }
                _ => {
                    return Err(Errors::RedisConnectionError(error.to_string()))
                }
            },
        };
        let mut result = vec![];
        for msg in values {
            let msg = match deserialize::<V>(&msg) {
                Ok(value) => value,
                Err(error) => {
                    return Err(Errors::DeserializeError(error.to_string()))
                }
            };
            result.push(msg);
        }
        Ok(result)
    }
}

// test ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use serde::Deserialize;

    async fn create_connection(channel: Option<&str>) -> RedisPubAsync {
        let channel = channel.unwrap_or("test_redis_pub_async");
        let url = Url::from_str("redis://127.0.0.1/").unwrap();
        RedisPubAsync::new(&url, channel)
            .await
            .expect("Соединение не создано")
    }

    /// Функция устанавливает, считывает, и проверяет результат
    async fn set_and_get<V>(hash: &mut RedisPubAsync, field: &str, value: V)
    where
        V: Serialize
            + DeserializeOwned
            + PartialEq
            + std::fmt::Debug
            + std::marker::Send,
    {
        hash.set(field, value).await.unwrap();
        let get_value: V = hash.get(field).await.unwrap();
        assert_eq!(get_value, get_value);
    }

    /// Проверяем создание подключения
    #[tokio::test]
    async fn test_new() {
        create_connection(None).await;
    }

    /// Записываем и читаем простые типы данных
    #[tokio::test]
    async fn set_get_base_types() {
        let mut hash = create_connection(None).await;

        set_and_get(&mut hash, "string_field", "string value".to_string())
            .await;
        set_and_get(&mut hash, "int_field", -10).await;
        set_and_get(&mut hash, "float_field", -1.23456).await;
        set_and_get(&mut hash, "bool_field", true).await;
    }

    /// Записываем и читаем структуру
    #[tokio::test]
    async fn set_get_struct() {
        #[derive(Serialize, PartialEq, Deserialize, Debug)]
        struct ChildStruct {
            memeber_in_child: String,
        }

        #[derive(Serialize, PartialEq, Deserialize, Debug)]
        struct TestStruct {
            member_str: String,
            member_int: i32,
            member_float: f64,
            member_bool: bool,
            child: ChildStruct,
        }

        let item1 = TestStruct {
            member_str: "member 1 value".to_string(),
            member_int: -77,
            member_float: -1.2345,
            member_bool: true,
            child: ChildStruct {
                memeber_in_child: "child field".to_string(),
            },
        };
        let mut hash = create_connection(None).await;
        set_and_get(&mut hash, "struct", item1).await;
    }

    /// Читаем из несуществующего хеша
    #[tokio::test]
    async fn get_from_notexist_hash() {
        let url = Url::from_str("redis://127.0.0.1").expect("");
        let mut hash = RedisPubAsync::new(&url, "hash_no_created")
            .await
            .expect("Соединение не создано");
        match hash.get::<i32>("no_created_field").await {
            Ok(value) => {
                panic!("Вернулось значение, хотя не должно было: {value}")
            }
            Err(error) => match error {
                Errors::FieldNotFoundError(_) => (),
                _ => panic!("Неправильный тип ошибки: {error:?}"),
            },
        };
    }

    /// Читаем из существующего хеша несуществующее поле
    #[tokio::test]
    async fn get_from_notexist_field() {
        let mut hash = create_connection(None).await;
        // создаем поле, чтобы убедиться, что хеш создан
        hash.set("field_for_hash_create", 10).await.unwrap();
        match hash.get::<i32>("no_created_field").await {
            Ok(value) => {
                panic!("Вернулось значение, хотя не должно было: {value}")
            }
            Err(error) => match error {
                Errors::FieldNotFoundError(_) => (),
                _ => panic!("Неправильный тип ошибки: {error:?}"),
            },
        };
    }

    /// Читаем все значения из хеша
    #[tokio::test]
    async fn hvals() {
        let mut hash = create_connection(Some("test_hvals_async")).await;
        hash.set("field1", "value1").await.unwrap();
        hash.set("field2", "value2").await.unwrap();
        let msgs = hash.hvals::<String>().await.unwrap();

        assert!(msgs.contains(&"value1".to_string()));
        assert!(msgs.contains(&"value2".to_string()));
        assert_eq!(msgs.len(), 2);
    }
}
