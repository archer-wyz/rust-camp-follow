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
        .with_derive_builder(
            &[
                "EmailMessage",
                "SmsMessage",
                "InAppMessage",
                "SendRequest",
                "SendResponse",
            ],
            None,
        )
        .compile(
            &[
                "../protos/notification/messages.proto",
                "../protos/notification/rpc.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}
