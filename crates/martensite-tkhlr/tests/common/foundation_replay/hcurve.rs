//! Contap's curve-tool surface and its distinct adaptive sample-count formula.
use super::common::{Sink, attempt, end, flag, int, pnt2d, real, show};
use super::geometry_records::{common::Reader, curve2d::curve2d};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkgeomalgo::int_start::TheArc;
use martensite_tkhlr::contap::ContapHCurve2dTool;
use martensite_tkmath::{geom_abs::Shape, gp::Vector2};
/// Read the bounded or natural arc used by the native tool; `arc(reader)`.
pub fn arc(r: &mut Reader) -> Result<TheArc, Failure> {
    let curve = curve2d(r)?;
    super::arc_fixture::from_fields(&curve, r.flag(), || {
        let first = r.real();
        (first, r.real())
    })
}

fn v2d(tokens: &mut Sink, v: Vector2) {
    real(tokens, v.x());
    real(tokens, v.y());
}

type C = ContapHCurve2dTool;

fn shape(k: i32) -> Shape {
    [Shape::C0, Shape::G1, Shape::C1, Shape::G2][k as usize]
}

fn hc2d_intervals(tokens: &mut Sink, a: &TheArc, k: i32) -> Result<(), Failure> {
    let n = C::nb_intervals(a, shape(k))?;
    int(tokens, n);
    let mut t: Array1<'static, f64> =
        Array1::with_bounds(1, n + 1).map_err(|e| e.into_failure("NCollection_Array1"))?;
    C::intervals(a, &mut t, shape(k))?;
    (1..=n + 1).for_each(|i| real(tokens, t[i]));
    Ok(())
}

fn hc2d_derivatives(tokens: &mut Sink, a: &TheArc, u: f64) {
    show(tokens, C::value(a, u), pnt2d);
    show(tokens, C::d0(a, u), pnt2d);
    show(tokens, C::d1(a, u), |tokens, (p, v)| {
        pnt2d(tokens, p);
        v2d(tokens, v);
    });
    hc2d_higher_derivatives(tokens, a, u);
}

fn hc2d_higher_derivatives(tokens: &mut Sink, a: &TheArc, u: f64) {
    show(tokens, C::d2(a, u), |tokens, (p, v, w)| {
        pnt2d(tokens, p);
        v2d(tokens, v);
        v2d(tokens, w);
    });
    show(tokens, C::d3(a, u), |tokens, (p, v, w, x)| {
        pnt2d(tokens, p);
        v2d(tokens, v);
        v2d(tokens, w);
        v2d(tokens, x);
    });
    (1..=3).for_each(|n| show(tokens, C::dn(a, u, n), v2d));
}

fn hc2d_conics(tokens: &mut Sink, a: &TheArc) {
    show(tokens, C::line(a), |tokens, x| pnt2d(tokens, x.location()));
    show(tokens, C::circle(a), |tokens, x| real(tokens, x.radius()));
    show(tokens, C::ellipse(a), |tokens, x| {
        real(tokens, x.minor_radius())
    });
    show(tokens, C::hyperbola(a), |tokens, x| {
        real(tokens, x.major_radius())
    });
    show(tokens, C::parabola(a), |tokens, x| real(tokens, x.focal()));
    hc2d_polynomial_handles(tokens, a);
}

fn hc2d_polynomial_handles(tokens: &mut Sink, a: &TheArc) {
    show(tokens, C::bezier(a), |tokens, x| {
        flag(tokens, x.is_null());
        if let Some(b) = x.get() {
            int(tokens, b.nb_poles());
        }
    });
    show(tokens, C::bspline(a), |tokens, x| {
        flag(tokens, x.is_null());
        if let Some(b) = x.get() {
            int(tokens, b.nb_knots());
        }
    });
}

/// All original Contap curve-tool calls; `hc2d_case(tokens, numbers)`.
pub fn hc2d_case(tokens: &mut Sink, values: &[f64]) {
    let mut reader = Reader::new(values);
    attempt(tokens, |tokens| hc2d_body(tokens, &mut reader));
    end(tokens);
}

fn hc2d_body(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let arc = arc(reader)?;
    let (u, first, last) = (reader.real(), reader.real(), reader.real());
    hc2d_limits(tokens, &arc)?;
    hc2d_derivatives(tokens, &arc, u);
    show(tokens, C::resolution(&arc, 1.0e-3), real);
    int(tokens, C::get_type(&arc)? as i32);
    hc2d_conics(tokens, &arc);
    show(tokens, C::nb_samples(&arc, first, last), int);
    Ok(())
}

fn hc2d_limits(tokens: &mut Sink, arc: &TheArc) -> Result<(), Failure> {
    real(tokens, C::first_parameter(arc)?);
    real(tokens, C::last_parameter(arc)?);
    int(tokens, C::continuity(arc)? as i32);
    (0..=3).for_each(|k| attempt(tokens, |tokens| hc2d_intervals(tokens, arc, k)));
    show(tokens, C::is_closed(arc), flag);
    show(tokens, C::is_periodic(arc), flag);
    show(tokens, C::period(arc), real);
    Ok(())
}
