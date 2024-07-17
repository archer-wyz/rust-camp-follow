use super::{SendResponse, ServiceError};
pub use crate::model::message::{InAppMessage, MessageError};
use chrono::Utc;
use thiserror::Error;
use tonic::async_trait;
#[cfg(feature = "test_utils")]
use unimock::unimock;

#[derive(Error, Debug)]
pub enum InAppError {
    #[error("InApp sending failed: {0}")]
    Send(InAppMessage),

    #[error("Model error")]
    Model(#[from] MessageError),
}

#[cfg_attr(feature = "test_utils", unimock(api=MockEmail))]
#[async_trait]
pub trait InApp: Send + Sync + 'static {
    async fn send_inapp(&self, msg: InAppMessage) -> Result<SendResponse, ServiceError>;
}

pub struct InAppFaker;

#[async_trait]
impl InApp for InAppFaker {
    async fn send_inapp(&self, msg: InAppMessage) -> Result<SendResponse, ServiceError> {
        let random = rand::random::<u8>();
        if random % 9 == 0 {
            Ok(SendResponse {
                id: msg.id,
                timestamp: Utc::now(),
            })
        } else {
            Err(ServiceError::InApp(InAppError::Send(msg)))
        }
    }
}

pub fn random_return_inapp() -> Box<dyn InApp> {
    Box::new(InAppFaker {})
}
