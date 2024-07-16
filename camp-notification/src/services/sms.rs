use super::SendResponse;
pub use crate::model::message::SmsMessage;
use crate::{model::message::MessageError, services::ServiceError};
use chrono::Utc;
use thiserror::Error;
use tonic::async_trait;
use unimock::{matching, unimock, MockFn, Unimock};

#[derive(Error, Debug)]
pub enum SmsError {
    #[error("Sms sending failed: {0}")]
    Send(String),

    #[error("Model error")]
    Model(#[from] MessageError),
}

#[unimock(api=MockSms)]
#[async_trait]
pub trait Sms: Send + Sync + 'static {
    async fn send_sms(&self, msg: SmsMessage) -> Result<SendResponse, ServiceError>;
}

pub fn return_sms_mock() -> Unimock {
    Unimock::new(MockSms::send_sms.each_call(matching!()).answers(&|_, msg| {
        let random = rand::random::<u8>();
        if random % 10 != 0 {
            Ok(SendResponse {
                id: msg.id,
                timestamp: Utc::now(),
            })
        } else {
            Err(SmsError::Send("Failed".to_string()).into())
        }
    }))
}
