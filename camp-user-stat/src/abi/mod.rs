use crate::pb::user_stat::{user_stat_server::UserStat, QueryRequest, RawQueryRequest, User};
use crate::services::{IdQuery, Query, TimeQuery, UserStatService, UserStatVO};
use chrono::{DateTime, TimeZone, Utc};
use derive_builder::Builder;
use futures::Stream;
use prost_types::Timestamp;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::info;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseUserStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

impl From<UserStatVO> for User {
    fn from(value: UserStatVO) -> Self {
        User {
            name: value.name,
            email: value.email,
            started_but_not_finished: value
                .started_but_not_finished
                .map_or(vec![], |v| v.iter().map(|v| *v as _).collect()),
        }
    }
}

#[derive(Debug, Builder)]
pub struct UserStatGRPC<T: UserStatService> {
    pub service: Arc<T>,
}

impl<T: UserStatService> Clone for UserStatGRPC<T> {
    fn clone(&self) -> Self {
        UserStatGRPC {
            service: self.service.clone(),
        }
    }
}

impl<T> UserStatGRPC<T>
where
    T: UserStatService + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        UserStatGRPC { service }
    }
}

fn ts_to_utc(ts_option: &Option<Timestamp>) -> Option<DateTime<Utc>> {
    if let Some(ts) = ts_option {
        return Some(Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap());
    }
    None
}

impl From<QueryRequest> for Query {
    fn from(value: QueryRequest) -> Self {
        let mut timestamps = HashMap::new();
        let mut ids = HashMap::new();
        for (k, v) in value.timestamps {
            timestamps.insert(
                k,
                TimeQuery {
                    lower: ts_to_utc(&v.lower),
                    upper: ts_to_utc(&v.upper),
                },
            );
        }
        for (k, v) in value.ids {
            ids.insert(k, IdQuery { ids: v.ids });
        }
        Query { timestamps, ids }
    }
}

impl From<RawQueryRequest> for String {
    fn from(value: RawQueryRequest) -> Self {
        value.query
    }
}

#[tonic::async_trait]
impl<T> UserStat for UserStatGRPC<T>
where
    T: UserStatService + Send + Sync + 'static,
{
    type QueryStream = ResponseUserStream;

    type RawQueryStream = ResponseUserStream;

    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        let qr = request.into_inner();
        match self.service.query(qr).await {
            Ok(users) => {
                info!("user_stat query get users: {:?}", users.len());
                Ok(Response::new(Box::pin(futures::stream::iter(
                    users.into_iter().map(|v| Ok(v.into())),
                ))))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let rq = request.into_inner();
        match self.service.raw_query(rq).await {
            Ok(users) => Ok(Response::new(Box::pin(futures::stream::iter(
                users.into_iter().map(|v| Ok(v.into())),
            )))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
