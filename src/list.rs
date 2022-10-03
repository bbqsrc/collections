use core::ops::{Index, IndexMut};

use crate::{Collection, CollectionMut};

pub trait List<T>: Index<usize> + Collection<T> {
    fn find_index(&self, other: &T) -> Option<usize>
    where
        T: PartialEq;
}

pub trait ListMut<T>: IndexMut<usize> + CollectionMut<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn capacity(&self) -> usize;
}

#[cfg(feature = "alloc")]
mod alloc {
    use alloc::vec::Vec;

    use super::{List, ListMut};
    use crate::{Collection, CollectionMut, Iterable, IterableMut};

    mod inner_vec {
        use core::slice::{Iter, IterMut};

        #[inline(always)]
        pub(crate) fn iter<T>(vec: &Vec<T>) -> Iter<'_, T> {
            vec.iter()
        }

        #[inline(always)]
        pub(crate) fn iter_mut<T>(vec: &mut Vec<T>) -> IterMut<'_, T> {
            vec.iter_mut()
        }

        #[inline(always)]
        pub(crate) fn clear<T>(vec: &mut Vec<T>) {
            vec.clear();
        }
    }

    impl<T> Iterable for Vec<T> {
        type Item<'collection> = &'collection T
        where
            T: 'collection;
        type Iterator<'collection> = std::slice::Iter<'collection, T>
        where
            T: 'collection;

        #[inline(always)]
        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            inner_vec::iter(self)
        }
    }

    impl<T> IterableMut for Vec<T> {
        type ItemMut<'collection> = &'collection mut T
        where
            T: 'collection;
        type IteratorMut<'collection> = std::slice::IterMut<'collection, T>
        where
            T: 'collection;

        #[inline(always)]
        fn iter_mut<'c>(&'c mut self) -> Self::IteratorMut<'c> {
            inner_vec::iter_mut(self)
        }
    }

    impl<T> Collection<T> for Vec<T> {
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

    impl<T> CollectionMut<T> for Vec<T> {
        fn clear(&mut self) {
            inner_vec::clear(self);
        }
    }

    impl<T> List<T> for Vec<T> {
        fn find_index(&self, other: &T) -> Option<usize>
        where
            T: PartialEq,
        {
            self.iter().position(|x| x == other)
        }
    }

    impl<T> ListMut<T> for Vec<T> {
        #[inline(always)]
        fn capacity(&self) -> usize {
            self.capacity()
        }

        #[inline(always)]
        fn push(&mut self, item: T) {
            self.push(item)
        }

        #[inline(always)]
        fn pop(&mut self) -> Option<T> {
            self.pop()
        }
    }
}

#[cfg(feature = "alloc")]
pub use self::alloc::*;
