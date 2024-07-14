use camp_core::proto::utc_to_ts;
use chrono::Utc;
use prost_types::Timestamp;
use std::{pin::Pin, sync::Arc};
use tokio::sync::mpsc::channel;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{async_trait, Request, Response, Status, Streaming};

use crate::{
    pb::notification::{
        notification_server::Notification, send_request::Msg, EmailMessage, InAppMessage,
        SendRequest, SendResponse, SendResponseType, SmsMessage,
    },
    services::{self, email, inapp, sms, ServiceError},
};

type NotificationStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[derive(Clone, derive_builder::Builder)]
pub struct NotificationGrpc {
    pub email: Arc<Box<dyn email::Email>>,
    pub inapp: Arc<Box<dyn inapp::InApp>>,
    pub sms: Arc<Box<dyn sms::Sms>>,
}

#[async_trait]
impl Notification for NotificationGrpc {
    /// Server streaming response type for the Send method.
    /// Send a notification to a user.
    type SendStream = NotificationStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> Result<Response<Self::SendStream>, Status> {
        let (tx, rx) = channel(5);
        let self_ = self.clone();
        tokio::spawn(async move {
            let mut stream: Streaming<SendRequest> = request.into_inner();
            loop {
                let request = match stream.message().await {
                    Ok(Some(request)) => request,
                    Ok(None) => {
                        tx.send(Err(Status::ok("notification stream end")))
                            .await
                            .unwrap();
                        break;
                    }
                    Err(_) => {
                        tx.send(Err(Status::unknown("notification send stream error")))
                            .await
                            .unwrap();
                        break;
                    }
                };
                if let Some(msg) = request.msg {
                    let (id, response) = match msg {
                        Msg::InApp(msg) => (
                            msg.message_id.clone(),
                            self_.inapp.send_inapp(msg.into()).await,
                        ),
                        Msg::Email(msg) => (
                            msg.message_id.clone(),
                            self_.email.send_email(msg.into()).await,
                        ),
                        Msg::Sms(msg) => {
                            (msg.message_id.clone(), self_.sms.send_sms(msg.into()).await)
                        }
                    };
                    match response {
                        Ok(response) => tx.send(Ok(response.into())).await.unwrap(),
                        Err(err) => tx.send(to_resp_result(id, err)).await.unwrap(),
                    }
                }
            }
        });
        let streamer = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(streamer)))
    }
}

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

impl From<services::SendResponse> for SendResponse {
    fn from(value: services::SendResponse) -> Self {
        Self {
            message_id: value.id,
            timestamp: Some(Timestamp {
                seconds: value.timestamp.timestamp(),
                nanos: value.timestamp.timestamp_subsec_nanos() as i32,
            }),
            status: SendResponseType::Success as i32,
        }
    }
}

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

fn to_resp_result(id: String, err: ServiceError) -> Result<SendResponse, Status> {
    match err {
        ServiceError::Email(err) => match err {
            email::EmailError::FailedButSaved(_) => Ok(SendResponse {
                message_id: id,
                timestamp: Some(utc_to_ts(Utc::now())),
                status: SendResponseType::Stored as i32,
            }),
            _ => Err(Status::unknown(err.to_string())),
        },
        ServiceError::InApp(err) => Err(Status::unknown(err.to_string())),
        ServiceError::Sms(err) => Err(Status::unknown(err.to_string())),
    }
}
