//! Error types for the distributed database system.

use thiserror::Error;

/// The main error type for the database system.
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
    
    #[error("RPC error: {0}")]
    Rpc(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Raft error: {0}")]
    Raft(String),
    
    #[error("SQL parsing error: {0}")]
    SqlParsing(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    
    #[error("Partition error: {0}")]
    Partition(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// A specialized Result type for database operations.
pub type Result<T> = std::result::Result<T, DatabaseError>;

impl From<&str> for DatabaseError {
    fn from(s: &str) -> Self {
        DatabaseError::Unknown(s.to_string())
    }
}

impl From<String> for DatabaseError {
    fn from(s: String) -> Self {
        DatabaseError::Unknown(s)
    }
}