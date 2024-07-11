use anyhow::Result;
use camp_core::config::config_load;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub grpc: GRPCConfig,
    pub http: HttpConfig,
    pub db: DBConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GRPCConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DBConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
}

impl DBConfig {
    pub fn to_connect_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config: Self = config_load(vec![
            "./user_stat.yml".to_string(),
            "/etc/config/user_stat.yml".to_string(),
        ])?;
        Ok(config)
    }
}
