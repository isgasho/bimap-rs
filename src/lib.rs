mod internal;
mod ordered;
mod unordered;

use crate::internal::Map;
pub use crate::{
    ordered::{Ordered, OrderedMap},
    unordered::{Unordered, UnorderedMap},
};
use std::hash::Hash;

pub trait MapKind<K, V> {
    type Map: Map<Key = K, Value = V>;
}

pub type BiBTreeMap<L, R> = BiMap<L, R, Ordered, Ordered>;
pub type BiHashMap<L, R> = BiMap<L, R, Unordered, Unordered>;

pub struct BiMap<L, R, LMap = Unordered, RMap = Unordered>
where
    LMap: MapKind<L, R>,
    RMap: MapKind<R, L>,
{
    left_map: LMap::Map,
    right_map: RMap::Map,
}

impl<L, R> BiHashMap<L, R>
where
    L: Eq + Hash,
    R: Eq + Hash,
{
}

impl<L, R, LMap, RMap> BiMap<L, R, LMap, RMap>
where
    LMap: MapKind<L, R>,
    RMap: MapKind<R, L>,
{
    pub fn new() -> Self {
        Self {
            left_map: LMap::Map::new(),
            right_map: RMap::Map::new(),
        }
    }

    pub fn get_by_left(&mut self, left: &L) -> Option<&R> {
        self.left_map.get(left).map(|ptr| unsafe { &*ptr })
    }

    pub fn get_by_right(&mut self, right: &R) -> Option<&L> {
        self.right_map.get(right).map(|ptr| unsafe { &*ptr })
    }

    pub fn remove_by_left(&mut self, left: &L) -> Option<(L, R)> {
        self.left_map
            .remove_entry(left)
            .map(|(left_box, right_ptr)| {
                let right_ref = unsafe { &*right_ptr };
                let (right_box, _left_ptr) = self.right_map.remove_entry(right_ref).unwrap();
                (*left_box, *right_box)
            })
    }

    pub fn insert_unchecked(&mut self, left: L, right: R) {
        let left_box = Box::new(left);
        let right_box = Box::new(right);

        let left_ptr: *const L = &*left_box;
        let right_ptr: *const R = &*right_box;

        self.left_map.insert(left_box, right_ptr);
        self.right_map.insert(right_box, left_ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bimap = BiMap::<char, u8>::new();

        bimap.insert_unchecked('a', 1);
        bimap.insert_unchecked('b', 2);
        bimap.insert_unchecked('c', 3);

        assert_eq!(bimap.get_by_left(&'a'), Some(&1));
        assert_eq!(bimap.get_by_left(&'x'), None);

        assert_eq!(bimap.get_by_right(&2), Some(&'b'));
        assert_eq!(bimap.get_by_right(&5), None);
    }
}
