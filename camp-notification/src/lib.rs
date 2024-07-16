use std::net::ToSocketAddrs as _;

use abi::{
    email::EmailGrpc, inapp::InAppGrpc, sms::SmsGrpc, NotificationGrpc, NotificationGrpcBuilder,
};
use anyhow::Result;
use config::AppConfig;
use derive_builder::Builder;
use services::{ServicesFactory, ServicesTypes};
use sqlx::PgPool;

use crate::pb::notification::notification_server::NotificationServer;

pub mod abi;
pub mod config;
#[cfg(feature = "test_utils")]
pub mod fake;
pub mod model;
pub mod pb;
pub mod services;

#[cfg(feature = "test_utils")]
mod test_utils {
    use anyhow::Result;
    use std::{env, path::Path};

    use super::AppState;
    use sqlx::{Executor, PgPool};
    use sqlx_db_tester::TestPg;

    use crate::config::AppConfig;

    #[allow(unused)]
    pub async fn common_test() -> Result<(TestPg, PgPool, AppConfig)> {
        let config = AppConfig::load()?;
        let db_url = config.db.to_db_url();
        let migrations = Path::new(&env::var("CARGO_MANIFEST_DIR")?).join("migrations");
        let tdb = TestPg::new(db_url, migrations);
        let pool = tdb.get_pool().await;

        let sql = include_str!("../fixtures/data.sql").split(';');
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");
        println!("execute ts done.");
        Ok((tdb, pool, config))
    }

    impl AppState {
        pub async fn new_for_test() -> Result<(TestPg, Self)> {
            let (tdb, pool, config) = common_test().await?;
            Self::new_inner(pool, config)
                .await
                .map(|app_state| (tdb, app_state))
        }
    }
}

#[derive(Clone, Builder)]
pub struct AppState {
    pub app_config: AppConfig,
    pub pool: PgPool,
    pub notification_grpc: NotificationGrpc,
}

impl AppState {
    async fn new_inner(pool: sqlx::PgPool, app_config: AppConfig) -> Result<Self> {
        let services_factory: Box<dyn ServicesFactory> = Box::new(
            services::ServicesFactoryImpl::new(ServicesTypes::AllUnimock, pool.clone()),
        );

        let notification_grpc = NotificationGrpcBuilder::default()
            .email(EmailGrpc(services_factory.email()))
            .inapp(InAppGrpc(services_factory.inapp()))
            .sms(SmsGrpc(services_factory.sms()))
            .build()?;

        Ok(Self {
            app_config,
            pool,
            notification_grpc,
        })
    }
    pub async fn new() -> Result<Self> {
        let app_config = AppConfig::load()?;
        let pool = PgPool::connect(&app_config.db.to_connect_url()).await?;
        Self::new_inner(pool, app_config).await
    }

    pub async fn grpc_run(&self) -> Result<()> {
        let addr = format!("[::1]:{:?}", self.app_config.grpc.port);
        println!("grpc server running on {:?}", addr);
        tonic::transport::Server::builder()
            .add_service(NotificationServer::new(self.notification_grpc.clone()))
            .serve(addr.to_socket_addrs()?.next().unwrap())
            .await?;
        println!("grpc quit");
        Ok(())
    }
}
