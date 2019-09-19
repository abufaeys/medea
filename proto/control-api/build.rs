use std::{error::Error, fs::File, io::ErrorKind};

#[cfg(feature = "grpc")]
fn main() -> Result<(), Box<dyn Error>> {
    const GRPC_DIR: &str = "src/grpc/";
    const GRPC_SPEC_FILE: &str = "src/grpc/control_api.proto";
    const OUT_FILES: [&str; 2] =
        ["src/grpc/control_api.rs", "src/grpc/control_api_grpc.rs"];

    println!("cargo:rerun-if-changed={}", GRPC_DIR);

    for filename in &OUT_FILES {
        if let Err(e) = File::open(filename) {
            if let ErrorKind::NotFound = e.kind() {
                protoc_grpcio::compile_grpc_protos(
                    &[GRPC_SPEC_FILE],
                    &[GRPC_DIR],
                    &GRPC_DIR,
                    None,
                )
                .expect("Failed to compile gRPC definitions!");
                break;
            } else {
                panic!("{:?}", e);
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "grpc"))]
fn main() {}