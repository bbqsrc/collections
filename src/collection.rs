pub trait Collection<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn contains(&self, other: &T) -> bool
    where
        T: PartialEq;
}

pub trait CollectionMut<T>: Collection<T> {
    fn clear(&mut self);
}
