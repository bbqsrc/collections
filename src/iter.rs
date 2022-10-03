pub trait Iterable {
    type Item<'collection>
    where
        Self: 'collection;

    type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
    where
        Self: 'collection;

    fn iter<'c>(&'c self) -> Self::Iterator<'c>;
}

pub trait IterableMut {
    type ItemMut<'collection>
    where
        Self: 'collection;

    type IteratorMut<'collection>: Iterator<Item = Self::ItemMut<'collection>>
    where
        Self: 'collection;

    fn iter_mut<'c>(&'c mut self) -> Self::IteratorMut<'c>;
}
