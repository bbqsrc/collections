use alloc::vec::Vec;

use super::{List, ListMut, ListResizable, ListSortable};
use crate::{Collection, CollectionMut, Error, Iterable, IterableMut};

mod inner_vec {
    use alloc::vec::Vec;
    use core::{
        panic::AssertUnwindSafe,
        slice::{Iter, IterMut},
    };

    #[cfg(feature = "std")]
    use std::panic::catch_unwind;

    use crate::Error;

    #[cfg(not(feature = "std"))]
    fn catch_unwind<F: FnOnce() -> R + core::panic::UnwindSafe, R>(
        f: F,
    ) -> Result<R, alloc::boxed::Box<dyn core::any::Any + Send + 'static>> {
        Ok(f())
    }

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

    #[inline(always)]
    pub(crate) fn first<T>(vec: &Vec<T>) -> Option<&T> {
        vec.first()
    }

    #[inline(always)]
    pub(crate) fn last<T>(vec: &Vec<T>) -> Option<&T> {
        vec.last()
    }

    #[inline(always)]
    pub(crate) fn get<T>(vec: &Vec<T>, index: usize) -> Option<&T> {
        vec.get(index)
    }

    #[inline(always)]
    pub(crate) fn binary_search<T: Ord>(vec: &Vec<T>, x: &T) -> Result<usize, usize> {
        vec.binary_search(x)
    }

    #[inline(always)]
    pub(crate) fn binary_search_by<T, F>(vec: &Vec<T>, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        vec.binary_search_by(f)
    }

    #[inline(always)]
    pub(crate) fn binary_search_by_key<T, B, F>(vec: &Vec<T>, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        vec.binary_search_by_key(b, f)
    }

    #[inline(always)]
    pub(crate) fn starts_with<T: PartialEq>(vec: &Vec<T>, needle: &[T]) -> bool {
        vec.starts_with(needle)
    }

    #[inline(always)]
    pub(crate) fn ends_with<T: PartialEq>(vec: &Vec<T>, needle: &[T]) -> bool {
        vec.ends_with(needle)
    }

    #[inline(always)]
    pub(crate) fn push<T>(vec: &mut Vec<T>, item: T) -> Result<(), Error<T>> {
        use core::mem::ManuallyDrop;

        let item = ManuallyDrop::new(item);
        let self_ptr = vec as *mut Vec<T>;

        match catch_unwind(AssertUnwindSafe(|| unsafe {
            (*self_ptr).push(core::ptr::read(&*item));
        })) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::InsertFailed(ManuallyDrop::into_inner(item))),
        }
    }

    pub(crate) fn pop<T>(vec: &mut Vec<T>) -> Option<T> {
        vec.pop()
    }

    #[inline(always)]
    pub(crate) fn first_mut<T>(vec: &mut Vec<T>) -> Option<&mut T> {
        vec.first_mut()
    }

    #[inline(always)]
    pub(crate) fn last_mut<T>(vec: &mut Vec<T>) -> Option<&mut T> {
        vec.last_mut()
    }

    #[inline(always)]
    pub(crate) fn get_mut<T>(vec: &mut Vec<T>, index: usize) -> Option<&mut T> {
        vec.get_mut(index)
    }

    #[inline(always)]
    pub(crate) fn insert<T>(vec: &mut Vec<T>, index: usize, element: T) -> Result<(), Error<T>> {
        use core::mem::ManuallyDrop;

        let element = ManuallyDrop::new(element);
        let self_ptr = vec as *mut Vec<T>;

        match catch_unwind(AssertUnwindSafe(|| unsafe {
            (*self_ptr).insert(index, core::ptr::read(&*element));
        })) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::InsertFailed(ManuallyDrop::into_inner(element))),
        }
    }

    #[inline(always)]
    pub(crate) fn remove<T>(vec: &mut Vec<T>, index: usize) -> T {
        vec.remove(index)
    }

    #[inline(always)]
    pub(crate) fn swap_remove<T>(vec: &mut Vec<T>, index: usize) -> T {
        vec.swap_remove(index)
    }

    #[inline(always)]
    pub(crate) fn swap<T>(vec: &mut Vec<T>, a: usize, b: usize) {
        vec.swap(a, b)
    }

    #[inline(always)]
    pub(crate) fn reverse<T>(vec: &mut Vec<T>) {
        vec.reverse()
    }

    #[inline(always)]
    pub(crate) fn sort<T: Ord>(vec: &mut Vec<T>) {
        vec.sort()
    }

    #[inline(always)]
    pub(crate) fn sort_by<T, F>(vec: &mut Vec<T>, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        vec.sort_by(compare)
    }

    #[inline(always)]
    pub(crate) fn sort_by_key<T, K, F>(vec: &mut Vec<T>, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        vec.sort_by_key(f)
    }

    #[inline(always)]
    pub(crate) fn sort_unstable<T: Ord>(vec: &mut Vec<T>) {
        vec.sort_unstable()
    }

    #[inline(always)]
    pub(crate) fn sort_unstable_by<T, F>(vec: &mut Vec<T>, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        vec.sort_unstable_by(compare)
    }

    #[inline(always)]
    pub(crate) fn sort_unstable_by_key<T, K, F>(vec: &mut Vec<T>, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        vec.sort_unstable_by_key(f)
    }

    #[inline(always)]
    pub(crate) fn truncate<T>(vec: &mut Vec<T>, len: usize) {
        vec.truncate(len)
    }

    #[inline(always)]
    pub(crate) fn resize<T: Clone>(
        vec: &mut Vec<T>,
        new_len: usize,
        value: T,
    ) -> Result<(), Error<T>> {
        let self_ptr = vec as *mut Vec<T>;
        let value_ptr = &value as *const T;

        match catch_unwind(AssertUnwindSafe(|| unsafe {
            (*self_ptr).resize(new_len, (*value_ptr).clone());
        })) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::CapacityExceeded),
        }
    }

    #[inline(always)]
    pub(crate) fn resize_with<T, F>(
        vec: &mut Vec<T>,
        new_len: usize,
        mut f: F,
    ) -> Result<(), Error<T>>
    where
        F: FnMut() -> T,
    {
        let self_ptr = vec as *mut Vec<T>;
        let f_ptr = &mut f as *mut F;

        match catch_unwind(AssertUnwindSafe(|| unsafe {
            (*self_ptr).resize_with(new_len, || (*f_ptr)());
        })) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::CapacityExceeded),
        }
    }

    #[inline(always)]
    pub(crate) fn reserve<T>(vec: &mut Vec<T>, additional: usize) {
        vec.reserve(additional)
    }

    #[inline(always)]
    pub(crate) fn shrink_to_fit<T>(vec: &mut Vec<T>) {
        vec.shrink_to_fit()
    }

    #[inline(always)]
    pub(crate) fn retain<T, F>(vec: &mut Vec<T>, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        vec.retain(f)
    }

    #[inline(always)]
    pub(crate) fn retain_mut<T, F>(vec: &mut Vec<T>, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        vec.retain_mut(f)
    }

    #[inline(always)]
    pub(crate) fn dedup<T: PartialEq>(vec: &mut Vec<T>) {
        vec.dedup()
    }

    #[inline(always)]
    pub(crate) fn dedup_by<T, F>(vec: &mut Vec<T>, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        vec.dedup_by(same_bucket)
    }

    #[inline(always)]
    pub(crate) fn dedup_by_key<T, F, K>(vec: &mut Vec<T>, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        vec.dedup_by_key(key)
    }

    #[inline(always)]
    pub(crate) fn fill<T: Clone>(vec: &mut Vec<T>, value: T) {
        vec.fill(value)
    }

    #[inline(always)]
    pub(crate) fn fill_with<T, F>(vec: &mut Vec<T>, f: F)
    where
        F: FnMut() -> T,
    {
        vec.fill_with(f)
    }

    #[inline(always)]
    pub(crate) fn append<T>(vec: &mut Vec<T>, other: &mut Vec<T>) -> Result<(), Error<T>> {
        let self_ptr = vec as *mut Vec<T>;
        let other_ptr = other as *mut Vec<T>;

        match catch_unwind(AssertUnwindSafe(|| unsafe {
            (*self_ptr).append(&mut *other_ptr);
        })) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::CapacityExceeded),
        }
    }

    #[inline(always)]
    pub(crate) fn split_off<T>(vec: &mut Vec<T>, at: usize) -> Vec<T> {
        vec.split_off(at)
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

impl<T: PartialEq + core::fmt::Debug> List<T> for Vec<T> {
    type Slice<'a>
        = &'a [T]
    where
        T: 'a;

    #[inline(always)]
    fn as_slice(&self) -> Self::Slice<'_> {
        Vec::as_slice(self)
    }

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

impl<T: PartialEq + core::fmt::Debug> ListMut<T> for Vec<T> {
    #[inline(always)]
    fn new() -> Self
    where
        Self: Sized,
    {
        Vec::new()
    }

    #[inline(always)]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn push(&mut self, item: T) -> Result<(), Error<T>> {
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

    fn insert(&mut self, index: usize, element: T) -> Result<(), Error<T>> {
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

    fn append(&mut self, other: &mut Self) -> Result<(), Error<T>>
    where
        T: Clone,
    {
        inner_vec::append(self, other)
    }

    #[inline(always)]
    fn split_off(&mut self, at: usize) -> Self {
        inner_vec::split_off(self, at)
    }
}

impl<T: PartialEq + core::fmt::Debug> ListResizable<T> for Vec<T> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: T) -> Result<(), Error<T>>
    where
        T: Clone,
    {
        inner_vec::resize(self, new_len, value)
    }

    #[inline(always)]
    fn resize_with<F>(&mut self, new_len: usize, f: F) -> Result<(), Error<T>>
    where
        F: FnMut() -> T,
    {
        inner_vec::resize_with(self, new_len, f)
    }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) {
        inner_vec::reserve(self, additional)
    }

    #[inline(always)]
    fn shrink_to_fit(&mut self) {
        inner_vec::shrink_to_fit(self)
    }
}

impl<T: PartialEq + core::fmt::Debug> ListSortable<T> for Vec<T> {
    #[inline(always)]
    fn sort(&mut self)
    where
        T: Ord,
    {
        inner_vec::sort(self)
    }

    #[inline(always)]
    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        inner_vec::sort_by(self, compare)
    }

    #[inline(always)]
    fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        inner_vec::sort_by_key(self, f)
    }

    #[inline(always)]
    fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        inner_vec::sort_unstable(self)
    }

    #[inline(always)]
    fn sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        inner_vec::sort_unstable_by(self, compare)
    }

    #[inline(always)]
    fn sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        inner_vec::sort_unstable_by_key(self, f)
    }
}
