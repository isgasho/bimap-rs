use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub trait Map {
    type Key;
    type Value;

    fn new() -> Self;
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
    fn remove(&mut self, key: &Self::Key) -> Option<(Self::Key, Self::Value)>;
}

pub trait MapKind<K, V> {
    type Map: Map<Key = K, Value = V>;
}

pub struct OrderedMap<K, V> {
    inner: BTreeMap<K, V>,
}

impl<K, V> Map for OrderedMap<K, V>
where
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<(K, V)> {
        self.inner.remove_entry(key)
    }
}

pub struct UnorderedMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> Map for UnorderedMap<K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<(K, V)> {
        self.inner.remove_entry(key)
    }
}

pub struct OrderedMapKind;

impl<K, V> MapKind<K, V> for OrderedMapKind
where
    K: Ord,
{
    type Map = OrderedMap<K, V>;
}

pub struct UnorderedMapKind;

impl<K, V> MapKind<K, V> for UnorderedMapKind
where
    K: Eq + Hash,
{
    type Map = UnorderedMap<K, V>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_it<M>()
    where
        M: MapKind<char, u8>,
    {
        let mut map = M::Map::new();
        map.insert('a', 1);
        map.insert('b', 2);
        map.insert('c', 3);

        assert_eq!(map.get(&'b'), Some(&2));
        assert_eq!(map.get(&'x'), None);
    }

    #[test]
    fn test() {
        do_it::<OrderedMapKind>();
        do_it::<UnorderedMapKind>();
    }
}
