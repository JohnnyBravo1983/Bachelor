use std::collections::HashMap;

pub trait Cache<K, V> {
    fn get(&self, k: &K) -> Option<&V>;
    fn set(&mut self, k: K, v: V);
    fn invalidate(&mut self, k: &K);
}

pub struct StaticCache<K, V> {
    store: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> StaticCache<K, V> {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Cache<K, V> for StaticCache<K, V> {
    fn get(&self, k: &K) -> Option<&V> { self.store.get(k) }
    fn set(&mut self, k: K, v: V) { self.store.insert(k, v); }
    fn invalidate(&mut self, k: &K) { self.store.remove(k); }
}
