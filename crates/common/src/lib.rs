//! Common types and utilities for the distributed database system.

pub mod error;
pub mod types;
pub mod config;
pub mod util;

// Re-export commonly used items
pub use error::{DatabaseError, Result};
pub use types::{KeyRange, PartitionInfo, NodeId, LogEntry};
pub use config::{NodeConfig, CoordinatorConfig, load_config};
