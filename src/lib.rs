mod internal;
pub mod ordered;
pub mod unordered;

use crate::{internal::Map, ordered::Ordered, unordered::Unordered};
use std::hash::Hash;

pub trait MapKind<K, V> {
    type Map: Map<Key = K, Value = V>;
}

pub type BiBTreeMap<L, R> = BiMap<L, R, Ordered, Ordered>;
pub type BiHashMap<L, R> = BiMap<L, R, Unordered, Unordered>;

pub enum Overwritten<L, R> {
    Neither,
    Left(L, R),
    Right(L, R),
    Pair(L, R),
    Both((L, R), (L, R)),
}

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
    L: Eq,
    R: Eq,
    LMap: MapKind<L, R>,
    RMap: MapKind<R, L>,
{
    pub fn new() -> Self {
        Self {
            left_map: LMap::Map::new(),
            right_map: RMap::Map::new(),
        }
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.left_map.len(), self.right_map.len());
        self.left_map.len()
    }

    pub fn get_by_left(&mut self, left: &L) -> Option<&R> {
        self.left_map.get(left).map(|ptr| unsafe { &*ptr })
    }

    pub fn get_by_right(&mut self, right: &R) -> Option<&L> {
        self.right_map.get(right).map(|ptr| unsafe { &*ptr })
    }

    pub fn contains_left(&mut self, left: &L) -> bool {
        self.left_map.contains_key(left)
    }

    pub fn contains_right(&mut self, right: &R) -> bool {
        self.right_map.contains_key(right)
    }

    pub fn contains_pair(&mut self, left: &L, right: &R) -> bool {
        self.contains_left(left) && self.contains_right(right)
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

    pub fn remove_by_right(&mut self, right: &R) -> Option<(L, R)> {
        self.right_map
            .remove_entry(right)
            .map(|(right_box, left_ptr)| {
                let left_ref = unsafe { &*left_ptr };
                let (left_box, _right_ptr) = self.left_map.remove_entry(left_ref).unwrap();
                (*left_box, *right_box)
            })
    }

    pub fn insert(&mut self, left: L, right: R) -> Overwritten<L, R> {
        let overwritten = match (self.remove_by_left(&left), self.remove_by_right(&right)) {
            (None, None) => Overwritten::Neither,
            (None, Some((l, r))) => Overwritten::Right(l, r),
            (Some((l, r)), None) => {
                // since remove_by_left() was called first, it's possible the right value was
                // removed if a duplicate pair is being inserted
                if r == right {
                    Overwritten::Pair(l, r)
                } else {
                    Overwritten::Left(l, r)
                }
            }
            (Some(left_pair), Some(right_pair)) => Overwritten::Both(left_pair, right_pair),
        };
        self.insert_unchecked(left, right);
        overwritten
    }

    pub fn try_insert(&mut self, left: L, right: R) -> Result<(), (L, R)> {
        if self.contains_left(&left) || self.contains_right(&right) {
            Err((left, right))
        } else {
            self.insert_unchecked(left, right);
            Ok(())
        }
    }

    fn insert_unchecked(&mut self, left: L, right: R) {
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
