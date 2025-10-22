#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod collection;
mod iter;
mod list;
mod map;
mod set;

pub use collection::{Collection, CollectionMut};
pub use iter::{Iterable, IterableMut};
pub use list::{List, ListMut, ListResizable, ListSortable};
pub use map::{Map, MapMut};
pub use set::{Set, SetMut};

/// Error returned when a collection's capacity is exceeded.
#[derive(Debug, PartialEq, Eq)]
pub enum Error<T> {
    /// The collection has reached its maximum capacity.
    CapacityExceeded,

    /// Insertion failed due to capacity constraints.
    InsertFailed(T),
}
