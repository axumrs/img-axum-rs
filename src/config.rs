use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct S3Config {
    /// 桶名称
    pub bucket: String,
    /// 地域
    pub region: String,
    /// 访问密钥
    pub access_key: String,
    /// 安全密钥
    pub secret_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PgConfig {
    pub dsn: String,
    pub max_connections: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImgConfig {
    pub expires_days: u8,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub s3: S3Config,
    pub pg: PgConfig,
    pub web: WebConfig,
    pub img: ImgConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }
}
