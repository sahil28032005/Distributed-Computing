//! Configuration types and utilities.

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::{DatabaseError, Result};
use crate::types::NodeId;

/// Configuration for a database node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_id: NodeId,
    pub listen_addr: String,
    pub data_dir: String,
    pub raft_peers: Vec<String>,
    pub coordinator_addr: Option<String>,
    pub heartbeat_interval_ms: u64,
    pub election_timeout_min_ms: u64,
    pub election_timeout_max_ms: u64,
    pub snapshot_threshold: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: NodeId("node1".to_string()),
            listen_addr: "127.0.0.1:9090".to_string(),
            data_dir: "data".to_string(),
            raft_peers: vec![],
            coordinator_addr: None,
            heartbeat_interval_ms: 100,
            election_timeout_min_ms: 150,
            election_timeout_max_ms: 300,
            snapshot_threshold: 1000,
        }
    }
}

/// Configuration for a coordinator node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorConfig {
    pub listen_addr: String,
    pub initial_nodes: Vec<String>,
    pub metadata_refresh_interval_ms: u64,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:8080".to_string(),
            initial_nodes: vec![],
            metadata_refresh_interval_ms: 1000,
        }
    }
}

/// Load configuration from a file.
pub fn load_config<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T> {
    let config_file = std::fs::read_to_string(path)
        .map_err(|e| DatabaseError::Config(format!("Failed to read config file: {}", e)))?;
    
    let config: T = serde_json::from_str(&config_file)
        .map_err(|e| DatabaseError::Config(format!("Failed to parse config file: {}", e)))?;
    
    Ok(config)
}

/// Save configuration to a file.
pub fn save_config<T: Serialize>(config: &T, path: impl AsRef<Path>) -> Result<()> {
    let config_str = serde_json::to_string_pretty(config)
        .map_err(|e| DatabaseError::Config(format!("Failed to serialize config: {}", e)))?;
    
    std::fs::write(path, config_str)
        .map_err(|e| DatabaseError::Config(format!("Failed to write config file: {}", e)))?;
    
    Ok(())
}