//! One constructor body preserves field/error order for owned and legacy readers.
use super::geometry_records::common::Reader;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomLine,
};
use martensite_tkhlr::hlr_algo::Projector;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

/// The exact fields needed by this fixture, independent of legacy Reader identity.
pub(crate) trait CurveFixtureInput {
    fn integer(&mut self) -> i32;
    fn real(&mut self) -> f64;
    fn pnt(&mut self) -> Point3;
    fn dir(&mut self) -> Direction3;
}
impl CurveFixtureInput for Reader<'_> {
    fn integer(&mut self) -> i32 {
        Reader::integer(self)
    }
    fn real(&mut self) -> f64 {
        Reader::real(self)
    }
    fn pnt(&mut self) -> Point3 {
        Reader::pnt(self)
    }
    fn dir(&mut self) -> Direction3 {
        Reader::dir(self)
    }
}

/// `pk [loc dir xdir [focus]]`: see `hb_projector`.
pub fn projector(r: &mut impl CurveFixtureInput) -> Result<Projector, Failure> {
    let kind = r.integer();
    if kind == 2 {
        return Projector::new();
    }
    let (loc, dir, xdir) = (r.pnt(), r.dir(), r.dir());
    let frame = Axis2::with_x_direction(loc, dir, xdir)?;
    if kind == 0 {
        return Projector::with_frame(frame);
    }
    Projector::with_frame_focus(frame, r.real())
}

fn poles(r: &mut impl CurveFixtureInput) -> Array1<'static, Point3> {
    let n = r.integer();
    let mut poles = Array1::with_bounds(1, n).expect("probe poles");
    for i in 1..=n {
        poles[i] = r.pnt();
    }
    poles
}

fn bspline(r: &mut impl CurveFixtureInput) -> Result<Handle<dyn GeomCurve>, Failure> {
    let poles = poles(r);
    let nk = poles.len() as i32 - 1;
    let mut knots = Array1::with_bounds(1, nk).expect("probe knots");
    let mut mults = Array1::with_bounds(1, nk).expect("probe knots");
    for i in 1..=nk {
        knots[i] = f64::from(i - 1);
        mults[i] = if i == 1 || i == nk { 3 } else { 1 };
    }
    Ok(Handle::new(GeomBSplineCurve::new(&poles, &knots, &mults, 2, false)?).cast())
}

/// `ck ...`: see `hb_geometry`.
pub fn geometry(r: &mut impl CurveFixtureInput) -> Result<Handle<dyn GeomCurve>, Failure> {
    let kind = r.integer();
    match kind {
        0 => {
            let p = r.pnt();
            return Ok(Handle::new(GeomLine::from_point(p, r.dir())).cast());
        }
        3 => return Ok(Handle::new(GeomBezierCurve::new(&poles(r))?).cast()),
        4 => return bspline(r),
        _ => {}
    }
    conic_geometry(r, kind)
}

fn conic_geometry(
    r: &mut impl CurveFixtureInput,
    kind: i32,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    let (p, n, x) = (r.pnt(), r.dir(), r.dir());
    let frame = Axis2::with_x_direction(p, n, x)?;
    if kind == 1 {
        return Ok(Handle::new(GeomCircle::new(frame, r.real())?).cast());
    }
    let major = r.real();
    Ok(Handle::new(GeomEllipse::new(frame, major, r.real())?).cast())
}

/// `geometry u1 u2`: see `hb_edge`.
pub fn edge(r: &mut impl CurveFixtureInput) -> Result<Edge, Failure> {
    let curve = geometry(r)?;
    let (u1, u2) = (r.real(), r.real());
    let mut edge = Edge::default();
    let builder = BRepBuilder::default();
    builder.make_edge_with_curve(&mut edge, &curve, 1e-7)?;
    builder.range(&edge, u1, u2, false)?;
    Ok(edge)
}
