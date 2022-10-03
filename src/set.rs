use core::{borrow::Borrow, hash::Hash};

use crate::{Collection, CollectionMut};

pub trait Set<T>: Collection<T> {
    fn len(&self) -> usize;
    fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq;
}

pub trait SetMut<T>: CollectionMut<T> {
    fn insert(&mut self, key: T) -> bool;
    fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq;
    fn capacity(&mut self) -> usize;
}
