//! Eviction policies for the cache

mod policy;
mod lru;
mod lfu;

pub use policy::EvictionPolicy;
pub use lru::LRU;
pub use lfu::LFU;