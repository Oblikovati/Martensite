//! Tools shared by the mesh file formats.
//!
//! Integer-backed mesh exchange settings retain explicit and unrecognized values.
mod caf_reader;
mod caf_reader_attributes;
mod caf_reader_fill;
mod converter;
mod coordinate_system;
mod edge_iterator;
mod face_iterator;
mod material_map;
mod material_map_files;
mod name_format;
mod names;
mod node_attributes;
mod number_text;
mod shape_iterator;
mod text_lines;
mod triangulation_reader;
mod triangulation_source;
mod vertex_iterator;

pub use caf_reader::{
    RWMESH_CAF_READER_STATUS_EX_NONE, RWMESH_CAF_READER_STATUS_EX_PARTIAL, RwMeshCafDocumentTools,
    RwMeshCafReader, RwMeshCafReaderBase, RwMeshInputStream,
};
pub use converter::RwMeshCoordinateSystemConverter;
pub use coordinate_system::RwMeshCoordinateSystem;
pub use edge_iterator::RwMeshEdgeIterator;
pub use face_iterator::RwMeshFaceIterator;
pub use material_map::{
    RwMeshMaterialMap, RwMeshMaterialMapBase, RwMeshStyleHashPolicy, RwMeshStyleKey,
    add_material_to,
};
pub use name_format::RwMeshNameFormat;
pub use names::RwMesh;
pub use node_attributes::RwMeshNodeAttributes;
pub use number_text::{double_precision_text, single_precision_text};
pub use shape_iterator::{RwMeshShapeIterator, RwMeshShapeIteratorBase};
pub use text_lines::split_text_lines;
pub use triangulation_reader::{
    RwMeshLoadingStatistic, RwMeshTriangulationReader, RwMeshTriangulationReaderBase,
};
pub use triangulation_source::RwMeshTriangulationSource;
pub use vertex_iterator::RwMeshVertexIterator;

// Configuration readers cast integers directly, so a closed Rust enum would lose state.
macro_rules! mesh_integer_setting {
    ($name:ident, $documentation:literal) => {
        #[doc = $documentation]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name(i32);

        impl $name {
            /// Retains a raw integer, including unrecognized values; `Self::from_raw(42)`.
            pub const fn from_raw(value: i32) -> Self {
                Self(value)
            }

            /// Returns the retained integer without normalization; `self.as_raw()`.
            pub const fn as_raw(self) -> i32 {
                self.0
            }
        }
    };
}
pub(crate) use mesh_integer_setting;

#[cfg(test)]
mod tests {
    use super::{RwMeshCoordinateSystem, RwMeshNameFormat};

    fn assert_send_sync<T: Send + Sync>() {
        assert_eq!(std::mem::size_of::<T>(), 4);
        assert_eq!(std::mem::align_of::<T>(), std::mem::align_of::<i32>());
    }

    #[test]
    fn settings_preserve_every_representative_integer_and_are_publishable_values() {
        assert_send_sync::<RwMeshCoordinateSystem>();
        assert_send_sync::<RwMeshNameFormat>();
        for value in [i32::MIN, -2, -1, 0, 1, 2, 6, 42, i32::MAX] {
            assert_eq!(RwMeshCoordinateSystem::from_raw(value).as_raw(), value);
            assert_eq!(RwMeshNameFormat::from_raw(value).as_raw(), value);
        }
    }
}
