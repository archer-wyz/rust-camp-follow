use futures::StreamExt as _;
use prost_types::Timestamp;
use std::pin::Pin;
use tokio::sync::mpsc::channel;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{async_trait, Request, Response, Status, Streaming};
use tracing::info;
pub mod email;
pub mod inapp;
pub mod sms;

use crate::{
    pb::notification::{
        notification_server::Notification, send_request::Msg, SendRequest, SendResponse,
        SendResponseType,
    },
    services,
};

type NotificationStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[derive(Clone, derive_builder::Builder)]
pub struct NotificationGrpc {
    pub email: email::EmailGrpc,
    pub inapp: inapp::InAppGrpc,
    pub sms: sms::SmsGrpc,
}

impl NotificationGrpc {
    async fn notification(&self, req: SendRequest) -> Result<SendResponse, Status> {
        let Some(msg) = req.msg else {
            return Err(Status::not_found("msg is None"));
        };
        match msg {
            Msg::Email(msg) => self.email.send_email(msg).await,
            Msg::Sms(msg) => self.sms.send_sms(msg).await,
            Msg::InApp(msg) => self.inapp.send_inapp(msg).await,
        }
    }
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
        let (tx, rx) = channel(1024);
        let streamer = ReceiverStream::new(rx);
        let notification = self.clone();
        tokio::spawn(async move {
            let mut stream: Streaming<SendRequest> = request.into_inner();
            info!("streaming request");
            while let Some(request) = stream.next().await {
                let Ok(req) = request else {
                    tx.send(Err(Status::internal("request status error")))
                        .await
                        .unwrap();
                    break;
                };
                tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                tx.send(notification.notification(req).await).await.unwrap();
            }
        });
        Ok(Response::new(Box::pin(streamer)))
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
