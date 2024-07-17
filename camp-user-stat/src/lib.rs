pub mod abi;
pub mod config;
pub mod fake;
pub mod ioc;
pub mod model;
pub mod pb;
pub mod services;

use crate::pb::user_stat::user_stat_server::{UserStat as UserStatGrpc, UserStatServer};
use anyhow::Result;
use config::AppConfig;
use derive_builder::Builder;
use std::net::ToSocketAddrs as _;
use tracing::info;

// AppState
// AppState's fields are grpc-service、axum-service、config or any other stateless components,
// which are very cheap to clone.
#[derive(Clone, Builder, Debug)]
pub struct AppState<T: UserStatGrpc> {
    pub user_stat_grpc: T,
    pub app_config: AppConfig,
}

impl<T> AppState<T>
where
    T: UserStatGrpc,
{
    // try_new
    // 1. read config file
    // 2. connect to db
    // 3. creat grpc service but not run
    // 4. TODO registry axum handler but not run
    pub async fn grpc_run(self) -> Result<()> {
        let addr = format!("[::1]:{:?}", self.app_config.grpc.port);
        info!("user_stat_grpc server is ready to running on {:?}", addr);
        tonic::transport::Server::builder()
            .add_service(UserStatServer::new(self.user_stat_grpc))
            .serve(addr.to_socket_addrs()?.next().unwrap())
            .await?;
        Ok(())
    }
}
