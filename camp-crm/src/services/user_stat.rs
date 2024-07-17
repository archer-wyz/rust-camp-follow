use anyhow::Result;
use camp_core::proto::utc_to_ts;
use camp_user_stat::pb::user_stat::{
    user_stat_client::UserStatClient, QueryRequest, RawQueryRequest, TimeQuery, User,
};
use chrono::{DateTime, Utc};
use futures::Stream;
use std::{collections::HashMap, pin::Pin};
use thiserror::Error;
use tonic::{async_trait, transport::Channel, Request, Status};
use tracing::info;

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

    async fn get_lasted_visit_before_stream(
        &self,
        lasted_visited_before: DateTime<Utc>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>, ServiceError>;

    async fn get_lasted_visit_but_not_finished(
        &self,
        lasted_visited_before: DateTime<Utc>,
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

    async fn get_lasted_visit_before_stream(
        &self,
        lasted_visited_before: DateTime<Utc>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>, ServiceError> {
        let request = new_user_req_last_visite(lasted_visited_before);
        let response = match self.client.clone().query(request).await {
            Ok(response) => response.into_inner(),
            Err(status) => return Err(UserError::GrpcStatus(status).into()),
        };
        Ok(Box::pin(response))
    }

    async fn get_lasted_visit_but_not_finished(
        &self,
        lasted_visited_before: DateTime<Utc>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>, ServiceError> {
        let raw_query = format!("
            SELECT name, email, started_but_not_finished from user_stats WHERE last_visited_at <= '{:?}'
        ", lasted_visited_before);
        info!("raw_query: {}", raw_query);
        let req = Request::new(RawQueryRequest { query: raw_query });
        let response = match self.client.clone().raw_query(req).await {
            Ok(stream) => stream.into_inner(),
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

fn new_user_req_last_visite(time: DateTime<Utc>) -> Request<QueryRequest> {
    Request::new(QueryRequest {
        timestamps: vec![(
            "last_visited_at".to_string(),
            TimeQuery {
                upper: Some(utc_to_ts(time)),
                lower: None,
            },
        )]
        .into_iter()
        .collect(),
        ids: HashMap::new(),
    })
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
