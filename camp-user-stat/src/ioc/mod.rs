use std::sync::Arc;

use crate::{
    abi::{UserStatGRPC, UserStatGRPCBuilder},
    config::AppConfig,
    services::{UserStatServiceImpl, UserStatServiceImplBuilder},
    AppState, AppStateBuilder,
};
use anyhow::Result;
use sqlx::PgPool;

pub async fn common() -> Result<(PgPool, AppConfig)> {
    let config = AppConfig::load()?;
    let sql_url = config.db.to_connect_url();
    let pg_pool = sqlx::PgPool::connect(&sql_url).await?;
    Ok((pg_pool, config))
}

pub type UserStatGRPCV1 = UserStatGRPC<UserStatServiceImpl>;

impl AppState<UserStatGRPCV1> {
    pub async fn new() -> Result<Self> {
        let (pool, config) = common().await?;
        let user_stat_service = Arc::new(
            UserStatServiceImplBuilder::default()
                .pool(pool.clone())
                .build()?,
        );
        let user_stat_grpc = UserStatGRPCBuilder::default()
            .service(user_stat_service.clone())
            .build()?;
        let app_state = AppStateBuilder::default()
            .user_stat_grpc(user_stat_grpc)
            .app_config(config)
            .build()?;
        Ok(app_state)
    }
}

#[cfg(feature = "test_utils")]
pub mod test_utils {
    use std::{env, path::Path, sync::Arc};

    use crate::{abi::UserStatGRPCBuilder, AppState, AppStateBuilder};

    use super::*;
    use sqlx::Executor;
    use sqlx_db_tester::TestPg;

    impl AppState<UserStatGRPCV1> {
        pub async fn new_for_test() -> Result<(TestPg, Self)> {
            let (tdb, pool, app_config) = common_test().await?;
            let user_stat_service = Arc::new(
                UserStatServiceImplBuilder::default()
                    .pool(pool.clone())
                    .build()?,
            );
            let user_stat_grpc = UserStatGRPCBuilder::<UserStatServiceImpl>::default()
                .service(user_stat_service.clone())
                .build()?;
            let app_state = AppStateBuilder::default()
                .user_stat_grpc(user_stat_grpc)
                .app_config(app_config)
                .build()?;
            Ok((tdb, app_state))
        }
    }

    pub async fn common_test() -> Result<(TestPg, PgPool, AppConfig)> {
        let config = AppConfig::load()?;
        let tb_url = config.db.to_connect_url();
        let (db_url, _) = tb_url.rsplit_once('/').expect("wrong db config");
        let (tdb, pool) = get_test_pool(Some(db_url)).await;

        let sql = include_str!("../../fixtures/data.sql").split(';');
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

    pub async fn get_test_pool(url: Option<&str>) -> (TestPg, PgPool) {
        let db_url = match url {
            Some(url) => url.to_string(),
            None => "postgres://postgres:postgres@localhost:5432".to_string(),
        };
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let p = Path::new(&dir).join("migrations");
        println!("migrations path: {:?}", p);
        let tdb = TestPg::new(db_url, p);
        let pool = tdb.get_pool().await;
        (tdb, pool)
    }
}
