use anyhow::Result;
use camp_core::config::config_load;
use std::{net::ToSocketAddrs as _, time::Duration};
use tokio::time::sleep;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

use camp_metadata::{
    abi::MetadataGRPC,
    config::AppConfig,
    pb::metadata::{
        metadata_client::MetadataClient, metadata_server::MetadataServer, MaterializeRequest,
    },
};

fn no_end_stream() -> impl Stream<Item = MaterializeRequest> {
    tokio_stream::iter(vec![
        MaterializeRequest { id: 1 },
        MaterializeRequest { id: 2 },
        MaterializeRequest { id: 3 },
    ])
}

pub async fn echo_metadata(config: &AppConfig) -> Result<()> {
    let grpc_url = format!("http://[::1]:{}", config.server.port);
    let mut client = MetadataClient::connect(grpc_url).await?;
    let request = tonic::Request::new(no_end_stream());
    let mut stream = client.materialize(request).await?.into_inner();

    while let Some(response) = stream.next().await {
        info!("RESPONSE={:?}", response?);
    }
    Ok(())
}

#[tokio::test]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let server = MetadataGRPC {};
    let config: AppConfig = config_load(vec![
        "./camp-metadata/metadata.yml".to_string(),
        "./metadata.yml".to_string(),
        "/etc/config/metadata.yml".to_string(),
    ])?;
    let addr = format!("[::1]:{:?}", config.server.port);
    info!("grpc server port: {:?}", config.server.port);
    tokio::spawn(async move {
        Server::builder()
            .add_service(MetadataServer::new(server))
            .serve(addr.to_socket_addrs().unwrap().next().unwrap())
            .await
    });
    sleep(Duration::from_micros(1)).await;
    echo_metadata(&config).await?;
    Ok(())
}
