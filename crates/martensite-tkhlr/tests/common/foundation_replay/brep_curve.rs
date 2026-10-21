//! Whole projected-curve report shared by direct Rust tests and historical CLI.
use super::brep_curve_fixture::{edge, projector};
use super::brep_curve_print::{circ2d, elips2d, lin2d};
use super::common::{Sink, attempt, end, int, pnt, pnt2d, real, show, xy};
use super::geometry_records::common::Reader;
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkhlr::{
    hlr_algo::init_min_max,
    hlr_brep::{BCurveTool, Curve},
};
use martensite_tkmath::{
    geom_abs::CurveType,
    gp::{Point2, Vector2},
};
use std::sync::Arc;

fn curve_box(tokens: &mut Sink, c: &mut Curve) {
    let (mut mn, mut mx) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut mn, &mut mx);
    attempt(tokens, |tokens| {
        real(tokens, c.update(&mut mn, &mut mx)?);
        mn.iter().chain(mx.iter()).for_each(|v| real(tokens, *v));
        Ok(())
    });
    int(tokens, c.get_type() as i32);
    attempt(tokens, |tokens| {
        real(tokens, c.first_parameter()?);
        real(tokens, c.last_parameter()?);
        Ok(())
    });
}

fn curve_at(tokens: &mut Sink, c: &Curve, t: f64) {
    curve_parameter_pair(tokens, c, t);
    show(tokens, c.z(t), real);
    attempt(tokens, |tokens| {
        let mut p = Point2::new(7., 8.);
        c.d0(t, &mut p)?;
        pnt2d(tokens, p);
        Ok(())
    });
    show(tokens, c.value(t), pnt2d);
    show(tokens, c.value_3d(t), pnt);
    curve_d1(tokens, c, t);
    curve_d2(tokens, c, t);
}

fn curve_parameter_pair(tokens: &mut Sink, c: &Curve, t: f64) {
    attempt(tokens, |tokens| {
        let p2 = c.parameter_2d(t)?;
        real(tokens, p2);
        real(tokens, c.parameter_3d(p2)?);
        Ok(())
    });
}
fn curve_d1(tokens: &mut Sink, c: &Curve, t: f64) {
    attempt(tokens, |tokens| {
        let (mut p, mut v) = (Point2::new(7., 8.), Vector2::new(1., 2.));
        c.d1(t, &mut p, &mut v)?;
        pnt2d(tokens, p);
        xy(tokens, v.xy());
        Ok(())
    });
}
fn curve_d2(tokens: &mut Sink, c: &Curve, t: f64) {
    attempt(tokens, |tokens| {
        let (mut p, mut v, mut w) = (
            Point2::new(7., 8.),
            Vector2::new(1., 2.),
            Vector2::new(3., 4.),
        );
        c.d2(t, &mut p, &mut v, &mut w)?;
        pnt2d(tokens, p);
        xy(tokens, v.xy());
        xy(tokens, w.xy());
        Ok(())
    });
}

fn curve_conics(tokens: &mut Sink, c: &Curve) {
    for start in [true, false] {
        attempt(tokens, |tokens| {
            let (p, d) = c.tangent(start)?;
            pnt2d(tokens, p);
            xy(tokens, d.xy());
            Ok(())
        });
    }
    show(tokens, c.line(), lin2d);
    show(tokens, c.circle(), circ2d);
    show(tokens, c.ellipse(), elips2d);
    real(tokens, c.hyperbola().major_radius());
    real(tokens, c.parabola().focal());
    xy(tokens, c.dn(0.5, 1).xy());
}

fn print_poles(tokens: &mut Sink, poles: &Array1<'_, Point2>, weights: Option<&Array1<'_, f64>>) {
    poles.iter().for_each(|p| pnt2d(tokens, *p));
    weights
        .into_iter()
        .flat_map(|w| w.iter())
        .for_each(|x| real(tokens, *x));
}

fn curve_poles(tokens: &mut Sink, c: &Curve) -> Result<(), Failure> {
    let t = BCurveTool::get_type(c.get_curve())?;
    if t != CurveType::BezierCurve && t != CurveType::BSplineCurve {
        return Ok(());
    }
    let n = c.nb_poles()?;
    let mut poles = Array1::with_bounds(1, n).expect("probe array");
    let mut w = Array1::with_bounds(1, n).expect("probe array");
    general_curve_poles(tokens, c, &mut poles, &mut w);
    if t != CurveType::BSplineCurve {
        return Ok(());
    }
    curve_knots(tokens, c)?;
    specific_spline_poles(tokens, c, &mut poles, &mut w)
}

fn general_curve_poles(
    tokens: &mut Sink,
    c: &Curve,
    poles: &mut Array1<'_, Point2>,
    w: &mut Array1<'_, f64>,
) {
    attempt(tokens, |tokens| {
        c.poles(poles).map(|()| print_poles(tokens, poles, None))
    });
    attempt(tokens, |tokens| {
        c.poles_and_weights(poles, w)
            .map(|()| print_poles(tokens, poles, Some(w)))
    });
}

fn curve_knots(tokens: &mut Sink, c: &Curve) -> Result<(), Failure> {
    let k = c.nb_knots()?;
    let mut kn = Array1::with_bounds(1, k).expect("probe array");
    let mut mu = Array1::with_bounds(1, k).expect("probe array");
    attempt(tokens, |tokens| {
        c.knots(&mut kn)
            .map(|()| kn.iter().for_each(|x| real(tokens, *x)))
    });
    attempt(tokens, |tokens| {
        c.multiplicities(&mut mu)
            .map(|()| mu.iter().for_each(|x| int(tokens, *x)))
    });
    Ok(())
}

fn specific_spline_poles(
    tokens: &mut Sink,
    c: &Curve,
    poles: &mut Array1<'_, Point2>,
    w: &mut Array1<'_, f64>,
) -> Result<(), Failure> {
    let b = BCurveTool::bspline(c.get_curve())?;
    attempt(tokens, |tokens| {
        c.poles_of(&b, poles)
            .map(|()| print_poles(tokens, poles, None))
    });
    attempt(tokens, |tokens| {
        c.poles_and_weights_of(&b, poles, w)
            .map(|()| print_poles(tokens, poles, Some(w)))
    });
    Ok(())
}

/// `projector edge t1 t2 t3`: see `hb_curve_case`.
pub fn curve_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| {
        let projector = projector(&mut r)?;
        let edge = edge(&mut r)?;
        let mut c = Curve::new();
        c.set_projector(Some(Arc::new(projector)));
        c.set_curve(&edge)?;
        curve_box(tokens, &mut c);
        for _ in 0..3 {
            curve_at(tokens, &c, r.real());
        }
        curve_conics(tokens, &c);
        curve_poles(tokens, &c)
    });
    end(tokens);
}
