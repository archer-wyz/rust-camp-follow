use anyhow::Result;
use camp_notification::pb::notification::notification_client::NotificationClient;
use tonic::{async_trait, transport::Channel};

#[async_trait]
pub trait Notification: Clone + Send + Sync + 'static {}

#[derive(Clone)]
pub struct NotificationImpl {
    pub client: NotificationClient<Channel>,
}

#[async_trait]
impl Notification for NotificationImpl {}

impl NotificationImpl {
    pub async fn try_new(url: &str) -> Result<Self> {
        let client = NotificationClient::connect(url.to_string()).await?;
        Ok(Self { client })
    }
}
