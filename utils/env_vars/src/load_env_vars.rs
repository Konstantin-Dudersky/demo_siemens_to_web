use std::fmt::Debug;

use dotenvy::dotenv;
use envy::from_env;
use serde::de::DeserializeOwned;
use tracing::{error, info};

use crate::errors::Errors;

/// Загрузить переменные окружения, в т.ч. из файла .env
pub fn load_env_vars<T>() -> Result<T, Errors>
where
    T: DeserializeOwned + Debug,
{
    let vars = _load_env_vars();
    match &vars {
        Ok(value) => {
            info!("Загружен файл с переменными: {:?}", value);
        }
        Err(err) => {
            error!("Ошибка загрузки переменных среды: {:?}", err);
        }
    };
    vars
}

fn _load_env_vars<T>() -> Result<T, Errors>
where
    T: DeserializeOwned + Debug,
{
    // загружаем из файла .env
    dotenv()?;
    // десериализуем в структуру
    let vars = from_env::<T>()?;
    Ok(vars)
}
