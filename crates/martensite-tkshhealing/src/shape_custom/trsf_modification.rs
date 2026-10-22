// Rust translation of ModelingAlgorithms/TKShHealing/ShapeCustom/ShapeCustom_TrsfModification.hxx/.cxx at
// 3d097a0328e71b826377d4814ab05ec3c3d23871.

//! `ShapeCustom_TrsfModification`: the BRepTools transformation with the stored
//! tolerances scaled (docs/shhealing-design.md section 7).
use martensite_tkbrep::brep::access::{edge_tshape, face_tshape, vertex_tshape};
use martensite_tkbrep::brep_tools::{
    BRepToolsModification, NewCurve, NewSurface, TrsfModification,
};
use martensite_tkbrep::topods::{Edge, Face, Vertex};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkmath::poly::{Polygon3D, PolygonOnTriangulation, Triangulation};

/// `ShapeCustom_TrsfModification`: `BRepTools_TrsfModification` whose output tolerance
/// is the tolerance stored in the TShape (not raised to `Precision::Confusion()` as
/// `BRep_Tool::Tolerance` does) times `|ScaleFactor|`, after the inherited member ran.
///
/// ```
/// use martensite_tkmath::gp::Transform3;
/// use martensite_tkshhealing::shape_custom::ShapeCustomTrsfModification;
/// let modification = ShapeCustomTrsfModification::new(Transform3::default());
/// assert_eq!(modification.trsf_modification().trsf().scale_factor(), 1.0);
/// ```
pub struct ShapeCustomTrsfModification {
    parent: TrsfModification,
}
impl_transient!(
    ShapeCustomTrsfModification,
    "ShapeCustom_TrsfModification",
    TrsfModification,
    parent,
    interfaces: [dyn BRepToolsModification]
);

impl ShapeCustomTrsfModification {
    /// `ShapeCustom_TrsfModification(T)`; `ShapeCustomTrsfModification::new(trsf)`.
    pub fn new(trsf: Transform3) -> Self {
        Self {
            parent: TrsfModification::new(trsf),
        }
    }

    /// The inherited `BRepTools_TrsfModification` part (`Trsf()`, `IsCopyMesh()`);
    /// `modification.trsf_modification().trsf()`.
    pub fn trsf_modification(&self) -> &TrsfModification {
        &self.parent
    }

    /// `|Trsf().ScaleFactor()|`.
    fn abs_scale(&self) -> f64 {
        self.parent.trsf().scale_factor().abs()
    }
}

impl BRepToolsModification for ShapeCustomTrsfModification {
    /// The inherited result, with the TFace tolerance times `|ScaleFactor|`.
    fn new_surface(&self, face: &Face, result: &mut NewSurface) -> Result<bool, Failure> {
        let done = self.parent.new_surface(face, result)?;
        result.tolerance = face_tshape(face, "ShapeCustom_TrsfModification::NewSurface")?
            .tolerance()
            * self.abs_scale();
        Ok(done)
    }

    fn new_triangulation(
        &self,
        face: &Face,
        triangulation: &mut Handle<Triangulation>,
    ) -> Result<bool, Failure> {
        self.parent.new_triangulation(face, triangulation)
    }

    /// The inherited result, with the TEdge tolerance times `|ScaleFactor|`.
    fn new_curve(&self, edge: &Edge, result: &mut NewCurve) -> Result<bool, Failure> {
        let done = self.parent.new_curve(edge, result)?;
        result.tolerance = edge_tolerance(edge, "NewCurve")? * self.abs_scale();
        Ok(done)
    }

    fn new_polygon(&self, edge: &Edge, polygon: &mut Handle<Polygon3D>) -> Result<bool, Failure> {
        self.parent.new_polygon(edge, polygon)
    }

    fn new_polygon_on_triangulation(
        &self,
        edge: &Edge,
        face: &Face,
        polygon: &mut Handle<PolygonOnTriangulation>,
    ) -> Result<bool, Failure> {
        self.parent
            .new_polygon_on_triangulation(edge, face, polygon)
    }

    /// The inherited result, with the TVertex tolerance times `|ScaleFactor|`.
    fn new_point(
        &self,
        vertex: &Vertex,
        point: &mut Point3,
        tolerance: &mut f64,
    ) -> Result<bool, Failure> {
        let done = self.parent.new_point(vertex, point, tolerance)?;
        *tolerance = vertex_tolerance(vertex, "NewPoint")? * self.abs_scale();
        Ok(done)
    }

    /// The inherited result, with the TEdge tolerance times `|ScaleFactor|`.
    fn new_curve2d(
        &self,
        edge: &Edge,
        face: &Face,
        new_edge: &Edge,
        new_face: &Face,
        curve: &mut Handle<dyn Geom2dCurve>,
        tolerance: &mut f64,
    ) -> Result<bool, Failure> {
        let done = self
            .parent
            .new_curve2d(edge, face, new_edge, new_face, curve, tolerance)?;
        *tolerance = edge_tolerance(edge, "NewCurve2d")? * self.abs_scale();
        Ok(done)
    }

    /// The inherited result, with the TVertex tolerance times `|ScaleFactor|`; a null
    /// vertex (an infinite edge), which the native member dereferences after the
    /// inherited call, is Standard_NullObject.
    fn new_parameter(
        &self,
        vertex: &Vertex,
        edge: &Edge,
        parameter: &mut f64,
        tolerance: &mut f64,
    ) -> Result<bool, Failure> {
        let done = self
            .parent
            .new_parameter(vertex, edge, parameter, tolerance)?;
        *tolerance = vertex_tolerance(vertex, "NewParameter")? * self.abs_scale();
        Ok(done)
    }

    fn continuity(
        &self,
        edge: &Edge,
        face1: &Face,
        face2: &Face,
        new_edge: &Edge,
        new_face1: &Face,
        new_face2: &Face,
    ) -> Result<Continuity, Failure> {
        self.parent
            .continuity(edge, face1, face2, new_edge, new_face1, new_face2)
    }
}

/// `BRep_TEdge::Tolerance()` of the edge's TShape.
fn edge_tolerance(edge: &Edge, member: &str) -> Result<f64, Failure> {
    let context = format!("ShapeCustom_TrsfModification::{member}");
    Ok(edge_tshape(edge, &context)?.tolerance())
}

/// `BRep_TVertex::Tolerance()` of the vertex's TShape.
fn vertex_tolerance(vertex: &Vertex, member: &str) -> Result<f64, Failure> {
    let context = format!("ShapeCustom_TrsfModification::{member}");
    Ok(vertex_tshape(vertex, &context)?.tolerance())
}
