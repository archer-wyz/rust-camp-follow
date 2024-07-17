use camp_core::proto::utc_to_ts;
use chrono::Utc;
use std::sync::Arc;
use tonic::Status;
use tracing::{info, warn};

use crate::{
    pb::notification::{InAppMessage, SendResponse, SendResponseType},
    services::{
        inapp::{self, InApp, InAppError},
        ServiceError,
    },
};

#[derive(Clone)]
pub struct InAppGrpc(pub Arc<Box<dyn InApp>>);

impl From<InAppMessage> for inapp::InAppMessage {
    fn from(value: InAppMessage) -> Self {
        Self {
            title: value.title,
            body: value.body,
            id: value.message_id,
            device_id: value.device_id,
            sender: value.sender,
        }
    }
}

impl InAppGrpc {
    pub async fn send_inapp(&self, req: InAppMessage) -> Result<SendResponse, Status> {
        info!("sending inapp {:?}", req.message_id);
        match self.0.send_inapp(req.into()).await {
            Ok(msg) => {
                info!("InApp sending failed: {:?}", msg.id);
                Ok(SendResponse {
                    message_id: msg.id,
                    timestamp: Some(utc_to_ts(msg.timestamp)),
                    status: SendResponseType::Success as i32,
                })
            }
            Err(e) => match e {
                ServiceError::InApp(InAppError::Send(e)) => {
                    warn!("InApp sending failed: {:?}", e.id);
                    Ok(SendResponse {
                        message_id: e.id,
                        timestamp: Some(utc_to_ts(Utc::now())),
                        status: SendResponseType::Failed as i32,
                    })
                }
                _ => Err(Status::internal("Internal error")),
            },
        }
    }
}
