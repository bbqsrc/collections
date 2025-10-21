use crate::{Collection, CollectionMut};

pub trait List<T>: Collection<T> {
    fn find_index(&self, other: &T) -> Option<usize>
    where
        T: PartialEq;
}

pub trait ListMut<T>: CollectionMut<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn capacity(&self) -> usize;
}

#[cfg(feature = "heapless")]
mod heapless {
    use super::{List, ListMut};
    use crate::{Collection, CollectionMut, Iterable, IterableMut};

    mod inner_vec {
        use core::slice::{Iter, IterMut};

        #[inline(always)]
        pub(crate) fn iter<T, const N: usize>(vec: &heapless::Vec<T, N>) -> Iter<'_, T> {
            vec.iter()
        }

        #[inline(always)]
        pub(crate) fn iter_mut<T, const N: usize>(vec: &mut heapless::Vec<T, N>) -> IterMut<'_, T> {
            vec.iter_mut()
        }

        #[inline(always)]
        pub(crate) fn clear<T, const N: usize>(vec: &mut heapless::Vec<T, N>) {
            vec.clear();
        }

        #[inline(always)]
        pub(crate) fn len<T, const N: usize>(vec: &heapless::Vec<T, N>) -> usize {
            vec.len()
        }

        #[inline(always)]
        pub(crate) fn push<T, const N: usize>(
            vec: &mut heapless::Vec<T, N>,
            item: T,
        ) -> Result<(), T> {
            vec.push(item)
        }

        #[inline(always)]
        pub(crate) fn pop<T, const N: usize>(vec: &mut heapless::Vec<T, N>) -> Option<T> {
            vec.pop()
        }
    }

    impl<T, const N: usize> Iterable for heapless::Vec<T, N> {
        type Item<'collection>
            = &'collection T
        where
            T: 'collection;
        type Iterator<'collection>
            = core::slice::Iter<'collection, T>
        where
            T: 'collection;

        #[inline(always)]
        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            inner_vec::iter(self)
        }
    }

    impl<T, const N: usize> IterableMut for heapless::Vec<T, N> {
        type ItemMut<'collection>
            = &'collection mut T
        where
            T: 'collection;
        type IteratorMut<'collection>
            = core::slice::IterMut<'collection, T>
        where
            T: 'collection;

        #[inline(always)]
        fn iter_mut<'c>(&'c mut self) -> Self::IteratorMut<'c> {
            inner_vec::iter_mut(self)
        }
    }

    impl<T, const N: usize> Collection<T> for heapless::Vec<T, N> {
        fn len(&self) -> usize {
            inner_vec::len(self)
        }

        fn contains(&self, other: &T) -> bool
        where
            T: PartialEq,
        {
            self.iter().find(|x| *x == other).is_some()
        }
    }

    impl<T, const N: usize> CollectionMut<T> for heapless::Vec<T, N> {
        fn clear(&mut self) {
            inner_vec::clear(self);
        }
    }

    impl<T, const N: usize> List<T> for heapless::Vec<T, N> {
        fn find_index(&self, other: &T) -> Option<usize>
        where
            T: PartialEq,
        {
            self.iter().position(|x| x == other)
        }
    }

    impl<T, const N: usize> ListMut<T> for heapless::Vec<T, N> {
        #[inline(always)]
        fn capacity(&self) -> usize {
            self.capacity()
        }

        #[inline(always)]
        fn push(&mut self, item: T) {
            let _ = inner_vec::push(self, item);
        }

        #[inline(always)]
        fn pop(&mut self) -> Option<T> {
            inner_vec::pop(self)
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc {
    use alloc::vec::Vec;

    use super::{List, ListMut};
    use crate::{Collection, CollectionMut, Iterable, IterableMut};

    mod inner_vec {
        use alloc::vec::Vec;
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
        type Item<'collection>
            = &'collection T
        where
            T: 'collection;
        type Iterator<'collection>
            = core::slice::Iter<'collection, T>
        where
            T: 'collection;

        #[inline(always)]
        fn iter<'c>(&'c self) -> Self::Iterator<'c> {
            inner_vec::iter(self)
        }
    }

    impl<T> IterableMut for Vec<T> {
        type ItemMut<'collection>
            = &'collection mut T
        where
            T: 'collection;
        type IteratorMut<'collection>
            = core::slice::IterMut<'collection, T>
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
