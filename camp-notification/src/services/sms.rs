use super::SendResponse;
pub use crate::model::message::SmsMessage;
use crate::{model::message::MessageError, services::ServiceError};
use chrono::Utc;
use thiserror::Error;
use tonic::async_trait;
#[cfg(feature = "test_utils")]
use unimock::unimock;

#[derive(Error, Debug)]
pub enum SmsError {
    #[error("Sms sending failed: {0}")]
    Send(SmsMessage),

    #[error("Model error")]
    Model(#[from] MessageError),
}

#[cfg_attr(feature = "test_utils", unimock(api=MockEmailInner))]
#[async_trait]
pub trait Sms: Send + Sync + 'static {
    async fn send_sms(&self, msg: SmsMessage) -> Result<SendResponse, ServiceError>;
}

pub struct SmsFaker;

#[async_trait]
impl Sms for SmsFaker {
    async fn send_sms(&self, msg: SmsMessage) -> Result<SendResponse, ServiceError> {
        let random = rand::random::<u8>();
        if random % 9 == 0 {
            Ok(SendResponse {
                id: msg.id,
                timestamp: Utc::now(),
            })
        } else {
            Err(SmsError::Send(msg).into())
        }
    }
}

pub fn return_sms_mock() -> Box<dyn Sms> {
    Box::new(SmsFaker {})
}
