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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lru_test() {
        let mut policy = LRU::new();

        policy.key_used(&"a");
        policy.key_used(&"b");
        policy.key_used(&"c");
        policy.key_used(&"a");

        assert!(policy.evict_next() == "b");

        policy.remove_key(&"c");

        assert!(policy.evict_next() == "a");
    }
}