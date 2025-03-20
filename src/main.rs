use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

pub struct Simcache<K, V> {
    store: HashMap<K, (V, Option<Instant>)>,
}

impl<K, V> Simcache<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
    {
        /// return a new, empty cache
        pub fn new() -> Self {
            Simcache {
                store: HashMap::new(),
            }
        }

        /// return a new, empty cache with the specified capacity
        pub fn new_with_capacity(capacity: usize) -> Self {
            Simcache {
                store: HashMap::with_capacity(capacity),
            }
        }

        /// insert a key value pair into the cache
        /// option to include a ttl for the item
        pub fn insert(&mut self, key: K, value: V, ttl: Option<Duration>) {
            match ttl {
                Some(x) => {self.store.insert(key, (value, Some(Instant::now() + x)));},
                None => {self.store.insert(key, (value, None));}
            }
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

            return self.store.get(key).map(|(val, _)| val)
        }

        /// remove the key value pair with the given key from the cache
        pub fn remove(&mut self, key: &K) -> Option<V> {
            self.store.remove(key).map(|(value, _)| value)
        }

        /// return the current size of the cache
        pub fn len(&self) -> usize {
            return self.store.len()
        }
    }

    fn main() {
        let mut cache = Simcache::new();

        cache.insert("key1", "val1", None);
        cache.insert("key2", "val2", Some(Duration::from_secs(3)));

        println!("key1: {:?}", cache.get(&"key1"));
        println!("key2: {:?}", cache.get(&"key2"));

        std::thread::sleep(Duration::from_secs(4));

        println!("After expiration time - key2: {:?}", cache.get(&"key2"));
    }