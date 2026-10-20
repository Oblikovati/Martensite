//! `BRepLib_MakeVertex`: a vertex at a point with the default precision.
use super::globals;
use super::make_shape::BRepLibMakeShape;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Vertex};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use std::ops::{Deref, DerefMut};

/// `BRepLib_MakeVertex`: a vertex at a point, of
/// tolerance `BRepLib::Precision()`; always done.
///
/// ```
/// use martensite_tkmath::gp::Point3;
/// use martensite_tktopalgo::brep_lib::BRepLibMakeVertex;
/// let maker = BRepLibMakeVertex::new(Point3::new(1.0, 2.0, 3.0)).unwrap();
/// assert!(maker.is_done());
/// assert!(!maker.vertex().unwrap().is_null());
/// ```
#[derive(Clone)]
pub struct BRepLibMakeVertex {
    base: BRepLibMakeShape,
}

impl Deref for BRepLibMakeVertex {
    type Target = BRepLibMakeShape;
    fn deref(&self) -> &BRepLibMakeShape {
        &self.base
    }
}

impl DerefMut for BRepLibMakeVertex {
    fn deref_mut(&mut self) -> &mut BRepLibMakeShape {
        &mut self.base
    }
}

impl BRepLibMakeVertex {
    /// `BRepLib_MakeVertex(P)`: `BRep_Builder::MakeVertex(V, P, BRepLib::Precision())`;
    /// `BRepLibMakeVertex::new(point)?`.
    pub fn new(point: Point3) -> Result<Self, Failure> {
        let mut base = BRepLibMakeShape::new();
        let mut vertex = Vertex::default();
        BRepBuilder::default().make_vertex_at(&mut vertex, point, globals::precision())?;
        base.my_shape = vertex.into_shape();
        base.set_done();
        Ok(Self { base })
    }

    /// `Vertex()` and `operator TopoDS_Vertex()`: the vertex; `maker.vertex()?`.
    pub fn vertex(&self) -> Result<Vertex, Failure> {
        topods::vertex(self.shape()?)
    }
}
