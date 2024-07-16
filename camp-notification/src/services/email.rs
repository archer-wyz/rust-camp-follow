use super::{SendResponse, ServiceError};
pub use crate::model::message::{EmailMessage, Message, MessageError};
use anyhow::Result;
use chrono::Utc;
use thiserror::Error;
use tonic::async_trait;
use tracing::info;
use unimock::{matching, unimock, MockFn, Unimock};

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Email sending failed: {0}")]
    Send(String),
    #[error("Sqlx error")]
    Model(#[from] MessageError),
    #[error("Failed to save email message {0}")]
    FailedButSaved(Message),
}

#[unimock(api=MockEmailInner)]
#[async_trait]
pub trait EmailInner: Send + Sync + 'static {
    async fn send_email(
        &self,
        email: EmailMessage,
    ) -> Result<SendResponse, (EmailMessage, EmailError)>;
}

#[async_trait]
pub trait Email: Send + Sync + 'static {
    async fn send_email(&self, email: EmailMessage) -> Result<SendResponse, ServiceError>;
}

pub fn random_return_email(pool: sqlx::PgPool) -> Box<dyn Email> {
    let email = Unimock::new(MockEmailInner::send_email.each_call(matching!()).answers(
        &|_, email_msg| {
            let random = rand::random::<u8>();
            if random % 9 == 0 {
                Ok(SendResponse {
                    id: email_msg.id,
                    timestamp: Utc::now(),
                })
            } else {
                Err((email_msg, EmailError::Send("Failed".to_string())))
            }
        },
    ));

    let email_fail_over = EmailFailOver::<Unimock> {
        sender: email,
        pool,
    };
    Box::new(email_fail_over)
}

pub struct EmailFailOver<T: EmailInner> {
    pub sender: T,
    pub pool: sqlx::PgPool,
}

impl<T> EmailFailOver<T>
where
    T: EmailInner,
{
    pub fn new(sender: T, pool: sqlx::PgPool) -> Self {
        Self { sender, pool }
    }
}

#[async_trait]
impl<T: EmailInner> Email for EmailFailOver<T> {
    async fn send_email(&self, email: EmailMessage) -> Result<SendResponse, ServiceError> {
        //  try again if the first attempt fails
        let resp = match self.sender.send_email(email).await {
            Ok(resp) => Ok(resp),
            Err((msg, err)) => match err {
                EmailError::Send(_) => self.sender.send_email(msg).await,
                _ => Err((msg, err)),
            },
        };

        match resp {
            Ok(send_response) => Ok(send_response),
            Err((msg, _)) => match Message::insert_email(msg.clone(), &self.pool).await {
                Ok(msg) => Err(EmailError::FailedButSaved(msg).into()),
                Err(err) => {
                    info!("Failed to save email message: {:?}", err);
                    Err(EmailError::Model(err).into())
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn run_email_test<T: EmailInner>(
        email: T,
    ) -> Result<SendResponse, (EmailMessage, EmailError)> {
        email
            .send_email(EmailMessage {
                id: "123".to_string(),
                subject: "Test".to_string(),
                sender: "Test".to_string(),
                recipients: vec!["test1".to_string()],
                body: "Test".to_string(),
            })
            .await
    }

    #[tokio::test]
    async fn test() {
        let mock_email = Unimock::new(MockEmailInner::send_email.each_call(matching!()).answers(
            &|_, email_msg| {
                let random = rand::random::<u8>();
                if random % 2 == 0 {
                    Ok(SendResponse {
                        id: email_msg.id,
                        timestamp: Utc::now(),
                    })
                } else {
                    Err((email_msg, EmailError::Send("Failed".to_string())))
                }
            },
        ));
        for _ in 0..10 {
            let run = run_email_test(mock_email.clone()).await;
            println!("{:?}", run);
        }
    }
}
