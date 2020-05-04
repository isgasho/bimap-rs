use crate::{internal::Map, MapKind};
use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

pub struct Unordered<S = RandomState> {
    marker: PhantomData<S>,
}

impl<K, V, S> MapKind<K, V> for Unordered<S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Map = UnorderedMap<K, V, S>;
}

pub struct UnorderedMap<K, V, S = RandomState> {
    inner: HashMap<Box<K>, *const V, S>,
}

impl<K, V> UnorderedMap<K, V>
where
    K: Eq + Hash,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }
}

impl<K, V, S> UnorderedMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn with_capacity_and_hasher(capacity: usize, hasher_builder: S) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher(capacity, hasher_builder),
        }
    }
}

impl<K, V, S> Map for UnorderedMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self {
            inner: HashMap::with_hasher(S::default()),
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    fn get(&self, key: &K) -> Option<*const V> {
        self.inner.get(key).copied()
    }

    fn insert(&mut self, key: Box<K>, value: *const V) {
        self.inner.insert(key, value);
    }

    fn remove_entry(&mut self, key: &K) -> Option<(Box<K>, *const V)> {
        self.inner.remove_entry(key)
    }
}
