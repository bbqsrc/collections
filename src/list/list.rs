use crate::Collection;

pub trait List<T>: Collection<T> {
    type Slice<'a>: AsRef<[T]> + PartialEq<&'a [T]> + core::fmt::Debug
    where
        T: 'a,
        Self: 'a;

    fn as_slice(&self) -> Self::Slice<'_>;

    fn find_index(&self, other: &T) -> Option<usize>
    where
        T: PartialEq;

    fn first(&self) -> Option<&T>;
    fn last(&self) -> Option<&T>;
    fn get(&self, index: usize) -> Option<&T>;

    fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord;
    fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> core::cmp::Ordering;
    fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord;

    fn starts_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq;
    fn ends_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq;
}
