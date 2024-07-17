use anyhow::Result;
use camp_user_stat::{
    ioc::UserStatGRPCV1,
    pb::user_stat::{user_stat_client::UserStatClient, IdQuery, QueryRequestBuilder},
    AppState,
};
use futures::StreamExt;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn query_should_work() -> Result<()> {
    let (_tdb, app_state) = AppState::<UserStatGRPCV1>::new_for_test().await?;
    //let app_state = AppState::<UserStatGRPCV1>::new().await?;
    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        app_state_clone.grpc_run().await.unwrap();
    });
    sleep(Duration::from_secs(1)).await;
    println!("grpc server started");
    let mut client = UserStatClient::connect("http://[::1]:50055").await?;
    let req = QueryRequestBuilder::default()
        .id((
            "viewed_but_not_started".to_string(),
            IdQuery { ids: vec![252790] },
        ))
        .build()?;
    let stream = client.query(req).await?.into_inner();
    let ret = stream.collect::<Vec<_>>().await;
    assert_eq!(ret.len(), 16);
    Ok(())
}
