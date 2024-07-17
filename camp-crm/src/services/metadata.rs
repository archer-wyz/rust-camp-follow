use anyhow::Result;
use camp_metadata::pb::metadata::metadata_client::MetadataClient;
use camp_metadata::pb::metadata::{Content, MaterializeRequest};
use futures::{stream, Stream, StreamExt};
use std::collections::HashSet;
use thiserror::Error;
use tonic::{async_trait, transport::Channel};
use tracing::info;

use super::ServiceError;

#[derive(Debug, Error)]
pub enum MetaDataError {
    #[error("gRPC return status: {0}")]
    GrpcStatus(#[from] tonic::Status),
}

#[cfg_attr(feature = "test_utils", unimock::unimock(api=MockMetaData))]
#[async_trait]
pub trait MetaData: Send + Sync + 'static {
    async fn get_content(&self, ids: &[u32]) -> Result<Vec<Content>, ServiceError>;
}

#[derive(Clone)]
pub struct MetaDataImpl {
    pub client: MetadataClient<Channel>,
}

impl MetaDataImpl {
    pub async fn try_new(url: &str) -> Result<Self> {
        let client = MetadataClient::connect(url.to_string()).await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl MetaData for MetaDataImpl {
    async fn get_content(&self, ids: &[u32]) -> Result<Vec<Content>, ServiceError> {
        let mut response = match self
            .client
            .clone()
            .materialize(new_content_req_with_ids(ids))
            .await
        {
            Ok(response) => response.into_inner(),
            Err(status) => return Err(MetaDataError::GrpcStatus(status).into()),
        };

        info!("getting content from stream");
        let mut contents = Vec::new();
        while let Some(resp) = response.next().await {
            match resp {
                Ok(content) => contents.push(content),
                Err(e) => info!("Error while fetching content: {:?}", e),
            }
        }
        Ok(contents)
    }
}

fn new_content_req_with_ids(ids: &[u32]) -> impl Stream<Item = MaterializeRequest> {
    let req_ids: HashSet<MaterializeRequest> = ids
        .iter()
        .map(|id| MaterializeRequest { id: *id })
        .collect();
    stream::iter(req_ids)
}
