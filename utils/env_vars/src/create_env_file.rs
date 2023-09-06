use std::fs::write;

use serde::Serialize;
use toml::to_string as serialize;

use crate::errors::Errors;

pub fn create_env_file<T>(filename: &str) -> Result<(), Errors>
where
    T: Default + Serialize,
{
    // TODO - вручную перевести названия переменных в SNAKE_UPPER_CASE
    let default = T::default();
    let s = serialize(&default)?;
    write(filename, s)?;
    Ok(())
}
