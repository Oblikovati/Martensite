// Infinite edges can lack vertices, and a spatial curve can be absent.
// Endpoint and tangent queries must handle both conditions.

//! `ShapeAnalysisEdge`: the geometry of an edge (3D curve, p-curves, vertices) and the
//! consistency checks between them.
mod checks;
mod outputs;
mod overlap;
mod same_parameter;
mod tangent;
mod vertex_tolerance;

use crate::shape_extend::{ShapeExtend, ShapeExtendStatus};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Edge, Face, Vertex};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Point2, Vector2};
use martensite_tkmath::toploc::Location;

/// `ShapeAnalysisEdge`: queries the representations of an edge (3D curve, p-curve on a
/// face or on a surface with a location) and its bounding vertices, and checks their
/// consistency. The checks write a ShapeExtend status, read with [`Self::status_flag`].
///
/// The members that return a curve and its range give the curve (null when the edge has
/// none; the `bool` result is `!curve.is_null()`) and the range, swapped for a
/// REVERSED edge when `orient` is set.
///
/// ```
/// use martensite_tkbrep::topods::Edge;
/// use martensite_tkshhealing::shape_analysis::ShapeAnalysisEdge;
/// use martensite_tkshhealing::shape_extend::ShapeExtendStatus;
/// let sae = ShapeAnalysisEdge::new();
/// assert!(sae.status_flag(ShapeExtendStatus::Ok));
/// assert!(sae.first_vertex(&Edge::default()).is_null());
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeAnalysisEdge {
    /// `myStatus` (protected): the ShapeExtend flags of the last check.
    status: i32,
}

/// A 3D curve with its parameter range (the outputs `C3d`, `cf`, `cl`).
pub type Curve3dRange = (Handle<dyn GeomCurve>, f64, f64);

/// A p-curve with its parameter range (the outputs `C2d`, `cf`, `cl`).
pub type PCurveRange = (Handle<dyn Geom2dCurve>, f64, f64);

/// Swaps the range of a REVERSED edge when `orient` is set.
fn oriented_range(edge: &Edge, first: f64, last: f64, orient: bool) -> (f64, f64) {
    if orient && edge.orientation() == Orientation::Reversed {
        return (last, first);
    }
    (first, last)
}

impl ShapeAnalysisEdge {
    /// the status OK; `ShapeAnalysisEdge::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// the edge has a 3D curve; `sae.has_curve_3d(&edge)?`.
    pub fn has_curve_3d(&self, edge: &Edge) -> Result<bool, Failure> {
        Ok(!BRepTool::curve(edge)?.0.is_null())
    }

    /// the 3D curve (a transformed copy when the
    /// representation has a location, with the parameters transformed too) and its
    /// range; `sae.curve_3d(&edge, true)?`.
    pub fn curve_3d(&self, edge: &Edge, orient: bool) -> Result<Curve3dRange, Failure> {
        let found = BRepTool::curve_with_location(edge)?;
        let (curve, cf, cl) =
            same_parameter::moved_curve(&found.curve, &found.location, found.first, found.last)?;
        let (cf, cl) = oriented_range(edge, cf, cl, orient);
        Ok((curve, cf, cl))
    }

    /// the edge has a closed 3D curve and the same vertex at both
    /// ends; `sae.is_closed_3d(&edge)?`.
    pub fn is_closed_3d(&self, edge: &Edge) -> Result<bool, Failure> {
        let (curve, _, _) = BRepTool::curve(edge)?;
        let Some(c3d) = curve.get() else {
            return Ok(false);
        };
        if !c3d.is_closed()? {
            return Ok(false);
        }
        Ok(self.first_vertex(edge).is_same(&self.last_vertex(edge)))
    }

    /// `HasPCurve(edge, face)`; `sae.has_pcurve_on_face(&edge, &face)?`.
    pub fn has_pcurve_on_face(&self, edge: &Edge, face: &Face) -> Result<bool, Failure> {
        let (surface, location) = BRepTool::surface_with_location(face)?;
        self.has_pcurve_on_surface(edge, &surface, &location)
    }

    /// the edge has a p-curve on the surface (on a
    /// plane, the projection of its 3D curve counts); `sae.has_pcurve_on_surface(&e, &s, &l)?`.
    pub fn has_pcurve_on_surface(
        &self,
        edge: &Edge,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Result<bool, Failure> {
        Ok(!BRepTool::curve_on_surface(edge, surface, location)?
            .pcurve
            .is_null())
    }

    /// [`Self::pcurve_on_surface`] on the
    /// surface and location of the face; `sae.pcurve_on_face(&edge, &face, true)?`.
    pub fn pcurve_on_face(
        &self,
        edge: &Edge,
        face: &Face,
        orient: bool,
    ) -> Result<PCurveRange, Failure> {
        let (surface, location) = BRepTool::surface_with_location(face)?;
        self.pcurve_on_surface(edge, &surface, &location, orient)
    }

    /// the p-curve of the edge on
    /// the surface (the second one of a seam for a REVERSED edge) and its range;
    /// `sae.pcurve_on_surface(&edge, &surface, &location, true)?`.
    pub fn pcurve_on_surface(
        &self,
        edge: &Edge,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
        orient: bool,
    ) -> Result<PCurveRange, Failure> {
        let found = BRepTool::curve_on_surface(edge, surface, location)?;
        let (cf, cl) = oriented_range(edge, found.first, found.last, orient);
        Ok((found.pcurve, cf, cl))
    }

    /// `BoundUV(edge, face, first, last)`; `sae.bound_uv_on_face(&edge, &face)?`.
    pub fn bound_uv_on_face(
        &self,
        edge: &Edge,
        face: &Face,
    ) -> Result<Option<(Point2, Point2)>, Failure> {
        let (surface, location) = BRepTool::surface_with_location(face)?;
        self.bound_uv_on_surface(edge, &surface, &location)
    }

    /// the ends of the oriented p-curve;
    /// `None` (false) without p-curve; `sae.bound_uv_on_surface(&e, &s, &l)?`.
    pub fn bound_uv_on_surface(
        &self,
        edge: &Edge,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Result<Option<(Point2, Point2)>, Failure> {
        let (pcurve, uf, ul) = self.pcurve_on_surface(edge, surface, location, true)?;
        let Some(c2d) = pcurve.get() else {
            return Ok(None);
        };
        Ok(Some((c2d.value(uf)?, c2d.value(ul)?)))
    }

    /// Check whether the edge has both seam p-curves on the face; `sae.is_seam_on_face(&e, &f)?`.
    pub fn is_seam_on_face(&self, edge: &Edge, face: &Face) -> Result<bool, Failure> {
        BRepTool::is_closed_on_face(edge, face)
    }

    /// the edge has two p-curves on the surface;
    /// `sae.is_seam_on_surface(&e, &s, &l)?`.
    pub fn is_seam_on_surface(
        &self,
        edge: &Edge,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
    ) -> Result<bool, Failure> {
        BRepTool::is_closed_on_surface(edge, surface, location)
    }

    /// the start vertex in the orientation of the edge (the last
    /// vertex reversed for a REVERSED edge); `sae.first_vertex(&edge)`.
    pub fn first_vertex(&self, edge: &Edge) -> Vertex {
        if edge.orientation() == Orientation::Reversed {
            let mut v = topexp::last_vertex(edge, false);
            v.reverse();
            return v;
        }
        topexp::first_vertex(edge, false)
    }

    /// the end vertex in the orientation of the edge;
    /// `sae.last_vertex(&edge)`.
    pub fn last_vertex(&self, edge: &Edge) -> Vertex {
        if edge.orientation() == Orientation::Reversed {
            let mut v = topexp::first_vertex(edge, false);
            v.reverse();
            return v;
        }
        topexp::last_vertex(edge, false)
    }

    /// `GetEndTangent2d(edge, face, atEnd, pos, tang, dparam)`;
    /// `sae.get_end_tangent_2d_on_face(&edge, &face, false, 0.0)?`.
    pub fn get_end_tangent_2d_on_face(
        &self,
        edge: &Edge,
        face: &Face,
        at_end: bool,
        dparam: f64,
    ) -> Result<(bool, Point2, Vector2), Failure> {
        let (surface, location) = BRepTool::surface_with_location(face)?;
        self.get_end_tangent_2d_on_surface(edge, &surface, &location, at_end, dparam)
    }

    /// the point and
    /// tangent of the oriented p-curve at its start or end. With `dparam` above
    /// `precision::confusion()` the tangent is the chord to the point at that fraction of
    /// the range; otherwise the first non-null derivative up to the third, else the chord
    /// to the other end, reversed for a REVERSED edge. False without p-curve (tangent
    /// zero and point at the origin) or when every tangent is null;
    /// `sae.get_end_tangent_2d_on_surface(&e, &s, &l, true, 0.0)?`.
    pub fn get_end_tangent_2d_on_surface(
        &self,
        edge: &Edge,
        surface: &Handle<dyn GeomSurface>,
        location: &Location,
        at_end: bool,
        dparam: f64,
    ) -> Result<(bool, Point2, Vector2), Failure> {
        let mut outputs = EdgeTangent2d::default();
        let found = self.get_end_tangent_2d_on_surface_into(
            edge,
            surface,
            location,
            at_end,
            dparam,
            &mut outputs,
        )?;
        Ok((found, outputs.point, outputs.tangent))
    }

    /// the flag in the status of the last check; `sae.status_flag(s)`.
    pub fn status_flag(&self, status: ShapeExtendStatus) -> bool {
        ShapeExtend::decode_status(self.status, status)
    }
}

pub use outputs::EdgeTangent2d;
