use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub db: DbConfig,
    pub grpc: GrpcConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrpcConfig {
    pub port: u16,
}

impl DbConfig {
    pub fn to_connect_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }
    #[cfg(feature = "test_utils")]
    pub fn to_db_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let res = camp_core::config::config_load(vec![
            "../camp-notification/notification.yml".to_string(),
            "./camp-notification/notification.yml".to_string(),
            "./notification.yml".to_string(),
            "/etc/config/notification.yml".to_string(),
        ])?;
        Ok(res)
    }
}
