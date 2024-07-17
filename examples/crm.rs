
#[tokio::main]
fn main() {
}

async fn start_metadata() -> Result<()>{
    let server = MetadataGRPC {};
    let config: AppConfig = config_load(vec![
        "./metadata.yml".to_string(),
        "/etc/config/metadata.yml".to_string(),
    ])?;
    let addr = format!("[::1]:{:?}", config.server.port);
    Server::builder()
        .add_service(MetadataServer::new(server))
        .serve(addr.to_socket_addrs()?.next().unwrap())
        .await?;
    Ok(())
}
