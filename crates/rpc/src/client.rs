//! Client implementations for connecting to services.

use tonic::transport::{Channel, Endpoint};
use crate::proto::database::database_service_client::DatabaseServiceClient;
use crate::proto::node::node_service_client::NodeServiceClient;
use crate::proto::raft::raft_service_client::RaftServiceClient;
use crate::proto::database::{GetRequest, PutRequest, DeleteRequest, ScanRequest, QueryRequest};
use common::error::{DatabaseError, Result};
use std::time::Duration;

/// Client for the database service.
pub struct DatabaseClient {
    client: DatabaseServiceClient<Channel>,
}

impl DatabaseClient {
    /// Create a new database client.
    pub async fn connect(addr: &str) -> Result<Self> {
        let endpoint = Endpoint::from_shared(format!("http://{}", addr))
            .map_err(|e| DatabaseError::Rpc(format!("Invalid endpoint: {}", e)))?
            .timeout(Duration::from_secs(5));
        
        let client = DatabaseServiceClient::connect(endpoint)
            .await
            .map_err(|e| DatabaseError::Rpc(format!("Failed to connect: {}", e)))?;
        
        Ok(Self { client })
    }

    /// Execute a SQL query
    pub async fn execute_query(&mut self, query: String, parameters: std::collections::HashMap<String, String>) -> Result<crate::proto::database::QueryResponse> {
        let request = QueryRequest {
            query,
            parameters,
        };
        
        self.client.execute_query(request)
            .await
            .map(|r| r.into_inner())
            .map_err(|e| DatabaseError::Rpc(format!("Query execution failed: {}", e)))
    }

    /// Get a value by key
    pub async fn get(&mut self, key: String) -> Result<crate::proto::database::GetResponse> {
        let request = GetRequest { key };
        
        self.client.get(request)
            .await
            .map(|r| r.into_inner())
            .map_err(|e| DatabaseError::Rpc(format!("Get operation failed: {}", e)))
    }

    /// Put a key-value pair
    pub async fn put(&mut self, key: String, value: Vec<u8>) -> Result<crate::proto::database::PutResponse> {
        let request = PutRequest { key, value };
        
        self.client.put(request)
            .await
            .map(|r| r.into_inner())
            .map_err(|e| DatabaseError::Rpc(format!("Put operation failed: {}", e)))
    }

    /// Delete a key
    pub async fn delete(&mut self, key: String) -> Result<crate::proto::database::DeleteResponse> {
        let request = DeleteRequest { key };
        
        self.client.delete(request)
            .await
            .map(|r| r.into_inner())
            .map_err(|e| DatabaseError::Rpc(format!("Delete operation failed: {}", e)))
    }

    /// Scan a range of keys
    pub async fn scan(&mut self, start_key: String, end_key: String, limit: i32) -> Result<crate::proto::database::ScanResponse> {
        let request = ScanRequest {
            start_key,
            end_key,
            limit,
        };
        
        self.client.scan(request)
            .await
            .map(|r| r.into_inner())
            .map_err(|e| DatabaseError::Rpc(format!("Scan operation failed: {}", e)))
    }
}

/// Client for the node service.
pub struct NodeClient {
    client: NodeServiceClient<Channel>,
}

impl NodeClient {
    /// Create a new node client.
    pub async fn connect(addr: &str) -> Result<Self> {
        let endpoint = Endpoint::from_shared(format!("http://{}", addr))
            .map_err(|e| DatabaseError::Rpc(format!("Invalid endpoint: {}", e)))?
            .timeout(Duration::from_secs(5));
        
        let client = NodeServiceClient::connect(endpoint)
            .await
            .map_err(|e| DatabaseError::Rpc(format!("Failed to connect: {}", e)))?;
        
        Ok(Self { client })
    }

    // Add methods for interacting with the node service
}

/// Client for the Raft service.
pub struct RaftClient {
    client: RaftServiceClient<Channel>,
}

impl RaftClient {
    /// Create a new Raft client.
    pub async fn connect(addr: &str) -> Result<Self> {
        let endpoint = Endpoint::from_shared(format!("http://{}", addr))
            .map_err(|e| DatabaseError::Rpc(format!("Invalid endpoint: {}", e)))?
            .timeout(Duration::from_secs(5));
        
        let client = RaftServiceClient::connect(endpoint)
            .await
            .map_err(|e| DatabaseError::Rpc(format!("Failed to connect: {}", e)))?;
        
        Ok(Self { client })
    }

    // Add methods for interacting with the Raft service
}