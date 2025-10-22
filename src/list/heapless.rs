use super::{CapacityError, List, ListMut};
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
    pub(crate) fn push<T, const N: usize>(vec: &mut heapless::Vec<T, N>, item: T) -> Result<(), T> {
        vec.push(item)
    }

    #[inline(always)]
    pub(crate) fn pop<T, const N: usize>(vec: &mut heapless::Vec<T, N>) -> Option<T> {
        vec.pop()
    }

    #[inline(always)]
    pub(crate) fn first<T, const N: usize>(vec: &heapless::Vec<T, N>) -> Option<&T> {
        vec.first()
    }

    #[inline(always)]
    pub(crate) fn last<T, const N: usize>(vec: &heapless::Vec<T, N>) -> Option<&T> {
        vec.last()
    }

    #[inline(always)]
    pub(crate) fn get<T, const N: usize>(vec: &heapless::Vec<T, N>, index: usize) -> Option<&T> {
        vec.get(index)
    }

    #[inline(always)]
    pub(crate) fn binary_search<T: Ord, const N: usize>(
        vec: &heapless::Vec<T, N>,
        x: &T,
    ) -> Result<usize, usize> {
        vec.binary_search(x)
    }

    #[inline(always)]
    pub(crate) fn binary_search_by<T, F, const N: usize>(
        vec: &heapless::Vec<T, N>,
        f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        vec.binary_search_by(f)
    }

    #[inline(always)]
    pub(crate) fn binary_search_by_key<T, B, F, const N: usize>(
        vec: &heapless::Vec<T, N>,
        b: &B,
        f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        vec.binary_search_by_key(b, f)
    }

    #[inline(always)]
    pub(crate) fn starts_with<T: PartialEq, const N: usize>(
        vec: &heapless::Vec<T, N>,
        needle: &[T],
    ) -> bool {
        vec.starts_with(needle)
    }

    #[inline(always)]
    pub(crate) fn ends_with<T: PartialEq, const N: usize>(
        vec: &heapless::Vec<T, N>,
        needle: &[T],
    ) -> bool {
        vec.ends_with(needle)
    }

    #[inline(always)]
    pub(crate) fn first_mut<T, const N: usize>(vec: &mut heapless::Vec<T, N>) -> Option<&mut T> {
        vec.first_mut()
    }

    #[inline(always)]
    pub(crate) fn last_mut<T, const N: usize>(vec: &mut heapless::Vec<T, N>) -> Option<&mut T> {
        vec.last_mut()
    }

    #[inline(always)]
    pub(crate) fn get_mut<T, const N: usize>(
        vec: &mut heapless::Vec<T, N>,
        index: usize,
    ) -> Option<&mut T> {
        vec.get_mut(index)
    }

    #[inline(always)]
    pub(crate) fn insert<T, const N: usize>(
        vec: &mut heapless::Vec<T, N>,
        index: usize,
        element: T,
    ) -> Result<(), T> {
        vec.insert(index, element)
    }

    #[inline(always)]
    pub(crate) fn remove<T, const N: usize>(vec: &mut heapless::Vec<T, N>, index: usize) -> T {
        vec.remove(index)
    }

    #[inline(always)]
    pub(crate) fn swap_remove<T, const N: usize>(vec: &mut heapless::Vec<T, N>, index: usize) -> T {
        vec.swap_remove(index)
    }

    #[inline(always)]
    pub(crate) fn swap<T, const N: usize>(vec: &mut heapless::Vec<T, N>, a: usize, b: usize) {
        vec.swap(a, b)
    }

    #[inline(always)]
    pub(crate) fn reverse<T, const N: usize>(vec: &mut heapless::Vec<T, N>) {
        vec.reverse()
    }

    #[inline(always)]
    pub(crate) fn truncate<T, const N: usize>(vec: &mut heapless::Vec<T, N>, len: usize) {
        vec.truncate(len)
    }

    #[inline(always)]
    pub(crate) fn retain<T, F, const N: usize>(vec: &mut heapless::Vec<T, N>, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        vec.retain(f)
    }

    #[inline(always)]
    pub(crate) fn retain_mut<T, F, const N: usize>(vec: &mut heapless::Vec<T, N>, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        vec.retain_mut(f)
    }

    pub(crate) fn dedup<T: PartialEq, const N: usize>(vec: &mut heapless::Vec<T, N>) {
        let len = vec.len();
        if len <= 1 {
            return;
        }

        let mut write = 1;
        for read in 1..len {
            if vec[read - 1] != vec[read] {
                if write != read {
                    vec.swap(write, read);
                }
                write += 1;
            }
        }
        vec.truncate(write);
    }

    pub(crate) fn dedup_by<T, F, const N: usize>(vec: &mut heapless::Vec<T, N>, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = vec.len();
        if len <= 1 {
            return;
        }

        let ptr = vec.as_mut_ptr();
        let mut write = 1;

        for read in 1..len {
            unsafe {
                if !same_bucket(&mut *ptr.add(write - 1), &mut *ptr.add(read)) {
                    if write != read {
                        core::ptr::swap(ptr.add(write), ptr.add(read));
                    }
                    write += 1;
                }
            }
        }
        vec.truncate(write);
    }

    pub(crate) fn dedup_by_key<T, F, K, const N: usize>(vec: &mut heapless::Vec<T, N>, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        let len = vec.len();
        if len <= 1 {
            return;
        }

        let ptr = vec.as_mut_ptr();
        let mut write = 1;

        for read in 1..len {
            unsafe {
                let prev_key = key(&mut *ptr.add(write - 1));
                let curr_key = key(&mut *ptr.add(read));
                if prev_key != curr_key {
                    if write != read {
                        core::ptr::swap(ptr.add(write), ptr.add(read));
                    }
                    write += 1;
                }
            }
        }
        vec.truncate(write);
    }

    #[inline(always)]
    pub(crate) fn fill<T: Clone, const N: usize>(vec: &mut heapless::Vec<T, N>, value: T) {
        vec.fill(value)
    }

    #[inline(always)]
    pub(crate) fn fill_with<T, F, const N: usize>(vec: &mut heapless::Vec<T, N>, f: F)
    where
        F: FnMut() -> T,
    {
        vec.fill_with(f)
    }

    #[inline(always)]
    pub(crate) fn append<T: Clone, const N: usize>(
        vec: &mut heapless::Vec<T, N>,
        other: &mut heapless::Vec<T, N>,
    ) -> Result<(), super::CapacityError> {
        vec.extend_from_slice(other)
            .map_err(|_| super::CapacityError)?;
        other.clear();
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn split_off<T, const N: usize>(
        vec: &mut heapless::Vec<T, N>,
        at: usize,
    ) -> heapless::Vec<T, N>
    where
        T: Clone,
    {
        let mut new_vec = heapless::Vec::new();
        if at < vec.len() {
            let _ = new_vec.extend_from_slice(&vec[at..]);
            vec.truncate(at);
        }
        new_vec
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

    #[inline(always)]
    fn first(&self) -> Option<&T> {
        inner_vec::first(self)
    }

    #[inline(always)]
    fn last(&self) -> Option<&T> {
        inner_vec::last(self)
    }

    #[inline(always)]
    fn get(&self, index: usize) -> Option<&T> {
        inner_vec::get(self, index)
    }

    #[inline(always)]
    fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        inner_vec::binary_search(self, x)
    }

    #[inline(always)]
    fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        inner_vec::binary_search_by(self, f)
    }

    #[inline(always)]
    fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        inner_vec::binary_search_by_key(self, b, f)
    }

    #[inline(always)]
    fn starts_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        inner_vec::starts_with(self, needle)
    }

    #[inline(always)]
    fn ends_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        inner_vec::ends_with(self, needle)
    }
}

impl<T, const N: usize> ListMut<T> for heapless::Vec<T, N> {
    #[inline(always)]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    #[inline(always)]
    fn push(&mut self, item: T) -> Result<(), T> {
        inner_vec::push(self, item)
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        inner_vec::pop(self)
    }

    #[inline(always)]
    fn first_mut(&mut self) -> Option<&mut T> {
        inner_vec::first_mut(self)
    }

    #[inline(always)]
    fn last_mut(&mut self) -> Option<&mut T> {
        inner_vec::last_mut(self)
    }

    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        inner_vec::get_mut(self, index)
    }

    #[inline(always)]
    fn insert(&mut self, index: usize, element: T) -> Result<(), T> {
        inner_vec::insert(self, index, element)
    }

    #[inline(always)]
    fn remove(&mut self, index: usize) -> T {
        inner_vec::remove(self, index)
    }

    #[inline(always)]
    fn swap_remove(&mut self, index: usize) -> T {
        inner_vec::swap_remove(self, index)
    }

    #[inline(always)]
    fn swap(&mut self, a: usize, b: usize) {
        inner_vec::swap(self, a, b)
    }

    #[inline(always)]
    fn reverse(&mut self) {
        inner_vec::reverse(self)
    }

    #[inline(always)]
    fn truncate(&mut self, len: usize) {
        inner_vec::truncate(self, len)
    }

    #[inline(always)]
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        inner_vec::retain(self, f)
    }

    #[inline(always)]
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        inner_vec::retain_mut(self, f)
    }

    #[inline(always)]
    fn dedup(&mut self)
    where
        T: PartialEq,
    {
        inner_vec::dedup(self)
    }

    #[inline(always)]
    fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        inner_vec::dedup_by(self, same_bucket)
    }

    #[inline(always)]
    fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        inner_vec::dedup_by_key(self, key)
    }

    #[inline(always)]
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        inner_vec::fill(self, value)
    }

    #[inline(always)]
    fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T,
    {
        inner_vec::fill_with(self, f)
    }

    #[inline(always)]
    fn append(&mut self, other: &mut Self) -> Result<(), CapacityError>
    where
        T: Clone,
    {
        inner_vec::append(self, other)
    }

    #[inline(always)]
    fn split_off(&mut self, at: usize) -> Self
    where
        T: Clone,
    {
        inner_vec::split_off(self, at)
    }
}
