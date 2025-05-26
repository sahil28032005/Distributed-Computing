fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto files
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/database.proto",
                "proto/node.proto",
                "proto/raft.proto",
            ],
            &["proto"],
        )?;
    
    Ok(())
}