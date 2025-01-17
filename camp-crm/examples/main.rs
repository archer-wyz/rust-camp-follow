use anyhow::Result;
use camp_crm::pb::crm::{crm_client::CrmClient, RecallRequest, RemindRequest, WelcomeRequest};
use tokio::time::{sleep, Duration};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    tokio::spawn(async {
        info!("Starting metadata service ...");
        camp_metadata::start_metadata_grpc().await.unwrap()
    });

    tokio::spawn(async {
        let user_stat = camp_user_stat::AppState::try_new().await.unwrap();
        info!("Starting user_stat service ...");
        user_stat.grpc_run().await.unwrap()
    });
    tokio::spawn(async {
        info!("Starting notification service ...");
        let notification = camp_notification::AppState::new().await.unwrap();
        notification.grpc_run().await.unwrap()
    });

    sleep(Duration::from_secs(1)).await;

    let crm = camp_crm::AppState::try_new().await.unwrap();
    let config = crm.app_config.clone();
    tokio::spawn(async move {
        info!("Starting crm service ...");
        crm.grpc_run().await
    });
    sleep(Duration::from_millis(10)).await;

    let mut client = CrmClient::connect(format!("http://[::1]:{}", config.grpc.port)).await?;
    let welcome_resp = client
        .welcome(WelcomeRequest {
            interval: 1,
            content_ids: vec![10, 20],
        })
        .await
        .unwrap();
    info!("welcome resp ={:?}", welcome_resp);
    sleep(Duration::from_secs(5)).await;
    let recall_resp = client
        .recall(RecallRequest {
            id: "test".to_string(),
            last_visit_interval: 30,
            content_ids: vec![100, 200, 300],
        })
        .await
        .unwrap();
    info!("recall resp ={:?}", recall_resp);
    sleep(Duration::from_secs(5)).await;
    let remind_resp = client
        .remind(RemindRequest {
            id: "test".to_string(),
            last_visit_interval: 30,
        })
        .await
        .unwrap();
    info!("remind resp ={:?}", remind_resp);
    Ok(())
}
