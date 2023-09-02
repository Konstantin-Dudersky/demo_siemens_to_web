//! Реализация синхронной публикации сообщений redis

use redis::Commands;
use redis::Connection;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str as deserialize, to_string as serialize};
use tracing::trace;

use crate::errors::Errors;

pub struct RedisPubSync {
    connection: Connection,
    channel: String,
}

impl RedisPubSync {
    pub fn new(url: &str, channel: &str) -> Result<Self, Errors> {
        let client = redis::Client::open(url)?;
        let connection = client.get_connection()?;
        Ok(Self {
            connection: connection,
            channel: channel.to_string(),
        })
    }

    pub fn set<V>(&mut self, field: &str, value: V) -> Result<(), Errors>
    where
        V: Serialize,
    {
        let json = match serialize(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Errors::SerializeError(error.to_string()))
            }
        };
        trace!("Send message to Redis: {json}");
        self.connection.hset(&self.channel, field, &json)?;
        self.connection.publish(&self.channel, &json)?;
        Ok(())
    }

    /// Читаем поле из хеша
    /// Если хеша не существует, или поля в хеше нет, возвращается ошибка с
    /// kind() == TypeError
    pub fn get<V>(&mut self, field: &str) -> Result<V, Errors>
    where
        V: DeserializeOwned,
    {
        let json: Result<String, redis::RedisError> =
            self.connection.hget(&self.channel, field);
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
}

// test ------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    fn create_connection() -> RedisPubSync {
        RedisPubSync::new("redis://127.0.0.1/", "test_hash")
            .expect("Соединение не создано")
    }

    /// Функция устанавливает, считывает, и проверяет результат
    fn set_and_get<V>(hash: &mut RedisPubSync, field: &str, value: V)
    where
        V: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        hash.set(field, value).unwrap();
        let get_value: V = hash.get(field).unwrap();
        assert_eq!(get_value, get_value);
    }

    /// Проверяем создание подключения
    #[test]
    fn test_new() {
        create_connection();
    }

    /// Записываем и читаем простые типы данных
    #[test]
    fn set_get_base_types() {
        let mut hash = create_connection();

        set_and_get(&mut hash, "string_field", "string value".to_string());
        set_and_get(&mut hash, "int_field", -10);
        set_and_get(&mut hash, "float_field", -1.23456);
        set_and_get(&mut hash, "bool_field", true);
    }

    /// Записываем и читаем структуру
    #[test]
    fn set_get_struct() {
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
        let mut hash = create_connection();
        set_and_get(&mut hash, "struct", item1);
    }

    /// Читаем из несуществующего хеша
    #[test]
    fn get_from_notexist_hash() {
        let mut hash =
            RedisPubSync::new("redis://127.0.0.1/", "hash_no_created")
                .expect("Соединение не создано");
        match hash.get::<i32>("no_created_field") {
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
    #[test]
    fn get_from_notexist_field() {
        let mut hash = create_connection();
        // создаем поле, чтобы убедиться, что хеш создан
        hash.set("field_for_hash_create", 10).unwrap();
        match hash.get::<i32>("no_created_field") {
            Ok(value) => {
                panic!("Вернулось значение, хотя не должно было: {value}")
            }
            Err(error) => match error {
                Errors::FieldNotFoundError(_) => (),
                _ => panic!("Неправильный тип ошибки: {error:?}"),
            },
        };
    }
}
