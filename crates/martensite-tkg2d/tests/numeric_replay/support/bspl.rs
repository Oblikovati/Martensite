//! Bezier and B-spline edition and queries, and evaluation representations; the
//! counterpart of the frozen behavior fixtures
use super::common::{
    Reader, attempt, built, curve, dump, end, flag, int, pnt, real, show, summary, trsf, vec,
};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dBoundedCurve, Geom2dCurve, Geom2dGeometry,
    Geom2dOffsetCurve,
};
use martensite_tkg2d::geom2d_eval::{
    Domain1d, Map1d, RepCurveDesc, RepCurveDescDerivBounded, RepCurveDescFull, RepCurveDescMapped,
};
use martensite_tkmath::gp::{Point2, Transform2, Vector2};

// The deprecated copying getters, with matching and mismatching destinations.
fn bezier_arrays(output: &Output, b: &Geom2dBezierCurve) {
    let n = b.nb_poles();
    for lower in [1, 0] {
        let mut p = Array1::<Point2>::with_bounds(lower, lower + n - 1).unwrap();
        attempt(output, || {
            b.poles_into(&mut p)?;
            p.iter().copied().for_each(|value| pnt(output, value));
            Ok(())
        });
        let mut w = Array1::<f64>::with_bounds(lower, lower + n - 1).unwrap();
        attempt(output, || {
            b.weights_into(&mut w)?;
            w.iter().copied().for_each(|value| real(output, value));
            Ok(())
        });
    }
    let mut p = Array1::<Point2>::with_bounds(1, n + 1).unwrap();
    attempt(output, || b.poles_into(&mut p));
    let mut w = Array1::<f64>::with_bounds(1, n + 1).unwrap();
    attempt(output, || b.weights_into(&mut w));
}

fn bezier_state(output: &Output, b: &Geom2dBezierCurve, u: &[f64]) {
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
    summary(output, b, u);
    dump(output, b, -1);
}

struct Edit {
    i: i32,
    j: i32,
    k: i32,
    p: Point2,
    w: f64,
    a: f64,
    z: f64,
    flag: bool,
    t: Transform2,
}

fn bezier_action(b: &Geom2dBezierCurve, action: i32, r: &mut Reader) -> Result<(), Failure> {
    let i = r.integer();
    let p = r.pnt();
    let (w, a, z) = (r.real(), r.real(), r.real());
    let t = trsf(r);
    match action {
        1 => b.increase(i),
        2 => b.insert_pole_after(i, p, w),
        3 => b.insert_pole_before(i, p, w),
        4 => b.remove_pole(i),
        5 => b.reverse(),
        6 => b.segment(a, z),
        7 => b.set_pole(i, p),
        8 => b.set_pole_weighted(i, p, w),
        9 => b.set_weight(i, w),
        10 => b.transform(t),
        _ => Ok(()),
    }
}

pub fn bezier_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let b: Handle<Geom2dBezierCurve> = c.cast();
    let count = r.integer();
    bezier_state(output, &b, &u);
    for _ in 0..count {
        let action = r.integer();
        attempt(output, || bezier_action(&b, action, &mut r));
        bezier_state(output, &b, &u);
    }
    bezier_state(output, &Geom2dBezierCurve::copy_of(&b).unwrap(), &u);
    int(output, Geom2dBezierCurve::max_degree());
    end(output);
}

fn bspline_header(output: &Output, b: &Geom2dBSplineCurve) {
    int(output, b.degree());
    flag(output, b.is_periodic());
    flag(output, b.is_rational());
    int(output, b.knot_distribution() as i32);
    int(output, b.continuity() as i32);
    int(output, b.nb_knots());
    int(output, b.nb_poles());
    show(output, b.first_u_knot_index(), |value| int(output, value));
    show(output, b.last_u_knot_index(), |value| int(output, value));
    for i in 0..=b.nb_knots() + 1 {
        show(output, b.knot(i), |value| real(output, value));
        show(output, b.multiplicity(i), |value| int(output, value));
    }
    for i in 0..=b.nb_poles() + 1 {
        show(output, b.pole(i), |value| pnt(output, value));
        show(output, b.weight(i), |value| real(output, value));
    }
    b.knot_sequence()
        .iter()
        .copied()
        .for_each(|value| real(output, value));
    b.weights_array(|weights| {
        weights
            .iter()
            .copied()
            .for_each(|value| real(output, value))
    });
    flag(output, b.weights().is_some());
    show(output, b.start_point(), |value| pnt(output, value));
    show(output, b.end_point(), |value| pnt(output, value));
    show(output, b.resolution(1e-7), |value| real(output, value));
    let (first, last) = (b.first_parameter(), b.last_parameter());
    show(output, b.is_g1(first, last, 1e-7), |value| {
        flag(output, value)
    });
    show(output, b.is_g1(-1e9, 1e9, 0.5), |value| flag(output, value));
    bspline_arrays(output, b);
}

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
fn bspline_arrays(output: &Output, b: &Geom2dBSplineCurve) {
    let (nk, np, nf) = (b.nb_knots(), b.nb_poles(), b.knot_sequence().len() as i32);
    for lower in [1, 2, 0] {
        let mut k = Array1::<f64>::with_bounds(lower, nk).unwrap();
        print_into(
            output,
            &mut k,
            |a| b.knots_into(a),
            |value| real(output, value),
        );
        let mut f = Array1::<f64>::with_bounds(lower, nf).unwrap();
        print_into(
            output,
            &mut f,
            |a| b.knot_sequence_into(a),
            |value| real(output, value),
        );
    }
    for extra in [0, 1] {
        let mut m = Array1::<i32>::with_bounds(1, nk + extra).unwrap();
        print_into(
            output,
            &mut m,
            |a| b.multiplicities_into(a),
            |value| int(output, value),
        );
        let mut p = Array1::<Point2>::with_bounds(1, np + extra).unwrap();
        print_into(
            output,
            &mut p,
            |a| b.poles_into(a),
            |value| pnt(output, value),
        );
        let mut w = Array1::<f64>::with_bounds(0, np - 1 + extra).unwrap();
        print_into(
            output,
            &mut w,
            |a| b.weights_into(a),
            |value| real(output, value),
        );
    }
}

fn bspline_local(output: &Output, b: &Geom2dBSplineCurve, v: f64) -> Result<(), Failure> {
    for k1 in b.first_u_knot_index()?..b.last_u_knot_index()? {
        let k2 = k1 + 1;
        show(output, b.local_value(v, k1, k2), |value| pnt(output, value));
        attempt(output, || {
            let (p, d1) = b.local_d1(v, k1, k2)?;
            pnt(output, p);
            vec(output, d1);
            Ok(())
        });
        attempt(output, || {
            let (p, d1, d2) = b.local_d2(v, k1, k2)?;
            pnt(output, p);
            [d1, d2].into_iter().for_each(|value| vec(output, value));
            Ok(())
        });
        attempt(output, || {
            let (p, d1, d2, d3) = b.local_d3(v, k1, k2)?;
            pnt(output, p);
            [d1, d2, d3]
                .into_iter()
                .for_each(|value| vec(output, value));
            Ok(())
        });
        for n in 1..=4 {
            show(output, b.local_dn(v, k1, k2, n), |value| vec(output, value));
        }
    }
    Ok(())
}

fn bspline_state(output: &Output, b: &Geom2dBSplineCurve, u: &[f64]) {
    bspline_header(output, b);
    for &v in u {
        real(output, b.periodic_normalization(v).unwrap());
        for rep in [false, true] {
            for tolerance in [0.0, 1e-9, 0.25] {
                attempt(output, || {
                    let (i1, i2) = b.locate_u(v, tolerance, rep)?;
                    int(output, i1);
                    int(output, i2);
                    Ok(())
                });
            }
        }
        bspline_local(output, b, v).unwrap();
        show(output, b.local_value(v, 1, 1), |value| pnt(output, value));
    }
    summary(output, b, u);
    dump(output, b, -1);
}

fn read_edit(r: &mut Reader) -> Edit {
    let (i, j, k) = (r.integer(), r.integer(), r.integer());
    let p = r.pnt();
    let (w, a, z) = (r.real(), r.real(), r.real());
    let flag = r.flag();
    let t = trsf(r);
    Edit {
        i,
        j,
        k,
        p,
        w,
        a,
        z,
        flag,
        t,
    }
}

fn knot_pair(e: &Edit) -> (Array1<'static, f64>, Array1<'static, i32>) {
    let mut ks = Array1::<f64>::with_bounds(1, 2).unwrap();
    ks[1] = e.a;
    ks[2] = e.z;
    let mut ms = Array1::<i32>::with_bounds(1, 2).unwrap();
    ms[1] = e.i;
    ms[2] = e.j;
    (ks, ms)
}

fn bspline_knot_action(
    output: &Output,
    b: &Geom2dBSplineCurve,
    action: i32,
    e: &Edit,
) -> Result<(), Failure> {
    match action {
        1 => b.increase_degree(e.i),
        2 => b.increase_multiplicity(e.i, e.j),
        3 => b.increase_multiplicity_range(e.i, e.j, e.k),
        4 => b.increment_multiplicity(e.i, e.j, e.k),
        5 => b.insert_knot(e.a, e.i, e.w),
        6 => {
            let (ks, ms) = knot_pair(e);
            b.insert_knots(&ks, &ms, e.w, e.flag)
        }
        7 => b
            .remove_knot(e.i, e.j, e.w)
            .map(|value| flag(output, value)),
        12 => b.segment(e.a, e.z, e.w),
        13 => b.set_knot(e.i, e.a),
        14 => {
            let mut ks = b.knots();
            ks.iter_mut().for_each(|x| *x += e.a);
            b.set_knots(&ks)
        }
        15 => b.set_knot_with_multiplicity(e.i, e.a, e.j),
        _ => bspline_pole_action(output, b, action, e),
    }
}

fn bspline_pole_action(
    output: &Output,
    b: &Geom2dBSplineCurve,
    action: i32,
    e: &Edit,
) -> Result<(), Failure> {
    match action {
        8 => b.insert_pole_after(e.i, e.p, e.w),
        9 => b.insert_pole_before(e.i, e.p, e.w),
        10 => b.remove_pole(e.i),
        11 => b.reverse(),
        16 => b.set_periodic(),
        17 => b.set_origin(e.i),
        18 => b.set_not_periodic(),
        19 => b.set_pole(e.i, e.p),
        20 => b.set_pole_weighted(e.i, e.p, e.w),
        21 => b.set_weight(e.i, e.w),
        22 => {
            let (first, last) = b.move_point(e.a, e.p, e.i, e.j)?;
            int(output, first);
            int(output, last);
            Ok(())
        }
        23 => {
            let tangent = Vector2::new(e.z, e.w);
            b.move_point_and_tangent(e.a, e.p, tangent, 1e-9, e.i, e.j)
                .map(|value| int(output, value))
        }
        24 => b.transform(e.t),
        _ => Ok(()),
    }
}

pub fn bspline_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let b: Handle<Geom2dBSplineCurve> = c.cast();
    let count = r.integer();
    bspline_state(output, &b, &u);
    for _ in 0..count {
        let action = r.integer();
        let edit = read_edit(&mut r);
        attempt(output, || bspline_knot_action(output, &b, action, &edit));
        bspline_state(output, &b, &u);
    }
    bspline_state(output, &Geom2dBSplineCurve::copy_of(&b).unwrap(), &u);
    int(output, Geom2dBSplineCurve::max_degree());
    end(output);
}

/// A descriptor over a representation curve, as g2_desc reads it.
pub fn desc(r: &mut Reader, rep: Handle<dyn Geom2dCurve>) -> Handle<dyn RepCurveDesc> {
    let (kind, order) = (r.integer(), r.integer() as usize);
    let domain = r.flag();
    let (first, last, scale, offset) = (r.real(), r.real(), r.real(), r.real());
    match kind {
        0 => Handle::new(RepCurveDescFull::new(rep)).cast(),
        1 => Handle::new(RepCurveDescDerivBounded::new(rep, order)).cast(),
        _ => {
            let domain = domain.then_some(Domain1d { first, last });
            let map = Map1d { scale, offset };
            Handle::new(RepCurveDescMapped::new(rep, order, domain, map)).cast()
        }
    }
}

/// The Geom2d classes that accept an evaluation representation.
trait EvalRepresented: Geom2dCurve {
    fn set_rep(&self, desc: Handle<dyn RepCurveDesc>) -> Result<(), Failure>;
    fn has_rep(&self) -> bool;
}
macro_rules! eval_represented {
    ($class:ty) => {
        impl EvalRepresented for $class {
            fn set_rep(&self, desc: Handle<dyn RepCurveDesc>) -> Result<(), Failure> {
                self.set_eval_representation(desc)
            }
            fn has_rep(&self) -> bool {
                self.has_eval_representation()
            }
        }
    };
}
eval_represented!(Geom2dBezierCurve);
eval_represented!(Geom2dBSplineCurve);
eval_represented!(Geom2dOffsetCurve);

fn evalrep_on<C: EvalRepresented + martensite_tkernel::standard::RuntimeTyped>(
    output: &Output,
    c: &Handle<C>,
    d: &Handle<dyn RepCurveDesc>,
    u: &[f64],
    t: Transform2,
) {
    attempt(output, || c.set_rep(d.clone()));
    flag(output, c.has_rep());
    summary(output, &**c, u);
    let copy: Handle<C> = c.copy().unwrap().cast();
    flag(output, copy.has_rep());
    summary(output, &*copy, u);
    attempt(output, || {
        c.set_rep(Handle::new(RepCurveDescFull::new(Handle::default())).cast())
    });
    let own = Handle::new(RepCurveDescFull::new(c.cast()));
    attempt(output, || c.set_rep(own.cast()));
    c.reverse().unwrap();
    flag(output, c.has_rep());
    c.transform(t).unwrap();
    flag(output, c.has_rep());
    summary(output, &**c, u);
}

pub fn evalrep_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let rep = curve(&mut r).unwrap();
    let d = desc(&mut r, rep);
    let t = trsf(&mut r);
    if c.cast::<Geom2dBezierCurve>().is_some() {
        evalrep_on(output, &c.cast::<Geom2dBezierCurve>(), &d, &u, t);
    }
    if c.cast::<Geom2dBSplineCurve>().is_some() {
        evalrep_on(output, &c.cast::<Geom2dBSplineCurve>(), &d, &u, t);
    }
    if c.cast::<Geom2dOffsetCurve>().is_some() {
        evalrep_on(output, &c.cast::<Geom2dOffsetCurve>(), &d, &u, t);
    }
    end(output);
}
