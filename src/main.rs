// Include the generated protobuf types
pub mod event_msg {
    include!(concat!(env!("OUT_DIR"), "/event_msg.rs"));
}
pub mod mempool {
    include!(concat!(env!("OUT_DIR"), "/mempool.rs"));
}
pub mod net_msg {
    include!(concat!(env!("OUT_DIR"), "/net_msg.rs"));
}
pub mod net_conn {
    include!(concat!(env!("OUT_DIR"), "/net_conn.rs"));
}
pub mod addrman {
    include!(concat!(env!("OUT_DIR"), "/addrman.rs"));
}
pub mod validation {
    include!(concat!(env!("OUT_DIR"), "/validation.rs"));
}
pub mod rpc {
    include!(concat!(env!("OUT_DIR"), "/rpc.rs"));
}
pub mod primitive {
    include!(concat!(env!("OUT_DIR"), "/primitive.rs"));
}

fn main() {
    println!("Rust Memo - Bitcoin Mempool Monitor");
    
    // Test that we can access the protobuf types
    println!("Protobuf types available:");
    println!("- EventMsg from peer-observer");
    println!("- MempoolEvent types");
    println!("- Network message types");
}
