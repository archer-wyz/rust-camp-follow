use anyhow::Result;
use camp_core::config::config_load;
use camp_metadata::{
    abi::MetadataGRPC, config::AppConfig, pb::metadata::metadata_server::MetadataServer,
};
use std::net::ToSocketAddrs;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let server = MetadataGRPC {};
    let config: AppConfig = config_load(vec![
        "./metadata.yml".to_string(),
        "/etc/config/metadata.yml".to_string(),
    ])?;
    let addr = format!("[::1]:{:?}", config.server.port);
    Server::builder()
        .add_service(MetadataServer::new(server))
        .serve(addr.to_socket_addrs()?.next().unwrap())
        .await?;
    Ok(())
}
