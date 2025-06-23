//! Simcache: A flexible caching library with pluggable eviction policies

pub mod cache;
pub mod eviction;

// Re-export main types for convenience
pub use cache::Simcache;
pub use eviction::{EvictionPolicy, LRU};

// Re-export commonly used types
pub use std::time::Duration;