use anyhow::Result;
use camp_user_stat::fake::UserStatFakerList;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost/crm").await?;
    info!("Camp-User-Stat Faking");
    let faker_list = UserStatFakerList(2000);
    faker_list.fake_and_insert(pool).await?;
    info!("Camp-User-Stat Faking End");
    Ok(())
}
