use anyhow::Result;
use camp_user_stat::{abi::UserStatGRPC, services::UserStatServiceImpl, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    let app_state = AppState::<UserStatGRPC<UserStatServiceImpl>>::try_new().await?;
    app_state.grpc_run().await?;
    Ok(())
}
