//! Eviction policies for the cache

mod policy;
mod lru;

pub use policy::EvictionPolicy;
pub use lru::LRU;