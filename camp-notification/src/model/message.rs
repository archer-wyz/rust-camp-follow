use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor, Type};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl Display for EmailMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EmailMessage {{ id: {}, subject: {}, sender: {}, recipients: {:?}, body: {} }}",
            self.id, self.subject, self.sender, self.recipients, self.body
        )
    }
}

#[derive(Clone, Debug)]
pub struct EmailMessage {
    pub id: String,
    pub subject: String,
    pub sender: String,
    pub recipients: Vec<String>,
    pub body: String,
}

impl From<EmailMessage> for Message {
    fn from(email: EmailMessage) -> Self {
        let now = Utc::now();
        Message {
            id: email.id,
            r#type: MessageType::Email,
            sender: email.sender,
            body: email.body,
            subject: Some(email.subject),
            recipients: Some(email.recipients),
            times: 1,
            device_id: None,
            title: None,
            created_at: now,
            updated_at: now,
        }
    }
}

pub struct SmsMessage {
    pub id: String,
    pub sender: String,
    pub body: String,
    pub recipients: Vec<String>,
}

impl From<SmsMessage> for Message {
    fn from(value: SmsMessage) -> Self {
        let now = Utc::now();
        Message {
            id: value.id,
            r#type: MessageType::Sms,
            sender: value.sender,
            body: value.body,
            subject: None,
            recipients: Some(value.recipients),
            times: 1,
            device_id: None,
            title: None,
            created_at: now,
            updated_at: now,
        }
    }
}

pub struct InAppMessage {
    pub id: String,
    pub sender: String,
    pub body: String,
    pub device_id: String,
    pub title: String,
}

impl From<InAppMessage> for Message {
    fn from(value: InAppMessage) -> Self {
        let now = Utc::now();
        Message {
            id: value.id,
            r#type: MessageType::Inapp,
            sender: value.sender,
            body: value.body,
            subject: None,
            recipients: None,
            times: 1,
            device_id: Some(value.device_id),
            title: Some(value.title),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "message_type", rename_all = "snake_case")]
pub enum MessageType {
    Email,
    Sms,
    Inapp,
    Unknown,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Email => write!(f, "email"),
            MessageType::Sms => write!(f, "sms"),
            MessageType::Inapp => write!(f, "inapp"),
            MessageType::Unknown => write!(f, "unknown"),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Message {{ id: {}, type: {}, sender: {}, body: {}, created_at: {}, updated_at: {}, subject: {:?}, recipients: {:?}, device_id: {:?}, title: {:?}, times: {} }}",
            self.id, self.r#type, self.sender, self.body, self.created_at, self.updated_at, self.subject, self.recipients, self.device_id, self.title, self.times
        )
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub r#type: MessageType,
    pub sender: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub subject: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub device_id: Option<String>,
    pub title: Option<String>,
    pub times: i32,
}

impl<'a> Message {
    pub async fn insert_email<T>(msg: EmailMessage, executor: T) -> Result<Self, MessageError>
    where
        T: PgExecutor<'a>,
    {
        let message: Self = msg.into();
        message.pg_insert(executor).await?;
        Ok(message)
    }

    async fn pg_insert<T>(&self, executor: T) -> Result<(), MessageError>
    where
        T: PgExecutor<'a>,
    {
        sqlx::query(
            r#"
            INSERT INTO messages (id, type, sender, body, created_at, updated_at, subject, recipients, device_id, title, times)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        ).bind(&self.id)
            .bind(&self.r#type)
            .bind(&self.sender)
            .bind(&self.body)
            .bind(self.created_at)
            .bind(self.updated_at)
            .bind(&self.subject)
            .bind(&self.recipients)
            .bind(&self.device_id)
            .bind(&self.title)
            .bind(1)
            .execute(executor)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::common_test;

    #[tokio::test]
    async fn test_insert_email() {
        let (_tdb, pool, _) = common_test().await.unwrap();
        let message = Message::insert_email(
            EmailMessage {
                id: "test_id".to_string(),
                subject: "test_subject".to_string(),
                sender: "test_sender".to_string(),
                recipients: vec!["test_recipient".to_string()],
                body: "test_body".to_string(),
            },
            &pool,
        )
        .await
        .unwrap();
        assert_eq!(message.id, "test_id")
    }
}
