use redis::aio::Connection;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};

use crate::errors::Errors;

pub struct RedisHash {
    connection: Connection,
    hash_key: String,
}

impl RedisHash {
    pub async fn new(url: &str, hash_key: &str) -> Result<Self, Errors> {
        let client = redis::Client::open(url)?;
        let connection = client.get_async_connection().await?;
        Ok(Self {
            connection,
            hash_key: hash_key.to_string(),
        })
    }

    pub async fn set<V>(&mut self, field: &str, value: V) -> Result<(), Errors>
    where
        V: ToRedisArgs + Send + Sync,
    {
        self.connection.hset(&self.hash_key, field, value).await?;
        Ok(())
    }

    /// Читаем поле из хеша
    /// Если хеша не существует, или поля в хеше нет, возвращается ошибка с
    /// kind() == TypeError
    pub async fn get<V>(&mut self, field: &str) -> Result<V, Errors>
    where
        V: FromRedisValue + Send + Sync,
    {
        let value = self.connection.hget(&self.hash_key, field).await;
        match value {
            Ok(result) => Ok(result),
            Err(error) => match error.kind() {
                redis::ErrorKind::TypeError => {
                    return Err(Errors::FieldNotFoundError(error.to_string()))
                }
                _ => {
                    return Err(Errors::RedisConnectionError(error.to_string()))
                }
            },
        }
    }
}

// test ------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    async fn create_connection() -> RedisHash {
        RedisHash::new("redis://127.0.0.1/", "test_hash")
            .await
            .expect("Соединение не создано")
    }

    /// Функция устанавливает, считывает, и проверяет результат
    async fn set_and_get<V>(hash: &mut RedisHash, field: &str, value: V)
    where
        V: ToRedisArgs
            + FromRedisValue
            + Send
            + Sync
            + PartialEq
            + std::fmt::Debug,
    {
        hash.set(field, value).await.unwrap();
        let get_value: V = hash.get(field).await.unwrap();
        assert_eq!(get_value, get_value);
    }

    /// Проверяем создание подключения
    #[tokio::test]
    async fn test_new() {
        create_connection().await;
    }

    /// Записываем и читаем простые типы данных
    #[tokio::test]
    async fn set_get_base_types() {
        let mut hash = create_connection().await;

        set_and_get(&mut hash, "string_field", "string value".to_string())
            .await;
        set_and_get(&mut hash, "int_field", -10).await;
        set_and_get(&mut hash, "float_field", -1.23456).await;
        set_and_get(&mut hash, "bool_field", true).await;
    }

    /// Читаем из несуществующего хеша
    #[tokio::test]
    async fn get_from_notexist_hash() {
        let mut hash = RedisHash::new("redis://127.0.0.1/", "hash_no_created")
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
        let mut hash = create_connection().await;
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
}
