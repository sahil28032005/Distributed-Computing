//! Common types used across the database system.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// A unique identifier for a node in the cluster.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NodeId {
    fn from(s: String) -> Self {
        NodeId(s)
    }
}

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        NodeId(s.to_string())
    }
}

/// Represents a range of keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRange {
    pub start: String,
    pub end: String,
}

impl KeyRange {
    pub fn new(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Check if a key is within this range.
    pub fn contains(&self, key: &str) -> bool {
        key >= self.start.as_str() && key < self.end.as_str() //needs to be properly derefernced otherwise crates errors
    }
}

/// Information about a partition in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub id: u64,
    pub range: KeyRange,
    pub leader: NodeId,
    pub followers: Vec<NodeId>,
}

/// A log entry in the Raft consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub command: Command,
    pub timestamp: DateTime<Utc>,
}

/// Commands that can be executed on the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Write { key: String, value: Vec<u8> },
    Delete { key: String },
    CreatePartition { partition: PartitionInfo },
    UpdatePartition { partition: PartitionInfo },
    DeletePartition { partition_id: u64 },
}

/// Metadata about the cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMetadata {
    pub nodes: HashMap<NodeId, NodeInfo>,
    pub partitions: HashMap<u64, PartitionInfo>,
    pub version: u64,
}

/// Information about a node in the cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: NodeId,
    pub address: String,
    pub status: NodeStatus,
    pub last_heartbeat: Option<DateTime<Utc>>,
}

/// The status of a node in the cluster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Active,
    Inactive,
    Joining,
    Leaving,
}