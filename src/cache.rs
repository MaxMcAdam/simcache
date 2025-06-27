use std::collections::{HashMap};
use std::hash::Hash;
use std::time::{Duration, Instant};
use crate::EvictionPolicy;

/// accessed objects are pushed onto the back of the access_order queue
/// therefore the oldest items are at the front
pub struct Simcache<K, V, E> 
where 
    E: EvictionPolicy<K>
{
    store: HashMap<K, (V, Option<Instant>)>,
    eviction_policy: E,
    max_capacity: usize,
}

impl<K, V, E> Simcache<K, V, E> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
    E: EvictionPolicy<K>,
    {
        /// return a new, empty cache
        pub fn new(max_capacity: usize) -> Self {
            Simcache {
                store: HashMap::new(),
                eviction_policy: E::new(),
                max_capacity,
            }
        }

        /// return a new, empty cache with the specified capacity
        pub fn new_with_capacity(capacity: usize, max_capacity: usize) -> Self {
            Simcache {
                store: HashMap::with_capacity(capacity),
                eviction_policy: E::new(),
                max_capacity,
            }
        }

        /// insert a key value pair into the cache
        /// option to include a ttl for the item
        pub fn insert(&mut self, key: K, value: V, ttl: Option<Duration>) {
            if self.store.len() > self.max_capacity - 1 && self.get(&key).is_none() {
                println!("Evicting");
                let key_to_evict = self.eviction_policy.evict_next();
                self.remove(&key_to_evict);
            }
            match ttl {
                Some(x) => {self.store.insert(key.clone(), (value, Some(Instant::now() + x)));},
                None => {self.store.insert(key.clone(), (value, None));}
            }
            self.eviction_policy.key_used(&key);
        }

        /// return the value of the given key from the cache if it is not expired
        /// or None if it does not exist in the cache or has expired
        pub fn get(&mut self, key: &K) -> Option<&V> {
            // self.store.get() is an immutable borrow
            // therefore, the mutable borrow self.store.remove(key) cannot be called using it
            // so the expiration check and the removal are performed in 2 steps
            let expired = if let Some((_, exp)) = self.store.get(key) {
                if let Some(expiry_time) = exp {
                    if Instant::now() > *expiry_time {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                return None;
            };

            if expired {
                self.store.remove(key);
                return None;
            }

            self.eviction_policy.key_used(&key);
            return self.store.get(key).map(|(val, _)| val)
        }

        /// remove the key value pair with the given key from the cache
        pub fn remove(&mut self, key: &K) -> Option<V> {
            // self.eviction_policy.remove_key(key);
            self.store.remove(key).map(|(value, _)| value)
        }

        /// return the current size of the cache
        pub fn len(&self) -> usize {
            return self.store.len()
        }
    }


    #[cfg(test)]
    mod common {
        use super::*;
        use super::super::eviction::*;

        #[test]
        fn test_cache_lru() {
            let mut cache: Simcache::<&'static str, &'static str, LRU<&'static str>> = Simcache::new(3);

            cache.insert("a", "1", None);
            cache.insert("b", "2", None);
            cache.insert("c", "3", None);

            assert_eq!(cache.len(), 3);

            cache.insert("d", "4", None);

            assert_eq!(cache.get(&"a"), None);

            assert_eq!(*(cache.get(&"b").expect("cache should have a value for key b")), "2");

            cache.insert("e","5", None);

            cache.remove(&"b");

            assert_eq!(cache.len(), 2);
        }

        #[test]
        fn test_cache_lfu() {
            let mut cache: Simcache::<&'static str, &'static str, LFU<&'static str>> = Simcache::new(3);

            cache.insert("a", "1", None);
            cache.insert("b", "2", None);
            cache.insert("c", "3", None);
            cache.get(&"a");
            cache.get(&"c");

            assert_eq!(cache.len(), 3);

            cache.insert("d", "4", None);

            assert_eq!(cache.get(&"b"), None);

            assert_eq!(*(cache.get(&"a").expect("cache should have a value for key a")), "1");

            cache.insert("e","5", None);

            cache.remove(&"a");

            assert_eq!(cache.len(), 2);
        }
    }