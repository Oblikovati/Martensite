//! The constraints of a filling: the curve constraints of the plate (on a face or on a
//! 3d curve) and the edge/face/order records of the boundary and the free constraints.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dCurveOnSurface};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkgeomalgo::geom_plate::GeomPlateCurveConstraint;
use martensite_tkmath::geom_abs::Shape as Continuity;
use std::ops::Deref;

/// A filling constraint preserving its derived runtime type through base handles.
/// `let c = BRepFillCurveConstraint::with_curve(&curve, 0, 10, 1e-4)?;`.
pub struct BRepFillCurveConstraint {
    parent: GeomPlateCurveConstraint,
}
impl_transient!(
    BRepFillCurveConstraint,
    "BRepFillCurveConstraint",
    GeomPlateCurveConstraint,
    parent
);

impl Deref for BRepFillCurveConstraint {
    type Target = GeomPlateCurveConstraint;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl BRepFillCurveConstraint {
    /// `BRepFillCurveConstraint(Boundary, Order, NPt, TolDist, TolAng, TolCurv)`
    /// (natively defaulted to 10, 1e-4, 0.01 and 0.1): the constraint of order -1..=2 on
    /// the curve on a surface (a geometric surface, or the located surface of a face);
    /// StandardFailure for another order or a null boundary.
    pub fn on_surface(
        boundary: &Handle<Adaptor3dCurveOnSurface>,
        order: i32,
        npt: i32,
        tols: [f64; 3],
    ) -> Result<Handle<GeomPlateCurveConstraint>, Failure> {
        let surf = checked_constraint_surface(boundary, order)?;
        Ok(Handle::new(Self {
            parent: GeomPlateCurveConstraint::from_derived_state(
                boundary, order, npt, tols, &surf,
            )?,
        })
        .cast())
    }

    /// `BRepFillCurveConstraint(Boundary, Tang, NPt, TolDist)` (natively defaulted to
    /// 10 points and 1e-4): the plate constraint on a 3d curve (orders G0 and -1), with
    /// the default angular and curvature tolerances.
    pub fn with_curve(
        boundary: &Handle<dyn Adaptor3dCurve>,
        tang: i32,
        npt: i32,
        tol_dist: f64,
    ) -> Result<Handle<GeomPlateCurveConstraint>, Failure> {
        Ok(Handle::new(Self {
            parent: GeomPlateCurveConstraint::new(boundary, tang, npt, tol_dist, 0.01, 0.1)?,
        })
        .cast())
    }
}

fn checked_constraint_surface(
    boundary: &Handle<Adaptor3dCurveOnSurface>,
    order: i32,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    if !(-1..=2).contains(&order) {
        return Err(Failure::new(&format!(
            "BRepFill : The continuity is not G0 G1 or G2; got order {order}"
        )));
    }
    let frontiere = boundary
        .get()
        .ok_or_else(|| Failure::new("BRepFillCurveConstraint : Curve must be on a Surface"))?;
    surface_of(frontiere)
}

/// The geometric surface of a curve on surface: that of a GeomAdaptorSurface, else
/// the located surface of the face of a BRepAdaptorSurface.
fn surface_of(frontiere: &Adaptor3dCurveOnSurface) -> Result<Handle<dyn GeomSurface>, Failure> {
    let gs1: Handle<GeomAdaptorSurface> = frontiere.get_surface().cast();
    if let Some(gs1) = gs1.get() {
        return Ok(gs1.surface());
    }
    let bs1: Handle<BRepAdaptorSurface> = frontiere.get_surface().cast();
    let bs1 = bs1.get().ok_or_else(|| {
        Failure::with_type("Standard_NullObject", "BRepFillCurveConstraint: surface adaptor neither GeomAdaptor_Surface nor BRepAdaptor_Surface; expected one of them", "BRepFillCurveConstraint")
    })?;
    BRepTool::surface(&bs1.face())
}

/// An edge, its support face (null for none) and the continuity order of a filling
/// constraint (`BRepFillEdgeFaceAndOrder`);
/// `BRepFillEdgeFaceAndOrder::with_edge(&edge, &Face::default(), Continuity::C0)`.
#[derive(Clone, Debug)]
pub struct BRepFillEdgeFaceAndOrder {
    pub(crate) my_edge: Edge,
    pub(crate) my_face: Face,
    pub(crate) my_order: Continuity,
}

impl BRepFillEdgeFaceAndOrder {
    /// `BRepFillEdgeFaceAndOrder()`: null shapes and C0 (the order is uninitialized
    /// natively); `BRepFillEdgeFaceAndOrder::new()`.
    pub fn new() -> Self {
        Self::with_edge(&Edge::default(), &Face::default(), Continuity::C0)
    }

    /// `BRepFillEdgeFaceAndOrder(anEdge, aFace, anOrder)`;
    /// `BRepFillEdgeFaceAndOrder::with_edge(&edge, &face, Continuity::G1)`.
    pub fn with_edge(an_edge: &Edge, a_face: &Face, an_order: Continuity) -> Self {
        Self {
            my_edge: an_edge.clone(),
            my_face: a_face.clone(),
            my_order: an_order,
        }
    }
}

/// A support face and the continuity order of a free filling constraint
/// (`BRepFillFaceAndOrder`); `BRepFillFaceAndOrder::with_face(&face, Continuity::G1)`.
#[derive(Clone, Debug)]
pub struct BRepFillFaceAndOrder {
    pub(crate) my_face: Face,
    pub(crate) my_order: Continuity,
}

impl BRepFillFaceAndOrder {
    /// `BRepFillFaceAndOrder()`: a null face and C0 (the order is uninitialized
    /// natively); `BRepFillFaceAndOrder::new()`.
    pub fn new() -> Self {
        Self::with_face(&Face::default(), Continuity::C0)
    }

    /// `BRepFillFaceAndOrder(aFace, anOrder)`;
    /// `BRepFillFaceAndOrder::with_face(&face, Continuity::G1)`.
    pub fn with_face(a_face: &Face, an_order: Continuity) -> Self {
        Self {
            my_face: a_face.clone(),
            my_order: an_order,
        }
    }
}

impl Default for BRepFillEdgeFaceAndOrder {
    /// The empty record; `BRepFillEdgeFaceAndOrder::default()`.
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BRepFillFaceAndOrder {
    /// The empty record; `BRepFillFaceAndOrder::default()`.
    fn default() -> Self {
        Self::new()
    }
}
