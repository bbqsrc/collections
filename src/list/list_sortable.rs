use crate::ListMut;

pub trait ListSortable<T>: ListMut<T> {
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
}
