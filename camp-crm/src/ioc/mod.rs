use std::sync::Arc;

use crate::{
    abi::{CrmGrpc, CrmGrpcBuilder},
    config::AppConfig,
    services::{MetaDataV1, NotificationV1, UserStatV1},
    AppState, AppStateBuilder,
};
use anyhow::Result;
pub type CrmGrpcV1 = CrmGrpc<MetaDataV1, UserStatV1, NotificationV1>;

impl AppState<CrmGrpcV1> {
    pub async fn try_new() -> Result<Self> {
        let app_config = AppConfig::load()?;
        let metadata_service = MetaDataV1::try_new(&app_config.grpc.metadata).await?;
        let user_stat_service = UserStatV1::try_new(&app_config.grpc.user_stat).await?;
        let notification_service = NotificationV1::try_new(&app_config.grpc.notification).await?;
        let crm_grpc: CrmGrpcV1 = CrmGrpcBuilder::default()
            .metadata_service(Arc::new(Box::new(metadata_service)))
            .user_stat_service(Arc::new(Box::new(user_stat_service)))
            .notification_service(Arc::new(Box::new(notification_service)))
            .welcome_config(app_config.welcome.clone())
            .build()?;
        Ok(AppStateBuilder::default()
            .crm_grpc(crm_grpc)
            .app_config(app_config)
            .build()?)
    }
}
