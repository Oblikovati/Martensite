//! B-spline curve construction, mutation and continuity.
use super::common::{
    Reader, attempt, built_curve, curve_summary, dump, end, flag, int, pnt, real, show, trsf, vec,
};
use super::output::Output;
use super::shapes;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomBoundedCurve, GeomCurve, GeomGeometry, GeomOffsetCurve,
};
use martensite_tkg3d::geom_eval::{
    Domain1d, Map1d, RepCurveDesc, RepCurveDescDerivBounded, RepCurveDescFull, RepCurveDescMapped,
};
use martensite_tkmath::gp::{Point3, Transform3, Vector3};

fn print_into<T: Copy + Default>(
    output: &Output,
    array: &mut Array1<'_, T>,
    fill: impl FnOnce(&mut Array1<'_, T>) -> Result<(), Failure>,
    print: impl Fn(T),
) {
    attempt(output, || {
        fill(array)?;
        array.iter().copied().for_each(print);
        Ok(())
    });
}

// The deprecated copying getters, with matching and mismatching destinations.
fn bezier_arrays(output: &Output, b: &GeomBezierCurve) {
    let n = b.nb_poles();
    for lower in [1, 0] {
        let mut p = Array1::<Point3>::with_bounds(lower, lower + n - 1).unwrap();
        print_into(
            output,
            &mut p,
            |a| b.poles_into(a),
            |value| pnt(output, value),
        );
        let mut w = Array1::<f64>::with_bounds(lower, lower + n - 1).unwrap();
        print_into(
            output,
            &mut w,
            |a| b.weights_into(a),
            |value| real(output, value),
        );
    }
    let mut p = Array1::<Point3>::with_bounds(1, n + 1).unwrap();
    attempt(output, || b.poles_into(&mut p));
    let mut w = Array1::<f64>::with_bounds(1, n + 1).unwrap();
    attempt(output, || b.weights_into(&mut w));
}

fn bezier_state(output: &Output, b: &GeomBezierCurve, u: &[f64]) {
    int(output, b.nb_poles());
    int(output, b.degree());
    flag(output, b.is_rational());
    show(output, b.is_closed(), |value| flag(output, value));
    for i in 0..=b.nb_poles() + 1 {
        show(output, b.pole(i), |value| pnt(output, value));
        show(output, b.weight(i), |value| real(output, value));
    }
    b.weights_array(|weights| {
        weights
            .iter()
            .copied()
            .for_each(|value| real(output, value))
    });
    flag(output, b.weights().is_some());
    b.poles()
        .iter()
        .copied()
        .for_each(|value| pnt(output, value));
    b.knots()
        .iter()
        .copied()
        .for_each(|value| real(output, value));
    b.multiplicities()
        .unwrap()
        .iter()
        .copied()
        .for_each(|value| int(output, value));
    b.knot_sequence()
        .unwrap()
        .iter()
        .copied()
        .for_each(|value| real(output, value));
    pnt(output, b.start_point().unwrap());
    pnt(output, b.end_point().unwrap());
    real(output, b.resolution(1e-7).unwrap());
    real(output, b.resolution(2.5).unwrap());
    bezier_arrays(output, b);
    curve_summary(output, b, u);
    dump(output, b, -1);
}

fn bezier_action(b: &GeomBezierCurve, action: i32, r: &mut Reader) -> Result<(), Failure> {
    let i = r.integer();
    let p = r.pnt();
    let (w, a, z) = (r.real(), r.real(), r.real());
    let t = trsf(r);
    match action {
        1 => b.increase(i),
        2 => b.insert_pole_after_weighted(i, p, w),
        3 => b.insert_pole_before_weighted(i, p, w),
        4 => b.remove_pole(i),
        5 => b.reverse(),
        6 => b.segment(a, z),
        7 => b.set_pole(i, p),
        8 => b.set_pole_weighted(i, p, w),
        9 => b.set_weight(i, w),
        10 => b.transform(t),
        11 => b.insert_pole_after(i, p),
        12 => b.insert_pole_before(i, p),
        _ => Ok(()),
    }
}

pub fn bezier_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let b: Handle<GeomBezierCurve> = c.cast();
    let count = r.integer();
    bezier_state(output, &b, &u);
    for _ in 0..count {
        let action = r.integer();
        attempt(output, || bezier_action(&b, action, &mut r));
        bezier_state(output, &b, &u);
    }
    bezier_state(output, &GeomBezierCurve::copy_of(&b).unwrap(), &u);
    int(output, GeomBezierCurve::max_degree());
    end(output);
}

// The deprecated copying getters, with matching and mismatching destinations.
include!("bspl/spline_and_representation.rs");
