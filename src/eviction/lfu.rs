use crate::EvictionPolicy;
use std::collections::{BTreeMap, HashMap, HashSet};

// Dual data structure allows for all necessary operations to be handled efficiently
// The HashMap stores Key:Count 
// The BTree stores Count:{set of Keys}
pub struct LFU<K>{
    usage_counter: HashMap<K, usize>,
    count_to_key: BTreeMap<usize, std::collections::HashSet<K>>,
} 

impl<K: Clone + Eq + std::hash::Hash> LFU<K> {
    fn update_count_mapping(&mut self, key: &K, old_count: usize, new_count: usize) {
        // Remove from old bucket
        if old_count > 0 {
            if let Some(old_set) = self.count_to_key.get_mut(&old_count) {
                old_set.remove(key);
                if old_set.is_empty() {
                    self.count_to_key.remove(&old_count);
                }
            }
        }
        
        // Add to new bucket
        self.count_to_key
            .entry(new_count)
            .or_insert_with(HashSet::new)
            .insert(key.clone());
    }
}

impl<K: Clone + Eq + std::hash::Hash>EvictionPolicy<K> for LFU<K> {
    fn evict_next(&mut self) -> K {
        let min_count = *self.count_to_key
            .first_key_value()
            .expect("btree should contain at least one value")
            .0;
        
        // Remove the entire entry to get owned access to the HashSet
        let mut key_set = self.count_to_key
            .remove(&min_count)
            .expect("count entry should exist");
        
        // Get a key to evict
        let key_to_evict = key_set.iter().next()
            .expect("key set should not be empty")
            .clone();
        
        // Remove from usage counter
        self.usage_counter.remove(&key_to_evict);
        
        // Remove from the key set
        key_set.remove(&key_to_evict);
        
        // Re-insert the set only if it's not empty
        if !key_set.is_empty() {
            self.count_to_key.insert(min_count, key_set);
        }
        
        key_to_evict
    }
    
    fn key_used(&mut self, key: &K) {
        use std::collections::hash_map::Entry;
        
        let (old_count, new_count) = match self.usage_counter.entry(key.clone()) {
            Entry::Occupied(mut e) => {
                let old = *e.get();
                let new = old.saturating_add(1);
                e.insert(new);
                (old, new)
            }
            Entry::Vacant(e) => {
                e.insert(1);
                (0, 1)
            }
        };
        
        // Handle count_to_key updates...
        self.update_count_mapping(key, old_count, new_count);
    }

    fn remove_key(&mut self, key: &K) {

    }

    fn new() -> Self {
        return LFU{usage_counter: HashMap::new(), count_to_key: BTreeMap::new()}
    }
}