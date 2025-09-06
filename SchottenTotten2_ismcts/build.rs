use std::io::Result;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    // Specify the path to your external protobuf files
    let proto_dir = Path::new("../SchottenTotten2_proto/protos");

    // A list of all your .proto file names
    let proto_files_list = vec!["card.proto", "client_move.proto", "game_state.proto", "wall.proto"];

    // Use a loop to create a vector of full paths
    let mut protos_to_compile: Vec<PathBuf> = Vec::new();
    for file_name in proto_files_list {
        protos_to_compile.push(proto_dir.join(file_name));
    }

    // Tell Cargo to rerun the build script if the .proto files change
    println!("cargo:rerun-if-changed={}", proto_dir.to_str().unwrap());

    // Call compile_protos with the vector of paths
    prost_build::compile_protos(
        &protos_to_compile, // Pass the vector here
        &[proto_dir],       // Specify the include directory
    )?;

    Ok(())
}