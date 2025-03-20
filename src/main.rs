use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::time::{Duration, Instant};

/// accessed objects are pushed onto the back of the access_order queue
/// therefore the oldest items are at the front
pub struct Simcache<K, V> {
    store: HashMap<K, (V, Option<Instant>)>,
    access_order: VecDeque<K>,
    max_capacity: usize,
}

impl<K, V> Simcache<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
    {
        /// return a new, empty cache
        pub fn new(max_capacity: usize) -> Self {
            Simcache {
                store: HashMap::new(),
                access_order: VecDeque::new(),
                max_capacity,
            }
        }

        /// return a new, empty cache with the specified capacity
        pub fn new_with_capacity(capacity: usize, max_capacity: usize) -> Self {
            Simcache {
                store: HashMap::with_capacity(capacity),
                access_order: VecDeque::new(),
                max_capacity,
            }
        }

        /// insert a key value pair into the cache
        /// option to include a ttl for the item
        pub fn insert(&mut self, key: K, value: V, ttl: Option<Duration>) {
            if self.len() > self.max_capacity - 1 && self.get(&key).is_none() {
                self.remove_oldest();
            }
            match ttl {
                Some(x) => {self.store.insert(key.clone(), (value, Some(Instant::now() + x)));},
                None => {self.store.insert(key.clone(), (value, None));}
            }
            self.remove_from_access_order(&key);
            self.access_order.push_back(key);
        }

        /// return true if the given key is present in the access_order queue already, false otherwise
        fn remove_from_access_order(&mut self, key: &K) {
            if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                self.access_order.remove(pos);
            }
        }

        /// remove the key:value pair from the cache that was least recently used
        pub fn remove_oldest(&mut self) {
            let oldest_key = self.access_order.pop_front().unwrap();

            self.remove(&oldest_key);
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
            self.remove_from_access_order(key);
            self.store.remove(key).map(|(value, _)| value)
        }

        /// return the current size of the cache
        pub fn len(&self) -> usize {
            return self.store.len()
        }
    }

    fn main() {
        let mut cache = Simcache::new(2);

        cache.insert("key1", "val1", None);
        cache.insert("key2", "val2", Some(Duration::from_secs(3)));
        cache.insert("key3", "val3", None);

        println!("key1: {:?}", cache.get(&"key1"));
        println!("key2: {:?}", cache.get(&"key2"));
        println!("key3: {:?}", cache.get(&"key3"));

        std::thread::sleep(Duration::from_secs(4));

        println!("After expiration time - key2: {:?}", cache.get(&"key2"));
    }