//! Shared datum chains with eagerly computed transformations.
//!
//! A [`Location`] is an immutable, shared list of `(datum, power)` items. Each list node
//! caches the product of its own powered datum and every item behind it, so reading the
//! transformation is a field read that is safe on many threads at once.
mod datum3d;
mod item_location;
mod location;
mod location_ops;
mod slist;
pub use datum3d::Datum3;
pub use item_location::ItemLocation;
pub use location::Location;
use martensite_tkernel::ncollection::{IndexedMap, Map};
pub use slist::{SListNodeOfItemLocation, SListOfItemLocation};
/// Locations hashed by datum identity and power; `MapOfLocation::default()`.
pub type MapOfLocation = Map<Location>;
/// Indexed locations, keyed like [`MapOfLocation`]; `IndexedMapOfLocation::new(1)`.
pub type IndexedMapOfLocation = IndexedMap<Location>;
