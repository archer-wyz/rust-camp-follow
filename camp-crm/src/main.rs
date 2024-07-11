use anyhow::Result;
use camp_crm::{ioc::CrmGrpcV1, AppState};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let app_state = AppState::<CrmGrpcV1>::try_new().await?;
    info!(
        "grpc server running on [::1]:{:?}",
        &app_state.app_config.grpc.port
    );
    app_state.grpc_run().await?;
    Ok(())
}
