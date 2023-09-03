use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{errors::Errors, load_env_vars};

pub fn load() -> Result<Config, Errors> {
    load_env_vars::load_env_vars()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub loki_url: Url,
    pub opcua_url: Url,
    pub redis_channel: String,
    pub redis_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            loki_url: Url::from_str("http://localhost:3100").unwrap(),
            opcua_url: Url::from_str("opc.tcp://192.168.101.180:4840/")
                .unwrap(),
            redis_channel: Default::default(),
            redis_url: Url::from_str("redis://localhost:6379").unwrap(),
        }
    }
}
