//! TopTools: the shape hasher and maps, the location set and the shape set of the
//! BRep ASCII format.
//!
//! The native `Standard_OStream`/`Standard_IStream` are the TKernel iostream model
//! [`IoStream`](martensite_tkernel::storage::IoStream), and the text the native code
//! writes to `std::cout` goes to the injected GeomTools
//! [`ConsoleOutput`](martensite_tkgeombase::geom_tools::ConsoleOutput).
pub mod format_version;
mod location_set;
mod location_text;
mod shape_map_hasher;
mod shape_set;
pub mod shape_set_io;
mod shape_set_read;
pub(crate) mod shape_set_text;

pub use location_set::LocationSet;
pub use shape_map_hasher::{
    IndexedDataMapOfShapeListOfShape, IndexedMapOfShape, MapOfShape, ShapeMapHasher,
};
pub use shape_set::{ShapeSet, ShapeSetData, THE_ASCII_VERSIONS, TopToolsShapeSet};
