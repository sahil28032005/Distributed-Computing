//! Utility functions for the database system.
#[allow(unused_imports)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use crate::error::Result;
use crate::types::NodeId;
use log::info;

/// Generate a timestamp in milliseconds.
pub fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as u64
}

/// Create a directory if it doesn't exist.
pub fn ensure_dir_exists(path: impl Into<PathBuf>) -> Result<PathBuf> {
    let path = path.into();
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
        info!("Created directory: {:?}", path);
    }
    Ok(path)
}

/// Initialize the logger.
pub fn init_logger(node_id: &NodeId) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .format_module_path(true)
        .format_target(false)
        .init();
    
    info!("Logger initialized for node: {}", node_id);
}

/// Generate a random delay within a range.
pub fn random_delay(min_ms: u64, max_ms: u64) -> Duration {
    use std::time::Duration;
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(min_ms..=max_ms);
    Duration::from_millis(delay_ms)
}