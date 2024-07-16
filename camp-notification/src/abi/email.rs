use camp_core::proto::utc_to_ts;
use chrono::Utc;
use std::sync::Arc;
use tonic::Status;
use tracing::info;

use crate::{
    pb::notification::{EmailMessage, SendResponse, SendResponseType},
    services::{
        email::{self, Email, EmailError},
        ServiceError,
    },
};

#[derive(Clone)]
pub struct EmailGrpc(pub Arc<Box<dyn Email>>);

impl From<EmailMessage> for email::EmailMessage {
    fn from(value: EmailMessage) -> Self {
        Self {
            subject: value.subject,
            body: value.body,
            id: value.message_id,
            sender: value.sender,
            recipients: value.recipients,
        }
    }
}

impl EmailGrpc {
    pub async fn send_email(&self, req: EmailMessage) -> Result<SendResponse, Status> {
        info!("sending email {:?}", req.message_id);
        match self.0.send_email(req.into()).await {
            Ok(msg) => Ok(SendResponse {
                message_id: msg.id,
                timestamp: Some(utc_to_ts(msg.timestamp)),
                status: SendResponseType::Success as i32,
            }),
            Err(e) => match e {
                ServiceError::Email(e) => match e {
                    EmailError::FailedButSaved(msg) => Ok(SendResponse {
                        message_id: msg.id,
                        timestamp: Some(utc_to_ts(Utc::now())),
                        status: SendResponseType::Stored as i32,
                    }),
                    _ => Err(Status::internal(e.to_string())),
                },
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
