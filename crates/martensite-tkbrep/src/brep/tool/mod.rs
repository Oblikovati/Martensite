//! `BRep_Tool`: the geometric queries on BRep shapes.
//!
//! The static members are associated functions of [`BRepTool`], split by what they
//! read: faces (`face.rs`), edge curves (`curves.rs`), the planar projection
//! (`curve_on_plane.rs`), edge polygons (`polygons.rs`), closedness (`closed.rs`), edge
//! flags, ranges and UV points (`edge.rs`), regularities (`continuity.rs`) and vertices
//! (`vertex.rs`). Output parameters become returned values.
mod closed;
mod continuity;
mod curve_on_plane;
mod curves;
mod edge;
mod face;
mod polygons;
mod vertex;

use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::toploc::Location;

/// `BRep_Tool`: static queries on the geometry of faces, edges and vertices.
///
/// ```
/// use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
/// use martensite_tkbrep::topods::Edge;
/// let mut edge = Edge::default();
/// BRepBuilder::default().make_edge(&mut edge).unwrap();
/// assert_eq!(BRepTool::edge_tolerance(&edge).unwrap(), 1e-7);
/// assert!(!BRepTool::is_geometric_edge(&edge).unwrap());
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct BRepTool;

/// The 3D curve of an edge (`Curve(E, L, First, Last)`): the curve, its location and
/// its range; a null curve with the identity and `0, 0` when the edge has none.
#[derive(Clone, Default)]
pub struct EdgeCurve3D {
    /// The 3D curve, null when the edge has none.
    pub curve: Handle<dyn GeomCurve>,
    /// The edge location composed with the representation location.
    pub location: Location,
    /// The first parameter.
    pub first: f64,
    /// The last parameter.
    pub last: f64,
}

/// The p-curve of an edge on a surface (`CurveOnSurface(E, S, L, First, Last,
/// theIsStored)`): the curve, its range and whether it is stored in the edge.
#[derive(Clone, Default)]
pub struct EdgePCurve {
    /// The p-curve, null when the edge has none on the surface.
    pub pcurve: Handle<dyn Geom2dCurve>,
    /// The first parameter.
    pub first: f64,
    /// The last parameter.
    pub last: f64,
    /// False when the curve was not found in the edge and the planar projection ran.
    pub is_stored: bool,
}

/// A curve on surface of an edge (`CurveOnSurface(E, C, S, L, First, Last[, Index])`):
/// the p-curve, its surface, the composed location and the range; null handles, the
/// identity and `0, 0` when there is none.
#[derive(Clone, Default)]
pub struct EdgeCurveOnSurface {
    /// The p-curve.
    pub pcurve: Handle<dyn Geom2dCurve>,
    /// The surface of the p-curve.
    pub surface: Handle<dyn GeomSurface>,
    /// The edge location composed with the representation location.
    pub location: Location,
    /// The first parameter.
    pub first: f64,
    /// The last parameter.
    pub last: f64,
}
