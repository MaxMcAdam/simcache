use simcache::{Simcache, eviction::LFU};
use std::time::Duration;

fn main() {
    let mut cache: Simcache::<&'static str, &'static str, LFU<&'static str>> = Simcache::new(2);

    cache.insert("key1", "val1", None);
    cache.insert("key2", "val2", Some(Duration::from_secs(330)));

    cache.get(&"key1");
    cache.get(&"key1");

    cache.insert("key3", "val3", None);
    //cache.insert("key4", "val4", None);
    println!("key1: {:?}", cache.get(&"key1"));
    println!("key2: {:?}", cache.get(&"key2"));
    println!("key3: {:?}", cache.get(&"key3"));

    // std::thread::sleep(Duration::from_secs(4));
    //println!("After adding 4 - key1: {:?}", cache.get(&"key1"));
}