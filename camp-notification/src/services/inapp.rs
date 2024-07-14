use super::{SendResponse, ServiceError};
pub use crate::model::message::{InAppMessage, MessageError};
use chrono::Utc;
use thiserror::Error;
use tonic::async_trait;
use unimock::{matching, unimock, MockFn, Unimock};

#[derive(Error, Debug)]
pub enum InAppError {
    #[error("InApp sending failed: {0}")]
    Send(String),

    #[error("Model error")]
    Model(#[from] MessageError),
}

#[unimock(api=MockEmail)]
#[async_trait]
pub trait InApp: Send + Sync + 'static {
    async fn send_inapp(&self, msg: InAppMessage) -> Result<SendResponse, ServiceError>;
}

pub fn random_return_inapp() -> Unimock {
    Unimock::new(
        MockEmail::send_inapp
            .each_call(matching!())
            .answers(&|_, msg| {
                let random = rand::random::<u8>();
                if random % 2 == 0 {
                    Ok(SendResponse {
                        id: msg.id,
                        timestamp: Utc::now(),
                    })
                } else {
                    Err(ServiceError::InApp(InAppError::Send("Failed".to_string())))
                }
            }),
    )
}
