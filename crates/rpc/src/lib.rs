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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use tonic::transport::Server;
    use std::net::SocketAddr;
    use std::str::FromStr;

    #[test]
    fn test_database_service() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // Start a server
            let addr = "127.0.0.1:50051";
            let socket_addr = SocketAddr::from_str(addr).unwrap();
            let service = database_service::DatabaseServiceImpl::new();
            
            let server_future = Server::builder()
                .add_service(service.into_server())
                .serve(socket_addr);
            
            // Spawn the server on a separate task
            tokio::spawn(server_future);
            
            // Give the server some time to start
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            // Connect a client
            let client = client::DatabaseClient::connect(addr).await;
            assert!(client.is_ok());
        });
    }
}
        