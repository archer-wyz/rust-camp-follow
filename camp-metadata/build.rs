use std::env::current_dir;

use anyhow::Result;

fn main() -> Result<()> {
    let dir = current_dir()?;
    println!("Current directory: {:?}", dir);
    std::fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder.out_dir("src/pb").compile(
        &[
            "../protos/metadata/rpc.proto",
            "../protos/metadata/messages.proto",
        ],
        &["../protos"],
    )?;
    Ok(())
}
