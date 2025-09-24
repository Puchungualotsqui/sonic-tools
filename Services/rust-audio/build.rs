use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = crate_dir.join("proto");

    // Re-run if any proto changes
    println!("cargo:rerun-if-changed={}", proto_dir.display());

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&[proto_dir.join("audio.proto")], &[proto_dir])?;

    Ok(())
}
