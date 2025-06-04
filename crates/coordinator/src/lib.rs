use common::error::Result;
use std::vec::Vec;

mod server;

/// Coordinator manages the distributed system components
pub struct Coordinator {
    // Add fields as needed for coordination logic
}

impl Coordinator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute_query(&mut self, _query: String, _parameters: std::collections::HashMap<String, String>) -> Result<()> {
        // TODO: Implement query execution logic
        // This will eventually need to coordinate with database nodes
        Ok(())
    }

    pub async fn get(&mut self, _key: String) -> Result<Vec<u8>> {
        // TODO: Implement get logic
        // This will eventually need to coordinate with storage nodes
        Ok(Vec::new())
    }
}

// Re-export the server module's start_grpc_server function
pub use server::start_grpc_server;