use serde::de::DeserializeOwned;
use url::Url;

use redis_client_lib::RedisPubAsync;

use crate::Errors;

/// Загрузка всех сообщений из Redis
/// При подключении нового клиента сначала выдаются все данные, затем только
/// изменившиеся
pub async fn load_all_messages_from_hash<M>(
    redis_url: Url,
    redis_channel: String,
) -> Result<Vec<M>, Errors>
where
    M: DeserializeOwned,
{
    // create redis connection
    let redis = RedisPubAsync::new(&redis_url, &redis_channel).await;
    let mut redis = match redis {
        Ok(value) => value,
        Err(error) => {
            let error = error.to_string();
            return Err(Errors::GetAllFromRedis(error));
        }
    };
    // get all values from hash
    let msgs = redis.hvals().await;
    let msgs = match msgs {
        Ok(value) => value,
        Err(error) => {
            let error = error.to_string();
            return Err(Errors::GetAllFromRedis(error));
        }
    };
    Ok(msgs)
}
