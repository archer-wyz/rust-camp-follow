use std::net::ToSocketAddrs;

use camp_metadata::{abi::MetadataGRPC, pb::metadata::metadata_server::MetadataServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = MetadataGRPC {};
    Server::builder()
        .add_service(MetadataServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}
