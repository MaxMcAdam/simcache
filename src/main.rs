use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::time::{Duration, Instant};

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
                self.eviction_policy.evict_next();
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

            return self.store.get(key).map(|(val, _)| val)
        }

        /// remove the key value pair with the given key from the cache
        pub fn remove(&mut self, key: &K) -> Option<V> {
            self.eviction_policy.remove_key(key);
            self.store.remove(key).map(|(value, _)| value)
        }

        /// return the current size of the cache
        pub fn len(&self) -> usize {
            return self.store.len()
        }
    }

trait EvictionPolicy<K> {
    fn evict_next(&mut self) -> K;
    fn key_used(&mut self, key: &K);
    fn remove_key(&mut self, key: &K);
    fn new() -> Self;
}

struct LRU<K> {access_order: VecDeque<K>}

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

    fn main() {
        let mut cache: Simcache::<&'static str, &'static str, LRU<&'static str>> = Simcache::new(2);

        cache.insert("key1", "val1", None);
        cache.insert("key2", "val2", Some(Duration::from_secs(33)));

        println!("key1: {:?}", cache.get(&"key1"));
        cache.insert("key4", "val4", None);
        println!("key2: {:?}", cache.get(&"key2"));
        println!("key3: {:?}", cache.get(&"key3"));

        // std::thread::sleep(Duration::from_secs(4));
        println!("After adding 4 - key1: {:?}", cache.get(&"key1"));
    }