use anyhow::Result;
use std::net::ToSocketAddrs as _;

use abi::MetadataGRPC;
use camp_core::config::config_load;
use config::AppConfig;
use pb::metadata::metadata_server::MetadataServer;
use tonic::transport::Server;
use tracing::info;

pub mod abi;
pub mod config;
pub mod pb;

pub async fn start_metadata_grpc() -> Result<()> {
    let server = MetadataGRPC {};
    let config: AppConfig = config_load(vec![
        "../camp-metadata/metadata.yml".to_string(),
        "./camp-metadata/metadata.yml".to_string(),
        "./metadata.yml".to_string(),
        "/etc/config/metadata.yml".to_string(),
    ])?;
    let addr = format!("[::1]:{:?}", config.server.port);
    info!("metadata_grpc server is ready to running on {:?}", addr);
    Server::builder()
        .add_service(MetadataServer::new(server))
        .serve(addr.to_socket_addrs()?.next().unwrap())
        .await?;
    Ok(())
}
