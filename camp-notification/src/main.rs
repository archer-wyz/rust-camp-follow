use anyhow::Result;
use camp_notification::AppState;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let app_state = AppState::new().await?;
    info!("Starting notification service ...");
    app_state.grpc_run().await?;
    Ok(())
}
