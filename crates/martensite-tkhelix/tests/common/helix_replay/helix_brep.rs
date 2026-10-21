//! HelixBRep probe operations.
use super::common::{Reader, Sink, attempt, end, flag, int, pnt, real};
use super::helix_geom::{bspline, shape};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Edge, Shape, Vertex};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkhelix::helix_brep::HelixBRepBuilderHelix;
use martensite_tkmath::gp::Axis3;

/// `hxb_array`: `lo n (v)...`, an array of n values from the lower bound lo.
fn array<T: Clone + Default>(r: &mut Reader<'_>, convert: fn(f64) -> T) -> Array1<'static, T> {
    let (lo, n) = (r.integer(), r.integer());
    let mut a: Array1<'static, T> = Array1::with_bounds(lo, lo + n - 1).expect("probe bounds");
    for i in lo..lo + n {
        *a.change_value(i).expect("probe index") = convert(r.real());
    }
    a
}

fn reals(r: &mut Reader<'_>) -> Array1<'static, f64> {
    array(r, |v| v)
}

fn flags(r: &mut Reader<'_>) -> Array1<'static, bool> {
    // T(r.real()) for bool: a nonzero double is true.
    array(r, |v| v != 0.0)
}

/// `hxb_vertex`: null flag, point and tolerance.
fn vertex(out: &mut Sink, v: &Vertex) -> Result<(), Failure> {
    flag(out, v.is_null());
    if v.is_null() {
        return Ok(());
    }
    pnt(out, BRepTool::pnt(v)?);
    real(out, BRepTool::vertex_tolerance(v)?);
    Ok(())
}

/// `hxb_edge`: orientation, tolerance, curve range and B-spline, vertices.
fn edge(out: &mut Sink, e: &Edge) -> Result<(), Failure> {
    int(out, e.orientation() as i32);
    real(out, BRepTool::edge_tolerance(e)?);
    let (c, f, l) = BRepTool::curve(e)?;
    real(out, f);
    real(out, l);
    let b: Handle<GeomBSplineCurve> = c.cast();
    bspline(out, &b)?;
    let (v1, v2) = topexp::vertices(e, false);
    vertex(out, &v1)?;
    vertex(out, &v2)
}

/// `hxb_shape`: null flag, type, orientation, edge count and edges.
fn result_shape(out: &mut Sink, s: &Shape) -> Result<(), Failure> {
    flag(out, s.is_null());
    if s.is_null() {
        return Ok(());
    }
    int(out, s.shape_type()? as i32);
    int(out, s.orientation() as i32);
    let mut edges: Vec<Edge> = Vec::new();
    let mut x = Explorer::new(s, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while x.more() {
        edges.push(topods::edge(x.current())?);
        x.next()?;
    }
    int(out, edges.len() as i32);
    edges.iter().try_for_each(|value| edge(out, value))
}

/// `hxb_state`: statuses, reached tolerance and the shape.
fn state(out: &mut Sink, b: &HelixBRepBuilderHelix) -> Result<(), Failure> {
    int(out, b.error_status());
    int(out, b.warning_status());
    real(out, b.tolerance_reached());
    result_shape(out, b.shape())
}

/// `hxb_set`: reads the data of one SetParameters overload, then calls it.
fn set(r: &mut Reader<'_>, b: &mut HelixBRepBuilderHelix, ax: &Axis3) -> Result<(), Failure> {
    match r.integer() {
        0 => {
            let (d, h, p, f) = (reals(r), reals(r), reals(r), flags(r));
            b.set_parameters(ax, &d, &h, &p, &f)
        }
        1 => {
            let (d, h, p, f) = (r.real(), reals(r), reals(r), flags(r));
            b.set_parameters_diam(ax, d, &h, &p, &f)
        }
        2 => {
            let (d1, d2, h, p, f) = (r.real(), r.real(), reals(r), reals(r), flags(r));
            b.set_parameters_spiral(ax, d1, d2, &h, &p, &f)
        }
        3 => {
            let (d, p, t) = (reals(r), reals(r), reals(r));
            b.set_parameters_turns(ax, &d, &p, &t)
        }
        4 => {
            let (d, p, t) = (r.real(), reals(r), reals(r));
            b.set_parameters_diam_turns(ax, d, &p, &t)
        }
        _ => {
            let (d1, d2, p, t) = (r.real(), r.real(), reals(r), reals(r));
            b.set_parameters_spiral_turns(ax, d1, d2, &p, &t)
        }
    }
}

/// `P N X indirect nsteps (step)...`: SetParameters, SetApproxParameters and Perform
/// steps on one HelixBRep_BuilderHelix.
pub fn build_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let (o, n, x, indirect) = (r.pnt(), r.dir(), r.dir(), r.flag());
    // The corpus only spells valid frames; gp_Ax3 would raise for a parallel X.
    let mut ax = Axis3::new(o, n, x).expect("probe frame");
    if indirect {
        ax.y_reverse();
    }
    let mut b = HelixBRepBuilderHelix::new();
    attempt(out, |out| state(out, &b));
    for _ in 0..r.integer() {
        match r.integer() {
            0 => attempt(out, |_| set(&mut r, &mut b, &ax)),
            1 => {
                let (tol, deg, cont) = (r.real(), r.integer(), r.integer());
                b.set_approx_parameters(tol, deg, shape(cont));
            }
            _ => attempt(out, |_| b.perform()),
        }
        attempt(out, |out| state(out, &b));
    }
    end(out);
}
