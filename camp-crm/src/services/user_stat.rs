use anyhow::Result;
use camp_user_stat::pb::user_stat::user_stat_client::UserStatClient;
use tonic::{async_trait, transport::Channel};

#[async_trait]
pub trait UserStat: Clone + Send + Sync + 'static {}

#[derive(Clone)]
pub struct UserStatImpl {
    pub client: UserStatClient<Channel>,
}

#[async_trait]
impl UserStat for UserStatImpl {}

impl UserStatImpl {
    pub async fn try_new(url: &str) -> Result<Self> {
        let client = UserStatClient::connect(url.to_string()).await?;
        Ok(Self { client })
    }
}
