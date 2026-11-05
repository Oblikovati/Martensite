//! `BRepFillTrimSurfaceTool`: the trimming of two neighbour faces of an evolved shape
//! by a bisector, and the intersection line of the faces along it.
mod eval;

use super::{BRepFillApproxSeewing, BRepFillMultiLine, MultiLineCurves};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::ncollection::Sequence;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomGeometry, GeomPlane, GeomSurface, GeomTrimmedCurve};
use martensite_tkgeomalgo::geom2d_api::ProjectPointOnCurve;
use martensite_tkgeombase::geom_proj_lib;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Direction3, Line3, Point2, Point3};
use martensite_tktopalgo::brep_int_curve_surface::BRepIntCurveSurfaceInter;

/// The trimming of two faces by a bisector (`BRepFillTrimSurfaceTool`); Edge1 and
/// Edge2 are the parallel edges of the minimal isos of the faces, Inv1 and Inv2 whether
/// they are reversed parallel;
/// `let t = BRepFillTrimSurfaceTool::new(&bis, &f1, &f2, &e1, &e2, false, false);`.
pub struct BRepFillTrimSurfaceTool {
    my_face1: Face,
    my_face2: Face,
    my_edge1: Edge,
    my_edge2: Edge,
    my_inv1: bool,
    my_inv2: bool,
    my_bis: Handle<dyn Geom2dCurve>,
}

/// The curves of a projection with their continuity.
pub type ProjectedCurves = (MultiLineCurves, Continuity);

impl BRepFillTrimSurfaceTool {
    /// `BRepFillTrimSurfaceTool(Bis, Face1, Face2, Edge1, Edge2, Inv1, Inv2)`.
    #[allow(clippy::too_many_arguments)] // The native seven-argument constructor.
    pub fn new(
        bis: &Handle<dyn Geom2dCurve>,
        face1: &Face,
        face2: &Face,
        edge1: &Edge,
        edge2: &Edge,
        inv1: bool,
        inv2: bool,
    ) -> Self {
        Self {
            my_face1: face1.clone(),
            my_face2: face2.clone(),
            my_edge1: edge1.clone(),
            my_edge2: edge2.clone(),
            my_inv1: inv1,
            my_inv2: inv2,
            my_bis: bis.clone(),
        }
    }

    /// `IntersectWith(EdgeOnF1, EdgeOnF2, Points)`: the intersections of the bisector
    /// with both edges as (parameter on the bisector, on the first, on the second);
    /// StdFailNotDone when their counts differ; `t.intersect_with(&e1, &e2, &mut pts)?`.
    pub fn intersect_with(
        &self,
        edge_on_f1: &Edge,
        edge_on_f2: &Edge,
        points: &mut Sequence<Point3>,
    ) -> Result<(), Failure> {
        points.clear(None);
        let p1 = eval::eval_parameters(edge_on_f1, &self.my_face1, &self.my_bis)?;
        let p2 = eval::eval_parameters(edge_on_f2, &self.my_face2, &self.my_bis)?;
        if p1.len() != p2.len() {
            return Err(Failure::with_type(
                "StdFail_NotDone",
                &format!(
                    "BRepFillTrimSurfaceTool::IntersectWith: incoherent intersection; {} and {} points",
                    p1.len(),
                    p2.len()
                ),
                "BRepFillTrimSurfaceTool",
            ));
        }
        for (a, b) in p1.iter().zip(&p2) {
            points.append(Point3::new(a.x(), a.y(), b.y()));
        }
        Ok(())
    }

    /// `IsOnFace(Point)`: whether the vertical line through the point meets the first
    /// face, else the second (within 1e-6); `t.is_on_face(p)?`.
    pub fn is_on_face(&self, point: Point2) -> Result<bool, Failure> {
        let line = Line3::new(Point3::new(point.x(), point.y(), 0.), Direction3::Z);
        let mut inter = BRepIntCurveSurfaceInter::new();
        inter.init_line(self.my_face1.as_ref(), line, 1e-6)?;
        if inter.more() {
            return Ok(true);
        }
        inter.init_line(self.my_face2.as_ref(), line, 1e-6)?;
        Ok(inter.more())
    }

    /// `ProjOn(Point, Edge)`: the parameter of the projection of the point on the edge
    /// projected in the XY plane; `t.proj_on(p, &edge)?`.
    pub fn proj_on(&self, point: Point2, edge: &Edge) -> Result<f64, Failure> {
        let found = BRepTool::curve_with_location(edge)?;
        let ct = Handle::new(GeomTrimmedCurve::new(
            &found.curve,
            found.first,
            found.last,
            true,
            true,
        )?);
        let curve = ct.get().ok_or_else(null_curve)?;
        curve.transform(found.location.transformation())?;
        let plane: Handle<dyn GeomSurface> =
            Handle::new(GeomPlane::from_coefficients(0., 0., 1., 0.)?).cast();
        let c2d = geom_proj_lib::curve2d(&ct.cast(), &plane)?;
        ProjectPointOnCurve::with_curve(point, &c2d)?.lower_distance_parameter()
    }

    /// `Project(U1, U2, Curve, PCurve1, PCurve2, Cont)`: the intersection line of the
    /// faces along the bisector between the parameters, exact in the particular cases,
    /// else approximated; `let ((c, pc1, pc2), cont) = t.project(0., 1.)?;`.
    pub fn project(&self, u1: f64, u2: f64) -> Result<ProjectedCurves, Failure> {
        let ct: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dTrimmedCurve::new(&self.my_bis, u1, u2, true, true)?).cast();
        let ml = BRepFillMultiLine::new(
            &self.my_face1,
            &self.my_face2,
            &self.my_edge1,
            &self.my_edge2,
            self.my_inv1,
            self.my_inv2,
            &ct,
        )?;
        let cont = ml.continuity();
        if ml.is_particular_case() {
            return Ok((ml.curves()?, cont));
        }
        let app_seew = BRepFillApproxSeewing::with_multi_line(&ml)?;
        let curves = (
            app_seew.curve()?.clone(),
            app_seew.curve_on_f1()?.clone(),
            app_seew.curve_on_f2()?.clone(),
        );
        Ok((curves, cont))
    }
}

/// StandardNullObject of an edge without 3d curve (natively a crash).
fn null_curve() -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        "BRepFillTrimSurfaceTool: edge without 3d curve; expected a curve",
        "BRepFillTrimSurfaceTool",
    )
}
