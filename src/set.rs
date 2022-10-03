use crate::{Collection, CollectionMut};

pub trait Set<T>: Collection<T> {
    fn len(&self) -> usize;
    fn get(&self, value: &T) -> Option<&T>;
}

pub trait SetMut<T>: CollectionMut<T> {
    fn insert(&mut self, value: T) -> bool;
    fn remove(&mut self, value: &T) -> bool;
    fn capacity(&mut self) -> usize;
}

#[cfg(feature = "std")]
mod std {
    use std::collections::{hash_set, HashSet};
    use std::hash::Hash;

    use super::{Set, SetMut};
    use crate::{Collection, CollectionMut, Iterable};

    mod inner_hashset {
        use std::collections::hash_set::Iter;
        use std::collections::HashSet;
        use std::hash::Hash;

        #[inline(always)]
        pub(crate) fn len<T>(set: &HashSet<T>) -> usize {
            set.len()
        }

        #[inline(always)]
        pub(crate) fn get<'a, T: Hash + Eq>(set: &'a HashSet<T>, key: &T) -> Option<&'a T> {
            set.get(key)
        }

        #[inline(always)]
        pub(crate) fn iter<T>(set: &HashSet<T>) -> Iter<'_, T> {
            set.iter()
        }

        #[inline(always)]
        pub(crate) fn clear<T>(set: &mut HashSet<T>) {
            set.clear();
        }

        #[inline(always)]
        pub(crate) fn capacity<T>(map: &mut HashSet<T>) -> usize {
            map.capacity()
        }

        #[inline(always)]
        pub(crate) fn insert<T: Hash + Eq>(set: &mut HashSet<T>, key: T) -> bool {
            set.insert(key)
        }

        #[inline(always)]
        pub(crate) fn remove<T: Hash + Eq>(map: &mut HashSet<T>, key: &T) -> bool {
            map.remove(key)
        }
    }

    impl<T> Iterable for HashSet<T> {
        type Item<'collection> = &'collection T
        where
            T: 'collection;
        type Iterator<'collection> = hash_set::Iter<'collection, T>
        where
            T: 'collection;

        #[inline(always)]
        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            inner_hashset::iter(self)
        }
    }

    impl<T> Collection<T> for HashSet<T> {
        fn len(&self) -> usize {
            self.len()
        }

        fn contains(&self, other: &T) -> bool
        where
            T: PartialEq,
        {
            self.iter().find(|x| *x == other).is_some()
        }
    }

    impl<T> CollectionMut<T> for HashSet<T> {
        fn clear(&mut self) {
            inner_hashset::clear(self);
        }
    }

    impl<T: Hash + Eq> Set<T> for HashSet<T> {
        fn len(&self) -> usize {
            inner_hashset::len(self)
        }

        fn get(&self, value: &T) -> Option<&T> {
            inner_hashset::get(self, value)
        }
    }

    impl<T: Hash + Eq> SetMut<T> for HashSet<T> {
        #[inline(always)]
        fn capacity(&mut self) -> usize {
            inner_hashset::capacity(self)
        }

        #[inline(always)]
        fn insert(&mut self, value: T) -> bool {
            inner_hashset::insert(self, value)
        }

        #[inline(always)]
        fn remove(&mut self, value: &T) -> bool {
            inner_hashset::remove(self, value)
        }
    }
}

#[cfg(feature = "std")]
pub use self::std::*;
