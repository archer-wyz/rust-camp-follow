use std::sync::Arc;

use camp_core::proto::utc_to_ts;
use tonic::Status;
use tracing::info;

use crate::{
    pb::notification::{SendResponse, SendResponseType, SmsMessage},
    services::sms::{self, Sms},
};

#[derive(Clone)]
pub struct SmsGrpc(pub Arc<Box<dyn Sms>>);

impl From<SmsMessage> for sms::SmsMessage {
    fn from(value: SmsMessage) -> Self {
        Self {
            id: value.message_id,
            sender: value.sender,
            recipients: value.recipients,
            body: value.body,
        }
    }
}

impl SmsGrpc {
    pub async fn send_sms(&self, req: SmsMessage) -> Result<SendResponse, Status> {
        info!("sending sms {:?}", req.message_id);
        match self.0.send_sms(req.into()).await {
            Ok(msg) => Ok(SendResponse {
                message_id: msg.id,
                timestamp: Some(utc_to_ts(msg.timestamp)),
                status: SendResponseType::Success as i32,
            }),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
