mod list;
mod list_mut;
mod list_resizable;
mod list_sortable;

#[cfg(feature = "heapless")]
mod heapless;

#[cfg(feature = "alloc")]
mod vec;

pub use list::List;
pub use list_mut::ListMut;
pub use list_resizable::ListResizable;
pub use list_sortable::ListSortable;

/// Error returned when a collection's capacity is exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapacityError;
