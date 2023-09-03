use serde::Deserialize;
use url::Url;

mod errors;
mod load_env_vars;

pub fn load() -> Result<Config, errors::Errors> {
    load_env_vars::load_env_vars()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub loki_url: Url,
    pub opcua_url: Url,
}