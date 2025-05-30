//! RPC module for the distributed database system.

pub mod proto {
    // Include the generated code from the build script
    pub mod database {
        //this line pulls generated code from protobuff file
        tonic::include_proto!("database");
    }

    pub mod node {
        tonic::include_proto!("node");
    }

    pub mod raft {
        tonic::include_proto!("raft");
    }
}

pub mod database_service;
pub mod node_service;
pub mod raft_service;
pub mod client;
        