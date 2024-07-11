use std::net::ToSocketAddrs as _;

use anyhow::Result;
use config::AppConfig;
use derive_builder::Builder;
use pb::crm::crm_server::Crm;

use crate::pb::crm::crm_server::CrmServer;

pub mod abi;
pub mod config;
pub mod ioc;
pub mod pb;
pub mod services;

#[derive(Clone, Builder, Debug)]
pub struct AppState<T: Crm> {
    pub crm_grpc: T,
    pub app_config: AppConfig,
}

impl<T> AppState<T>
where
    T: Crm,
{
    pub async fn grpc_run(self) -> Result<()> {
        let addr = format!("[::1]:{:?}", self.app_config.grpc.port);
        println!("grpc server running on {:?}", addr);
        tonic::transport::Server::builder()
            .add_service(CrmServer::new(self.crm_grpc))
            .serve(addr.to_socket_addrs()?.next().unwrap())
            .await?;
        println!("grpc quit");
        Ok(())
    }
}
