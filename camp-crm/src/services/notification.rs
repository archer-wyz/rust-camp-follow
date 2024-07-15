use anyhow::Result;
use camp_core::core_types::PinBoxTonicStream;
use camp_notification::pb::notification::{
    notification_client::NotificationClient, SendRequest, SendResponse,
};
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, transport::Channel};

use super::ServiceError;

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("gRPC return status: {0}")]
    GrpcStatus(#[from] tonic::Status),
}

#[cfg_attr(feature = "test_utils", unimock::unimock(api=MockNotification))]
#[async_trait]
pub trait Notification: Clone + Send + Sync + 'static {
    async fn notification(
        &self,
        request: mpsc::Receiver<SendRequest>,
    ) -> Result<PinBoxTonicStream<SendResponse>, ServiceError>;
}

#[derive(Clone)]
pub struct NotificationImpl {
    pub client: NotificationClient<Channel>,
}

#[async_trait]
impl Notification for NotificationImpl {
    async fn notification(
        &self,
        request: mpsc::Receiver<SendRequest>,
    ) -> Result<PinBoxTonicStream<SendResponse>, ServiceError> {
        let stream = ReceiverStream::new(request);
        let response = match self.client.clone().send(stream).await {
            Ok(response) => response.into_inner(),
            Err(status) => return Err(NotificationError::GrpcStatus(status).into()),
        };
        Ok(Box::pin(response))
    }
}

impl NotificationImpl {
    pub async fn try_new(url: &str) -> Result<Self> {
        let client = NotificationClient::connect(url.to_string()).await?;
        Ok(Self { client })
    }
}
