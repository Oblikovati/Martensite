mod callbacks;
mod preparation;

use super::{OffsetEdgeGeometry, OffsetVertexGeometry, SimpleOffsetFaceData};
use martensite_tkbrep::brep_tools::BRepToolsModification;
use martensite_tkbrep::topods::{Edge, Face, Shape, Vertex};
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, StandardTransient};

/// Precompute the geometry used by the reference's simple-offset modification callbacks.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkoffset::brep_offset::BRepOffsetSimpleOffset;
/// let modification = BRepOffsetSimpleOffset::new(&Shape::default(), 1.0, 1e-7).unwrap();
/// ```
pub struct BRepOffsetSimpleOffset {
    parent: StandardTransient,
    faces: DataMap<Face, SimpleOffsetFaceData>,
    edges: DataMap<Edge, OffsetEdgeGeometry>,
    vertices: DataMap<Vertex, OffsetVertexGeometry>,
}

impl_transient!(
    BRepOffsetSimpleOffset, "BRepOffset_SimpleOffset", dyn BRepToolsModification,
    parent, interfaces: [dyn BRepToolsModification]
);

impl BRepOffsetSimpleOffset {
    /// Prepare faces, then edges, then vertices, preserving reference ancestor order.
    /// `BRepOffsetSimpleOffset::new(&shape, distance, tolerance)?`.
    pub fn new(shape: &Shape, offset: f64, tolerance: f64) -> Result<Self, Failure> {
        let faces = preparation::offset_faces(shape, offset, tolerance)?;
        let edges = preparation::offset_edges(shape, &faces)?;
        let vertices = preparation::offset_vertices(shape, &edges)?;
        Ok(Self {
            parent: StandardTransient::default(),
            faces,
            edges,
            vertices,
        })
    }
}
