use anyhow::Result;
use camp_core::proto::utc_to_ts;
use camp_user_stat::pb::user_stat::{
    user_stat_client::UserStatClient, QueryRequest, TimeQuery, User,
};
use chrono::{DateTime, Utc};
use futures::Stream;
use std::{collections::HashMap, pin::Pin};
use thiserror::Error;
use tonic::{async_trait, transport::Channel, Request, Status};

use super::ServiceError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("gRPC return status: {0}")]
    GrpcStatus(#[from] tonic::Status),
}

#[cfg_attr(feature = "test_utils", unimock::unimock(api=MockUserStat))]
#[async_trait]
pub trait UserStat: Clone + Send + Sync + 'static {
    async fn get_new_user_stream(
        &self,
        lower: DateTime<Utc>,
        upper: DateTime<Utc>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>, ServiceError>;
}

#[derive(Clone)]
pub struct UserStatImpl {
    pub client: UserStatClient<Channel>,
}

#[async_trait]
impl UserStat for UserStatImpl {
    async fn get_new_user_stream(
        &self,
        lower: DateTime<Utc>,
        upper: DateTime<Utc>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>, ServiceError> {
        let request = new_user_req_created_between(lower, upper);
        let response = match self.client.clone().query(request).await {
            Ok(response) => response.into_inner(),
            Err(status) => return Err(UserError::GrpcStatus(status).into()),
        };
        Ok(Box::pin(response))
    }
}

impl UserStatImpl {
    pub async fn try_new(url: &str) -> Result<Self> {
        let client = UserStatClient::connect(url.to_string()).await?;
        Ok(Self { client })
    }
}

fn new_user_req_created_between(
    lower: DateTime<Utc>,
    upper: DateTime<Utc>,
) -> Request<QueryRequest> {
    Request::new(QueryRequest {
        timestamps: vec![(
            "created_at".to_string(),
            TimeQuery {
                lower: Some(utc_to_ts(lower)),
                upper: Some(utc_to_ts(upper)),
            },
        )]
        .into_iter()
        .collect(),
        ids: HashMap::new(),
    })
}