use anyhow::Result;
use camp_crm::pb::crm::{crm_client::CrmClient, GetUserRequest};
use tonic::{metadata::MetadataValue, transport::Channel, Request, Status};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let channel = Channel::from_static("http://[::1]:50052").connect().await?;
    let mut client = CrmClient::with_interceptor(channel, test_interceptor);
    let request = Request::new(GetUserRequest { id: 1 });
    let user = client.get_user(request).await?;
    println!("User: {:?}", user);
    Ok(())
}

fn test_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    let metadata = req.metadata_mut();
    match metadata.get("authorization") {
        Some(value) => {
            info!("Authorization: {:?}", value);
            Ok(req)
        }
        _ => {
            metadata.insert("authorization", MetadataValue::from_static("abc"));
            Ok(req)
        }
    }
}
