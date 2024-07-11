use std::sync::Arc;

use derive_builder::Builder;
use tonic::{async_trait, Status};

use crate::{
    pb::crm::{
        crm_server::Crm, RecallRequest, RecallResponse, RemindRequest, RemindResponse,
        WelcomeRequest, WelcomeResponse,
    },
    services::{MetaData, Notification, UserStat},
};

use tonic::{Request, Response};

#[derive(Clone, Debug, Builder)]
pub struct CrmGrpc<T: MetaData, D: UserStat, U: Notification> {
    pub metadata_service: Arc<T>,
    pub user_stat_service: Arc<D>,
    pub notification_service: Arc<U>,
}

#[async_trait]
impl<T: MetaData, D: UserStat, U: Notification> Crm for CrmGrpc<T, D, U> {
    async fn welcome(
        &self,
        _request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
    /// last watched in X days, given them something to watch
    async fn recall(
        &self,
        _request: Request<RecallRequest>,
    ) -> std::result::Result<Response<RecallResponse>, Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }
    /// last watched in X days, and user still have unfinished contents
    async fn remind(
        &self,
        _request: Request<RemindRequest>,
    ) -> std::result::Result<Response<RemindResponse>, Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }
}
