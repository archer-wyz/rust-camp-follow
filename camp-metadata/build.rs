use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::env::current_dir;

fn main() -> Result<()> {
    let dir = current_dir()?;
    println!("Current directory: {:?}", dir);
    std::fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .with_type_attributes(&["MaterializeRequest"], &[r#"#[derive(Eq, Hash)]"#])
        .compile(
            &[
                "../protos/metadata/rpc.proto",
                "../protos/metadata/messages.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}
