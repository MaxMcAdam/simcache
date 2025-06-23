pub trait EvictionPolicy<K> {
    fn evict_next(&mut self) -> K;
    fn key_used(&mut self, key: &K);
    fn remove_key(&mut self, key: &K);
    fn new() -> Self;
}

