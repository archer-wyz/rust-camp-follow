use camp_core::core_fake::before;
use camp_metadata::abi::Tpl;
use camp_notification::pb::notification::{send_request, EmailMessage, SendRequest};
use derive_builder::Builder;
use futures::StreamExt as _;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{async_trait, Status};
use tracing::info;

use crate::{
    config::WelcomeConfig,
    pb::crm::{
        crm_server::Crm, RecallRequest, RecallResponse, RemindRequest, RemindResponse,
        WelcomeRequest, WelcomeResponse,
    },
    services::{MetaData, Notification, UserStat},
};

use tonic::{Request, Response};

#[derive(Clone, Debug, Builder)]
pub struct CrmGrpc<T: MetaData, D: UserStat, U: Notification> {
    pub metadata_service: Arc<T>,
    pub user_stat_service: Arc<D>,
    pub notification_service: Arc<U>,
    pub welcome_config: WelcomeConfig,
}

#[async_trait]
impl<T: MetaData, D: UserStat, U: Notification> Crm for CrmGrpc<T, D, U> {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let welcome = request.into_inner();
        let Ok(contents) = self.metadata_service.get_content(welcome.content_ids).await else {
            return Err(Status::internal("Failed to fetch content"));
        };
        let Ok(mut user_stat_stream) = self
            .user_stat_service
            .get_new_user_stream(
                before(self.welcome_config.created_before_lower),
                before(self.welcome_config.created_before_upper),
            )
            .await
        else {
            return Err(Status::internal("Failed to fetch new user stream"));
        };
        let contents = Arc::new(contents);
        let (tx, rx) = mpsc::channel(1024);
        let mut notification_resp_stream = match self.notification_service.notification(rx).await {
            Ok(resp) => resp,
            Err(_) => return Err(Status::internal("Failed to send notification")),
        };
        info!("welcome {:?}", contents);
        tokio::spawn(async move {
            while let Some(user_resp) = user_stat_stream.next().await {
                let contents_clone = contents.clone();
                let user = match user_resp {
                    Ok(user) => user,
                    Err(e) => {
                        info!("Error while fetching user: {:?}", e);
                        continue;
                    }
                };
                // TODO use Faker
                let email_msg = send_request::Msg::Email(EmailMessage {
                    message_id: uuid::Uuid::new_v4().to_string(),
                    subject: "welcome".to_string(),
                    sender: "welcome".to_string(),
                    recipients: vec![user.email],
                    body: Tpl(contents_clone.as_ref()).to_body(),
                });
                tx.send(SendRequest {
                    msg: Some(email_msg),
                })
                .await
                .unwrap();
            }
        });
        while let Some(notification_resp) = notification_resp_stream.next().await {
            match notification_resp {
                Ok(_) => {}
                Err(e) => {
                    info!("Error while sending notification: {:?}", e);
                    return Err(Status::internal("Failed to send notification"));
                }
            }
        }
        Ok(Response::new(WelcomeResponse {
            id: uuid::Uuid::new_v4().to_string(),
        }))
    }

    /// last watched in X days, given them something to watch
    async fn recall(
        &self,
        _request: Request<RecallRequest>,
    ) -> std::result::Result<Response<RecallResponse>, Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }
    /// last watched in X days, and user still have unfinished contents
    async fn remind(
        &self,
        _request: Request<RemindRequest>,
    ) -> std::result::Result<Response<RemindResponse>, Status> {
        Err(tonic::Status::unimplemented("Not implemented"))
    }
}

#[cfg(test)]
mod test {
    use crate::services::{
        metadata::MockMetaData, notification::MockNotification, user_stat::MockUserStat,
    };
    use camp_core::{core_fake::UniqueEmail, proto::utc_to_ts};
    use camp_notification::pb::notification::{SendResponse, SendResponseType};
    use camp_user_stat::pb::user_stat::User;
    use chrono::Utc;
    use fake::{faker::name::zh_cn::Name, Fake, Faker};

    use super::*;
    use anyhow::Result;
    use camp_metadata::pb::metadata::Content;
    use unimock::{matching, MockFn, Unimock};

    #[tokio::test]
    async fn test_welcome() -> Result<()> {
        let email = Arc::new(Unimock::new(
            MockMetaData::get_content
                .some_call(matching!())
                .answers(&|_, ids| {
                    let mut contents = Vec::with_capacity(ids.len());
                    let mut rng = rand::thread_rng();
                    for id in ids {
                        let mut content: Content = Faker.fake_with_rng(&mut rng);
                        content.id = id;
                        contents.push(content)
                    }
                    Ok(contents)
                }),
        ));

        let user_stat = Arc::new(Unimock::new(
            MockUserStat::get_new_user_stream
                .some_call(matching!())
                .answers(&|_, _, _| {
                    let mut rng = rand::thread_rng();
                    let users: Vec<Result<User, Status>> = vec![
                        Ok(User {
                            email: UniqueEmail.fake_with_rng(&mut rng),
                            name: Name().fake_with_rng(&mut rng),
                        }),
                        Ok(User {
                            email: UniqueEmail.fake_with_rng(&mut rng),
                            name: Name().fake_with_rng(&mut rng),
                        }),
                        Err(Status::unknown("mock error")),
                    ];
                    let var_name = users.clone();
                    Ok(Box::pin(tokio_stream::iter(var_name)))
                }),
        ));
        let notification = Arc::new(Unimock::new(
            MockNotification::notification
                .some_call(matching!())
                .answers(&|_, _| {
                    let send_responses: Vec<Result<SendResponse, Status>> = vec![
                        Ok(SendResponse {
                            message_id: "1".to_string(),
                            timestamp: Some(utc_to_ts(Utc::now())),
                            status: SendResponseType::Success as i32,
                        }),
                        Ok(SendResponse {
                            message_id: "2".to_string(),
                            timestamp: Some(utc_to_ts(Utc::now())),
                            status: SendResponseType::Success as i32,
                        }),
                    ];
                    Ok(Box::pin(tokio_stream::iter(send_responses)))
                }),
        ));
        let crm = CrmGrpcBuilder::default()
            .metadata_service(email)
            .user_stat_service(user_stat)
            .notification_service(notification)
            .welcome_config(WelcomeConfig {
                created_before_lower: 1,
                created_before_upper: 0,
            })
            .build()
            .unwrap();
        crm.welcome(Request::new(WelcomeRequest {
            content_ids: vec![1, 2, 3],
            interval: 1,
        }))
        .await?;
        Ok(())
    }
}
