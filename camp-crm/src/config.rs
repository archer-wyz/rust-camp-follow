use std::{env, path::Path};

use anyhow::Result;
use camp_core::config::config_load;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    pub port: u16,
    pub metadata: String,
    pub user_stat: String,
    pub notification: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let p = Path::new(&env::var("CARGO_MANIFEST_DIR")?).join("crm.yml");
        Ok(config_load(vec![
            "./crm.yml".to_string(),
            "/etc/config/crm.yml".to_string(),
            p.to_string_lossy().to_string(),
        ])?)
    }
}
