#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use std::collections::{btree_map, BTreeMap};

pub trait Map {
    type Key;
    type Value;
    type Iter<'a, X: 'a, Y: 'a>: Iterator<Item = (&'a X, &'a Y)>;

    fn new() -> Self;
    fn iter<'a>(&'a self) -> Self::Iter<'a, Self::Key, Self::Value>
    where
        Self::Key: 'a,
        Self::Value: 'a;
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
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
}

pub struct OrderedMapIter<'a, K, V> {
    inner: btree_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for OrderedMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct OrderedMapKind;

impl<K, V> MapKind<K, V> for OrderedMapKind
where
    K: Ord,
{
    type Map = OrderedMap<K, V>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_it<'a, M>()
    where
        M: MapKind<char, u8> + 'a,
    {
        let mut map = <M::Map as Map>::new();
        map.insert('a', 1);
        map.insert('b', 2);
        map.insert('c', 3);

        assert_eq!(map.get(&'b'), Some(&2));
        assert_eq!(map.get(&'x'), None);

        {
            for (k, v) in M::iter(&map) {}
        }
    }

    #[test]
    fn test() {
        do_it::<OrderedMapKind<'static, char, u8>>();
    }
}
