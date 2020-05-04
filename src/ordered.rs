use crate::{internal::Map, MapKind};
use std::collections::BTreeMap;

pub struct Ordered;

impl<K, V> MapKind<K, V> for Ordered
where
    K: Ord,
{
    type Map = OrderedMap<K, V>;
}

pub struct OrderedMap<K, V>(BTreeMap<Box<K>, *const V>);

impl<K, V> Map for OrderedMap<K, V>
where
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }

    fn get(&self, key: &K) -> Option<*const V> {
        self.0.get(key).copied()
    }

    fn insert(&mut self, key: Box<K>, value: *const V) {
        self.0.insert(key, value);
    }

    fn remove_entry(&mut self, key: &K) -> Option<(Box<K>, *const V)> {
        self.0.remove_entry(key)
    }
}
