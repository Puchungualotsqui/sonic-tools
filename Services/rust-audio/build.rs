use std::env;
use std::path::PathBuf;

fn main() {
    let proto_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap() // go up from rust-audio/
        .join("../proto"); // points to SonicTools/proto

    tonic_build::configure()
        .compile(&[proto_root.join("audio.proto")], &[proto_root])
        .unwrap();
}
