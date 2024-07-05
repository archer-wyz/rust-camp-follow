use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: Server,
    pub auth: Auth,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub pk: String,
}
