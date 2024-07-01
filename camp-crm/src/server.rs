use anyhow::Result;
use camp_crm::pb::crm::{
    crm_server::{Crm, CrmServer},
    CreateUserRequest, GetUserRequest, User,
};
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

struct UserService;

#[tonic::async_trait]
impl Crm for UserService {
    async fn get_user(&self, _request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "".to_string(),
            create_at: None,
        };
        Ok(Response::new(user))
    }

    async fn create_user(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "".to_string(),
            create_at: None,
        };
        Ok(Response::new(user))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    info!("Starting crm server...");

    let addr = format!("[::1]:{}", 50052).parse().unwrap();
    info!("CRM service listening on {}", addr);
    let svc = CrmServer::new(UserService);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
