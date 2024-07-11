use proto_builder_trait::tonic::BuilderAttributes;
use std::env::current_dir;

use anyhow::Result;

fn main() -> Result<()> {
    let dir = current_dir()?;
    println!("Current directory: {:?}", dir);
    std::fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
        .compile(
            &["../protos/crm/messages.proto", "../protos/crm/rpc.proto"],
            &["../protos"],
        )?;
    Ok(())
}
