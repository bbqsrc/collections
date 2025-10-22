use crate::{CollectionMut, List};

pub trait ListMut<T>: List<T> + CollectionMut<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn capacity(&self) -> usize;

    fn first_mut(&mut self) -> Option<&mut T>;
    fn last_mut(&mut self) -> Option<&mut T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    fn insert(&mut self, index: usize, element: T);
    fn remove(&mut self, index: usize) -> T;
    fn swap_remove(&mut self, index: usize) -> T;

    fn swap(&mut self, a: usize, b: usize);
    fn reverse(&mut self);

    fn sort(&mut self)
    where
        T: Ord;
    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering;
    fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;
    fn sort_unstable(&mut self)
    where
        T: Ord;
    fn sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering;
    fn sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;

    fn truncate(&mut self, len: usize);

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool;
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool;
    fn dedup(&mut self)
    where
        T: PartialEq;
    fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool;
    fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq;

    fn fill(&mut self, value: T)
    where
        T: Clone;
    fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T;

    fn append(&mut self, other: &mut Self)
    where
        T: Clone;
    fn split_off(&mut self, at: usize) -> Self
    where
        T: Clone;
}
