use anyhow::Result;
use std::time::Duration;
use tokio_stream::{Stream, StreamExt};

use camp_crm::pb::metadata::{metadata_client::MetadataClient, MaterializeRequest};

fn no_end_stream() -> impl Stream<Item = MaterializeRequest> {
    let stream = std::iter::repeat(MaterializeRequest { id: 1 });
    tokio_stream::iter(stream).throttle(Duration::from_millis(200))
}

pub async fn echo_metadata() -> Result<()> {
    let mut client = MetadataClient::connect("http://[::1]:50002").await?;
    let request = tonic::Request::new(no_end_stream());
    let mut stream = client.materialize(request).await?.into_inner();

    while let Some(response) = stream.next().await {
        println!("RESPONSE={:?}", response?);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    echo_metadata().await?;
    Ok(())
}
