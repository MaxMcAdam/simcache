use crate::EvictionPolicy;
use std::collections::VecDeque;

pub struct LRU<K> {access_order: VecDeque<K>}

impl<K: PartialEq + Clone> EvictionPolicy<K> for LRU<K> {
    fn evict_next(&mut self) -> K {
        return self.access_order.pop_front().expect("there should be at least one element in the eviction queue")
    }
    fn key_used(&mut self, key: &K) {
        self.remove_key(key);
        self.access_order.push_back(key.clone());
    }
    fn remove_key(&mut self, key: &K) {
        if let Some(pos) = self.access_order.iter().position(|k| k == key) {
            self.access_order.remove(pos);
        }
    }
    fn new() -> Self {
        return LRU{access_order: VecDeque::new()}
    }
}