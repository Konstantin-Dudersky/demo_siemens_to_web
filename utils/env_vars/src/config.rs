use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{errors::Errors, load_env_vars};

pub fn load() -> Result<Config, Errors> {
    load_env_vars::load_env_vars()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_ws_port: u16,

    // Для статической проверки SQL-запросов, не переименовывать
    pub database_url: Url,

    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,

    pub grafana_port: u16,

    pub loki_port: u16,
    pub loki_url: Url,

    pub opcua_url: Url,

    pub redis_channel: String,
    pub redis_port: u16,
    pub redis_url: Url,

    pub webapp_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_ws_port: 8081,

            loki_port: 3100,
            loki_url: Url::from_str("http://localhost:3100").unwrap(),
            opcua_url: Url::from_str("opc.tcp://192.168.101.180:4840/")
                .unwrap(),

            database_url: Url::from_str(
                "postgres://postgres:password@localhost:5432/db_data_test",
            )
            .unwrap(),

            db_host: "localhost".to_string(),
            db_port: 5432,
            db_user: "postgres".to_string(),
            db_password: "postgres".to_string(),

            grafana_port: 3000,

            redis_channel: Default::default(),
            redis_port: 6379,
            redis_url: Url::from_str("redis://localhost:6379").unwrap(),

            webapp_port: 8090,
        }
    }
}
