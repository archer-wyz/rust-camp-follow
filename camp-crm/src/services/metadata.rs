use anyhow::Result;
use camp_metadata::pb::metadata::metadata_client::MetadataClient;
use tonic::{async_trait, transport::Channel};

#[async_trait]
pub trait MetaData: Clone + Send + Sync + 'static {}

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

impl MetaData for MetaDataImpl {}
