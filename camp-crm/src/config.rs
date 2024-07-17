use std::{env, path::Path};

use anyhow::Result;
use camp_core::config::config_load;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub welcome: WelcomeConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    pub port: u16,
    pub metadata: String,
    pub user_stat: String,
    pub notification: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WelcomeConfig {
    pub created_before_upper: usize,
    pub created_before_lower: usize,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let p = Path::new(&env::var("CARGO_MANIFEST_DIR")?).join("crm.yml");
        Ok(config_load(vec![
            "./camp-crm/crm.yml".to_string(),
            "../camp-crm/crm.yml".to_string(),
            "./crm.yml".to_string(),
            "/etc/config/crm.yml".to_string(),
            p.to_string_lossy().to_string(),
        ])?)
    }
}
