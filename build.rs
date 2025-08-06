use prost_build;

fn main() {
    // Generate Rust types for all protobuf definitions from peer-observer
    let proto_files = [
        "peer-observer/protobuf/proto-types/event_msg.proto",
        "peer-observer/protobuf/proto-types/mempool.proto", 
        "peer-observer/protobuf/proto-types/net_msg.proto",
        "peer-observer/protobuf/proto-types/net_conn.proto",
        "peer-observer/protobuf/proto-types/addrman.proto",
        "peer-observer/protobuf/proto-types/validation.proto",
        "peer-observer/protobuf/proto-types/rpc.proto",
        "peer-observer/protobuf/proto-types/primitive.proto",
    ];

    if let Err(e) = prost_build::Config::new()
        .compile_well_known_types()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &proto_files,
            &["peer-observer/protobuf/proto-types/"],
        )
    {
        println!("Error while compiling protos: {}", e);
        panic!("Failed to code-gen the Rust structs from the Protobuf definitions");
    }

    // Rebuild if protobuf files change
    println!("cargo:rerun-if-changed=peer-observer/protobuf/proto-types/");
    println!("cargo:rerun-if-changed=build.rs");
}