use std::env;

// build.rs
fn main() {
    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Create the full path for the .proto file
    let proto_folder = current_dir.join("proto");
    let proto_file = proto_folder.join("chess.proto");
    let out_dir = current_dir.join("data-loader/src/proto");

    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&[proto_file], &[proto_folder])
        .unwrap();
}
