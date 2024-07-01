use anyhow::Result;
use camp_crm::pb::crm::{crm_client::CrmClient, GetUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "http://[::1]:50052";
    let mut client = CrmClient::connect(addr).await?;
    let request = Request::new(GetUserRequest { id: 1 });
    let user = client.get_user(request).await?;
    println!("User: {:?}", user);
    Ok(())
}
