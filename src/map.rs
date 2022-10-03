use crate::{Iterable, IterableMut};

pub trait Map<K, V>: Iterable {
    type Keys<'key>: Iterator<Item = &'key K>
    where
        K: 'key,
        Self: 'key;

    type Values<'value>: Iterator<Item = &'value V>
    where
        V: 'value,
        Self: 'value;

    fn len(&self) -> usize;
    fn get(&self, k: &K) -> Option<&V>;
    fn keys<'c>(&'c self) -> Self::Keys<'c>;
    fn values<'c>(&'c self) -> Self::Values<'c>;
}

pub trait MapMut<K, V>: IterableMut {
    type ValuesMut<'value>: Iterator<Item = &'value mut V>
    where
        V: 'value,
        Self: 'value;

    fn values_mut<'c>(&'c mut self) -> Self::ValuesMut<'c>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn capacity(&mut self) -> usize;
}

#[cfg(feature = "std")]
mod std {
    use std::{hash::Hash, collections::{HashMap, hash_map}};

    use super::Map;
    use crate::{Iterable, IterableMut, MapMut};

    mod inner_hashmap {
        use std::{hash::Hash, collections::{hash_map, HashMap}};

        #[inline(always)]
        pub(crate) fn iter<K, V>(map: &HashMap<K, V>) -> hash_map::Iter<'_, K, V> {
            map.iter()
        }

        #[inline(always)]
        pub(crate) fn iter_mut<K, V>(map: &mut HashMap<K, V>) -> hash_map::IterMut<'_, K, V> {
            map.iter_mut()
        }

        #[inline(always)]
        pub(crate) fn keys<K, V>(map: &HashMap<K, V>) -> hash_map::Keys<'_, K, V> {
            map.keys()
        }

        #[inline(always)]
        pub(crate) fn values<K, V>(map: &HashMap<K, V>) -> hash_map::Values<'_, K, V> {
            map.values()
        }

        #[inline(always)]
        pub(crate) fn values_mut<K, V>(map: &mut HashMap<K, V>) -> hash_map::ValuesMut<'_, K, V> {
            map.values_mut()
        }

        #[inline(always)]
        pub(crate) fn len<K, V>(map: &HashMap<K, V>) -> usize {
            map.len()
        }

        #[inline(always)]
        pub(crate) fn capacity<K, V>(map: &HashMap<K, V>) -> usize {
            map.capacity()
        }

        #[inline(always)]
        pub(crate) fn get<'a, K: Hash + Eq, V>(map: &'a HashMap<K, V>, key: &K) -> Option<&'a V> {
            map.get(key)
        }

        #[inline(always)]
        pub(crate) fn insert<K: Hash + Eq, V>(map: &mut HashMap<K, V>, key: K, value: V) -> Option<V> {
            map.insert(key, value)
        }

        #[inline(always)]
        pub(crate) fn remove<K: Hash + Eq, V>(map: &mut HashMap<K, V>, key: &K) -> Option<V> {
            map.remove(key)
        }
    }

    impl<K, V> Iterable for HashMap<K, V> {
        type Item<'collection> = (&'collection K, &'collection V)
        where
            Self: 'collection;

        type Iterator<'collection> = hash_map::Iter<'collection, K, V>
        where
            Self: 'collection;

        #[inline(always)]
        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            inner_hashmap::iter(self)
        }
    }

    impl<K: Hash + Eq, V> Map<K, V> for HashMap<K, V> {
        type Keys<'key> = hash_map::Keys<'key, K, V>
        where
            K: 'key,
            Self: 'key;

        type Values<'value> = hash_map::Values<'value, K, V>
        where
            V: 'value,
            Self: 'value;

        #[inline(always)]
        fn len(&self) -> usize {
            inner_hashmap::len(self)
        }

        #[inline(always)]
        fn get(&self, k: &K) -> Option<&V> {
            inner_hashmap::get(self, k)
        }

        #[inline(always)]
        fn keys<'c>(&'c self) -> Self::Keys<'c> {
            inner_hashmap::keys(self)
        }

        #[inline(always)]
        fn values<'c>(&'c self) -> Self::Values<'c> {
            inner_hashmap::values(self)
        }
    }

    impl<K: Hash + Eq, V> IterableMut for HashMap<K, V> {
        type ItemMut<'collection> = (&'collection K, &'collection mut V)
        where
            Self: 'collection;

        type IteratorMut<'collection> = hash_map::IterMut<'collection, K, V>
        where
            Self: 'collection;

        fn iter_mut<'c>(&'c mut self) -> Self::IteratorMut<'c> {
            inner_hashmap::iter_mut(self)
        }
    }

    impl<K: Hash + Eq, V> MapMut<K, V> for HashMap<K, V> {
        type ValuesMut<'value> = hash_map::ValuesMut<'value, K, V>
        where
            V: 'value,
            Self: 'value;

        #[inline(always)]
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            inner_hashmap::insert(self, key, value)
        }

        #[inline(always)]
        fn remove(&mut self, key: &K) -> Option<V> {
            inner_hashmap::remove(self, key)
        }

        #[inline(always)]
        fn capacity(&mut self) -> usize {
            inner_hashmap::capacity(self)
        }

        #[inline(always)]
        fn values_mut<'c>(&'c mut self) -> Self::ValuesMut<'c> {
            inner_hashmap::values_mut(self)
        }
    }
}

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "alloc")]
mod alloc {
    use alloc::collections::{btree_map, BTreeMap};

    use super::Map;
    use crate::{Iterable, IterableMut, MapMut};

    mod inner_btreemap {
        use alloc::collections::{btree_map, BTreeMap};

        #[inline(always)]
        pub(crate) fn iter<K, V>(map: &BTreeMap<K, V>) -> btree_map::Iter<'_, K, V> {
            map.iter()
        }

        #[inline(always)]
        pub(crate) fn iter_mut<K, V>(map: &mut BTreeMap<K, V>) -> btree_map::IterMut<'_, K, V> {
            map.iter_mut()
        }

        #[inline(always)]
        pub(crate) fn keys<K, V>(map: &BTreeMap<K, V>) -> btree_map::Keys<'_, K, V> {
            map.keys()
        }

        #[inline(always)]
        pub(crate) fn values<K, V>(map: &BTreeMap<K, V>) -> btree_map::Values<'_, K, V> {
            map.values()
        }

        #[inline(always)]
        pub(crate) fn values_mut<K, V>(map: &mut BTreeMap<K, V>) -> btree_map::ValuesMut<'_, K, V> {
            map.values_mut()
        }

        #[inline(always)]
        pub(crate) fn len<K, V>(map: &BTreeMap<K, V>) -> usize {
            map.len()
        }

        #[inline(always)]
        pub(crate) fn get<'a, K: Ord, V>(map: &'a BTreeMap<K, V>, key: &K) -> Option<&'a V> {
            map.get(key)
        }

        #[inline(always)]
        pub(crate) fn insert<K: Ord, V>(map: &mut BTreeMap<K, V>, key: K, value: V) -> Option<V> {
            map.insert(key, value)
        }

        #[inline(always)]
        pub(crate) fn remove<K: Ord, V>(map: &mut BTreeMap<K, V>, key: &K) -> Option<V> {
            map.remove(key)
        }
    }

    impl<K, V> Iterable for BTreeMap<K, V> {
        type Item<'collection> = (&'collection K, &'collection V)
        where
            Self: 'collection;

        type Iterator<'collection> = btree_map::Iter<'collection, K, V>
        where
            Self: 'collection;

        #[inline(always)]
        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            inner_btreemap::iter(self)
        }
    }

    impl<K: Ord, V> Map<K, V> for BTreeMap<K, V> {
        type Keys<'key> = btree_map::Keys<'key, K, V>
        where
            K: 'key,
            Self: 'key;

        type Values<'value> = btree_map::Values<'value, K, V>
        where
            V: 'value,
            Self: 'value;

        #[inline(always)]
        fn len(&self) -> usize {
            inner_btreemap::len(self)
        }

        #[inline(always)]
        fn get(&self, k: &K) -> Option<&V> {
            inner_btreemap::get(self, k)
        }

        #[inline(always)]
        fn keys<'c>(&'c self) -> Self::Keys<'c> {
            inner_btreemap::keys(self)
        }

        #[inline(always)]
        fn values<'c>(&'c self) -> Self::Values<'c> {
            inner_btreemap::values(self)
        }
    }

    impl<K: Ord, V> IterableMut for BTreeMap<K, V> {
        type ItemMut<'collection> = (&'collection K, &'collection mut V)
        where
            Self: 'collection;

        type IteratorMut<'collection> = btree_map::IterMut<'collection, K, V>
        where
            Self: 'collection;

        fn iter_mut<'c>(&'c mut self) -> Self::IteratorMut<'c> {
            inner_btreemap::iter_mut(self)
        }
    }

    impl<K: Ord, V> MapMut<K, V> for BTreeMap<K, V> {
        type ValuesMut<'value> = btree_map::ValuesMut<'value, K, V>
        where
            V: 'value,
            Self: 'value;

        #[inline(always)]
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            inner_btreemap::insert(self, key, value)
        }

        #[inline(always)]
        fn remove(&mut self, key: &K) -> Option<V> {
            inner_btreemap::remove(self, key)
        }

        #[inline(always)]
        fn capacity(&mut self) -> usize {
            // There is no such thing as capacity for a BTreeMap, so we return len.
            self.len()
        }

        #[inline(always)]
        fn values_mut<'c>(&'c mut self) -> Self::ValuesMut<'c> {
            inner_btreemap::values_mut(self)
        }
    }
}

#[cfg(feature = "alloc")]
pub use self::alloc::*;