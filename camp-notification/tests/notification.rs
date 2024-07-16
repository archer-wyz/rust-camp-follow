use anyhow::Result;
use camp_core::core_fake::{UniqueEmail, VecRanger};
use camp_notification::pb::notification::{notification_client::NotificationClient, SendRequest};
use camp_notification::pb::notification::{send_request, EmailMessage, InAppMessage, SmsMessage};
use camp_notification::AppState;
use fake::faker::name::en::Name;
use fake::Fake as _;
use futures::StreamExt;
use tokio::time::{sleep, Duration};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::test]
async fn send_email_should_work() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let (_tbd, app_state) = AppState::new_for_test().await?;
    let config = app_state.app_config.clone();
    info!("Starting notification service ...");

    tokio::spawn(async move { app_state.grpc_run().await });
    sleep(Duration::from_millis(10)).await;

    let grpc_url = format!("http://[::1]:{}", config.grpc.port);
    let mut client = NotificationClient::connect(grpc_url).await?;
    let email = Some(send_request::Msg::Email(EmailMessage {
        message_id: "1".to_string(),
        subject: "test".to_string(),
        sender: "test".to_string(),
        recipients: vec!["test@163.com".to_string()],
        body: "test body 1".to_string(),
    }));

    let sms = Some(send_request::Msg::Sms(SmsMessage {
        message_id: "2".to_string(),
        sender: UniqueEmail.fake(),
        recipients: VecRanger {
            lower: 1,
            upper: 10,
            item: UniqueEmail,
        }
        .fake(),
        body: "test body 2".to_string(),
    }));
    let inapp = Some(send_request::Msg::InApp(InAppMessage {
        message_id: "3".to_string(),
        device_id: Name().fake(),
        title: Name().fake(),
        body: "test body 3".to_string(),
        sender: UniqueEmail.fake(),
    }));

    let send_requests = vec![
        SendRequest { msg: email },
        SendRequest { msg: sms },
        SendRequest { msg: inapp },
    ];
    let p = tokio_stream::iter(send_requests);
    let stream_resp = client.send(p).await?.into_inner();
    let ret: Vec<_> = stream_resp.then(|res| async { res }).collect().await;
    info!("send_requests received : {:?}", ret);
    Ok(())
}
