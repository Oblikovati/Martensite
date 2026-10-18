// Kept in the parent operation namespace to preserve the shared fixtures and callback contracts.
fn bspline_arrays(output: &Output, b: &GeomBSplineCurve) {
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
        let mut p = Array1::<Point3>::with_bounds(1, np + extra).unwrap();
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

fn bspline_header(output: &Output, b: &GeomBSplineCurve) {
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
    b.knots()
        .iter()
        .copied()
        .for_each(|value| real(output, value));
    b.multiplicities()
        .iter()
        .copied()
        .for_each(|value| int(output, value));
    b.poles()
        .iter()
        .copied()
        .for_each(|value| pnt(output, value));
    show(output, b.start_point(), |value| pnt(output, value));
    show(output, b.end_point(), |value| pnt(output, value));
    show(output, b.resolution(1e-7), |value| real(output, value));
    let (first, last) = (b.first_parameter(), b.last_parameter());
    show(output, b.is_g1(first, last, 1e-7), |value| {
        flag(output, value)
    });
    show(output, b.is_g1(-1e9, 1e9, 0.5), |value| flag(output, value));
}

// IsEqual against itself, an exact copy and a copy with the first pole moved by 1e-9.
fn bspline_equality(output: &Output, b: &GeomBSplineCurve) {
    flag(output, b.is_equal(b, 1e-7).unwrap());
    attempt(output, || {
        let c = GeomBSplineCurve::copy_of(b)?;
        flag(output, b.is_equal(&c, 0.0)?);
        c.set_pole(1, c.pole(1)?.translated(Vector3::new(1e-9, 0.0, 0.0)))?;
        flag(output, b.is_equal(&c, 1e-10)?);
        flag(output, b.is_equal(&c, 1e-8)?);
        Ok(())
    });
}

fn bspline_local_spans(output: &Output, b: &GeomBSplineCurve, v: f64) -> Result<(), Failure> {
    for k1 in b.first_u_knot_index()?..b.last_u_knot_index()? {
        let k2 = k1 + 1;
        show(output, b.local_value(v, k1, k2), |value| pnt(output, value));
        show(output, b.local_d0(v, k1, k2), |value| pnt(output, value));
        show(output, b.local_d1(v, k1, k2), |(p, d1)| {
            pnt(output, p);
            vec(output, d1);
        });
        show(output, b.local_d2(v, k1, k2), |(p, d1, d2)| {
            pnt(output, p);
            [d1, d2].into_iter().for_each(|value| vec(output, value));
        });
        show(output, b.local_d3(v, k1, k2), |(p, d1, d2, d3)| {
            pnt(output, p);
            [d1, d2, d3]
                .into_iter()
                .for_each(|value| vec(output, value));
        });
        for n in 1..=4 {
            show(output, b.local_dn(v, k1, k2, n), |value| vec(output, value));
        }
    }
    Ok(())
}

fn bspline_local(output: &Output, b: &GeomBSplineCurve, v: f64) {
    real(output, b.periodic_normalization(v).unwrap());
    for rep in [false, true] {
        for tolerance in [0.0, 1e-9, 0.25] {
            show(output, b.locate_u(v, tolerance, rep), |(i1, i2)| {
                int(output, i1);
                int(output, i2);
            });
        }
    }
    bspline_local_spans(output, b, v).unwrap();
    show(output, b.local_value(v, 1, 1), |value| pnt(output, value));
}

fn bspline_state(output: &Output, b: &GeomBSplineCurve, u: &[f64]) {
    bspline_header(output, b);
    bspline_equality(output, b);
    bspline_arrays(output, b);
    for &v in u {
        bspline_local(output, b, v);
    }
    curve_summary(output, b, u);
    dump(output, b, -1);
}

struct Edit {
    i: i32,
    j: i32,
    k: i32,
    p: Point3,
    w: f64,
    a: f64,
    z: f64,
    flag: bool,
    t: Transform3,
}

fn read_edit(_output: &Output, r: &mut Reader) -> Edit {
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
    b: &GeomBSplineCurve,
    action: i32,
    e: &Edit,
) -> Result<(), Failure> {
    match action {
        1 => b.increase_degree(e.i),
        2 => b.increase_multiplicity(e.i, e.j),
        3 => b.increase_multiplicity_range(e.i, e.j, e.k),
        4 => b.increment_multiplicity(e.i, e.j, e.k),
        5 => b.insert_knot(e.a, e.i, e.w, e.flag),
        6 => {
            let (ks, ms) = knot_pair(e);
            b.insert_knots(&ks, &ms, e.w, e.flag)
        }
        7 => b
            .remove_knot(e.i, e.j, e.w)
            .map(|value| flag(output, value)),
        8 => b.set_origin_parameter(e.a, e.w),
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
    b: &GeomBSplineCurve,
    action: i32,
    e: &Edit,
) -> Result<(), Failure> {
    match action {
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
            let tangent = Vector3::new(e.z, e.w, e.a);
            b.move_point_and_tangent(e.a, e.p, tangent, 1e-9, e.i, e.j)
                .map(|value| int(output, value))
        }
        24 => b.transform(e.t),
        _ => Ok(()),
    }
}

// The rational constructor with CheckRational false and true on the curve's own data.
fn bspline_rebuilt(output: &Output, b: &GeomBSplineCurve, u: &[f64]) {
    for check in [false, true] {
        attempt(output, || {
            let weights: Array1<'static, f64> = b.weights_array(Clone::clone);
            let c = GeomBSplineCurve::with_weights_checked(
                &b.poles(),
                &weights,
                &b.knots(),
                &b.multiplicities(),
                b.degree(),
                b.is_periodic(),
                check,
            )?;
            flag(output, c.is_rational());
            curve_summary(output, &c, u);
            dump(output, &c, -1);
            Ok(())
        });
    }
}

pub fn bspline_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let b: Handle<GeomBSplineCurve> = c.cast();
    let count = r.integer();
    bspline_state(output, &b, &u);
    for _ in 0..count {
        let action = r.integer();
        let edit = read_edit(output, &mut r);
        attempt(output, || bspline_knot_action(output, &b, action, &edit));
        bspline_state(output, &b, &u);
    }
    bspline_state(output, &GeomBSplineCurve::copy_of(&b).unwrap(), &u);
    bspline_rebuilt(output, &b, &u);
    int(output, GeomBSplineCurve::max_degree());
    end(output);
}

pub fn desc(
    _output: &Output,
    r: &mut Reader,
    rep: Handle<dyn GeomCurve>,
) -> Handle<dyn RepCurveDesc> {
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

/// The Geom classes of this probe that accept an evaluation representation.
trait EvalRepresented: GeomCurve {
    fn set_rep(&self, desc: Handle<dyn RepCurveDesc>) -> Result<(), Failure>;
    fn has_rep(&self) -> bool;
    fn clear_rep(&self);
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
            fn clear_rep(&self) {
                self.clear_eval_representation()
            }
        }
    };
}
eval_represented!(GeomBezierCurve);
eval_represented!(GeomBSplineCurve);
eval_represented!(GeomOffsetCurve);

fn evalrep_on<C: EvalRepresented + RuntimeTyped>(
    output: &Output,
    c: &Handle<C>,
    d: &Handle<dyn RepCurveDesc>,
    u: &[f64],
    t: Transform3,
) {
    attempt(output, || c.set_rep(d.clone()));
    flag(output, c.has_rep());
    curve_summary(output, &**c, u);
    let copy: Handle<C> = c.copy().unwrap().cast();
    flag(output, copy.has_rep());
    curve_summary(output, &*copy, u);
    attempt(output, || {
        c.set_rep(Handle::new(RepCurveDescFull::new(Handle::default())).cast())
    });
    let own = Handle::new(RepCurveDescFull::new(c.cast()));
    attempt(output, || c.set_rep(own.cast()));
    c.reverse().unwrap();
    flag(output, c.has_rep());
    curve_summary(output, &**c, u);
    c.transform(t).unwrap();
    flag(output, c.has_rep());
    curve_summary(output, &**c, u);
    c.clear_rep();
    flag(output, c.has_rep());
}

pub fn evalrep_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let rep = shapes::curve(output, &mut r).unwrap();
    let d = desc(output, &mut r, rep);
    let t = trsf(&mut r);
    if c.cast::<GeomBezierCurve>().is_some() {
        evalrep_on(output, &c.cast::<GeomBezierCurve>(), &d, &u, t);
    }
    if c.cast::<GeomBSplineCurve>().is_some() {
        evalrep_on(output, &c.cast::<GeomBSplineCurve>(), &d, &u, t);
    }
    if c.cast::<GeomOffsetCurve>().is_some() {
        evalrep_on(output, &c.cast::<GeomOffsetCurve>(), &d, &u, t);
    }
    end(output);
}

pub fn evalrep_edit_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let rep = shapes::curve(output, &mut r).unwrap();
    let d = desc(output, &mut r, rep);
    let b: Handle<GeomBSplineCurve> = c.cast();
    let count = r.integer();
    for _ in 0..count {
        let action = r.integer();
        attempt(output, || b.set_eval_representation(d.clone()));
        let edit = read_edit(output, &mut r);
        attempt(output, || bspline_knot_action(output, &b, action, &edit));
        flag(output, b.has_eval_representation());
        curve_summary(output, &*b, &u);
    }
    end(output);
}
