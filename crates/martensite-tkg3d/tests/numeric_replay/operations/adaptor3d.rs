//! Curve and surface adaptor evaluation cases.
use super::adaptor3d_inputs::{adaptor2d, curve_summary, inputs, surface};
use super::common::{Reader, attempt, ax1, ax3, dir, end, flag, int, name, pnt, real, show, vec};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::h_surface_tool as tool;
use martensite_tkg3d::adaptor3d::{
    Adaptor3dCurve, Adaptor3dCurveOnSurface, Adaptor3dInterFunc, Adaptor3dIsoCurve,
    Adaptor3dSurface,
};
use martensite_tkmath::geom_abs::{IsoType, Shape};
use martensite_tkmath::math::{Function, FunctionWithDerivative};

const SHAPES: [Shape; 7] = [
    Shape::C0,
    Shape::G1,
    Shape::C1,
    Shape::G2,
    Shape::C2,
    Shape::C3,
    Shape::CN,
];

fn report<T>(output: &Output, built: Result<T, Failure>) -> Option<T> {
    match built {
        Ok(value) => Some(value),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            None
        }
    }
}

// Intervals into an array of a wrong length (a3_interval_arrays).
fn interval_arrays(output: &Output, a: &dyn Adaptor3dCurve) {
    let n = match a.nb_intervals(Shape::C2) {
        Ok(n) => n,
        Err(e) => return replay_print!(output, "{} ", e.exception_type()),
    };
    let mut long: Array1<'static, f64> = Array1::with_bounds(1, n + 2).unwrap();
    attempt(output, || {
        a.intervals(&mut long, Shape::C2)?;
        real(output, *long.value(1).unwrap());
        Ok(())
    });
}

/// `a3_cos`: summary, interval arrays, Trim and ShallowCopy of a curve on a surface.
pub fn cos_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some((c, s)) = inputs(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let (t1, t2) = (r.real(), r.real());
    let Some(a) = report(output, Adaptor3dCurveOnSurface::with_curve(&c, &s)) else {
        return;
    };
    flag(output, a.get_curve() == c);
    flag(output, a.get_surface() == s);
    curve_summary(output, &a, &u);
    interval_arrays(output, &a);
    attempt(output, || {
        curve_summary(output, &*a.trim(t1, t2, 1e-7)?, &u);
        Ok(())
    });
    attempt(output, || {
        curve_summary(output, &*a.shallow_copy()?, &u);
        Ok(())
    });
    end(output);
}

/// `a3_cos_load`: the Load sequences of a curve on a surface.
pub fn cos_load_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some((c1, s1)) = inputs(output, &mut r) else {
        return;
    };
    let Some((c2, s2)) = inputs(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let a = Adaptor3dCurveOnSurface::default();
    flag(output, a.get_curve().is_null());
    flag(output, a.get_surface().is_null());
    show(output, a.get_type(), |k| int(output, k as i32));
    attempt(output, || {
        a.load_surface(&s1)?;
        int(output, a.get_type()? as i32);
        Ok(())
    });
    attempt(output, || {
        a.load_curve(&c1)?;
        curve_summary(output, &a, &u);
        Ok(())
    });
    attempt(output, || {
        a.load_surface(&s2)?;
        curve_summary(output, &a, &u);
        Ok(())
    });
    attempt(output, || {
        a.load(&c2, &s1)?;
        curve_summary(output, &a, &u);
        Ok(())
    });
    show(output, a.nb_intervals(Shape::C1), |value| {
        int(output, value)
    });
    a.change_curve(|curve| *curve = c1.clone());
    a.change_surface(|surface| *surface = s2.clone());
    flag(output, a.get_curve() == c1);
    curve_summary(output, &a, &u);
    attempt(output, || {
        let b = Adaptor3dCurveOnSurface::with_surface(&s2)?;
        int(output, b.get_type()? as i32);
        b.load_curve(&c2)?;
        curve_summary(output, &b, &u);
        Ok(())
    });
    end(output);
}

fn iso_of(value: i32) -> IsoType {
    [IsoType::IsoU, IsoType::IsoV, IsoType::NoneIso][value as usize]
}

fn iso_summary(output: &Output, a: &Adaptor3dIsoCurve, u: &[f64]) {
    int(output, a.iso() as i32);
    real(output, a.parameter());
    flag(output, a.surface().is_null());
    curve_summary(output, a, u);
}

// The empty iso curve: the members that do not dereference the null surface.
fn empty_iso(output: &Output) {
    let e = Adaptor3dIsoCurve::default();
    int(output, e.iso() as i32);
    real(output, e.parameter());
    flag(output, e.surface().is_null());
    real(output, e.first_parameter().unwrap());
    real(output, e.last_parameter().unwrap());
    show(output, e.continuity(), |c| int(output, c as i32));
    show(output, e.nb_intervals(Shape::C1), |value| {
        int(output, value)
    });
    show(output, e.is_closed(), |value| flag(output, value));
    show(output, e.resolution(0.5), |value| real(output, value));
    show(output, e.value(0.), |value| pnt(output, value));
}

/// `a3_iso`: the constructors, Load members, Trim and ShallowCopy of an iso curve.
pub fn iso_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = report(output, surface(output, &mut r)) else {
        return;
    };
    let iso = iso_of(r.integer());
    let (p, w1, w2) = (r.real(), r.real(), r.real());
    let u = r.reals();
    let (t1, t2) = (r.real(), r.real());
    empty_iso(output);
    let b = Adaptor3dIsoCurve::with_surface(&s);
    iso_summary(output, &b, &u);
    attempt(output, || {
        let a = Adaptor3dIsoCurve::with_iso(&s, iso, p)?;
        iso_summary(output, &a, &u);
        attempt(output, || {
            curve_summary(output, &*a.trim(t1, t2, 1e-7)?, &u);
            Ok(())
        });
        attempt(output, || {
            curve_summary(output, &*a.shallow_copy()?, &u);
            Ok(())
        });
        Ok(())
    });
    attempt(output, || {
        iso_summary(
            output,
            &Adaptor3dIsoCurve::with_iso_range(&s, iso, p, (w1, w2))?,
            &u,
        );
        Ok(())
    });
    attempt(output, || {
        b.load_iso_range(iso, p, (w1, w2))?;
        iso_summary(output, &b, &u);
        Ok(())
    });
    attempt(output, || {
        b.load_iso(iso, p)?;
        iso_summary(output, &b, &u);
        Ok(())
    });
    b.load_surface(&s);
    iso_summary(output, &b, &u);
    end(output);
}

/// `a3_interfunc`: Value, Derivative and Values of Adaptor3dInterFunc.
pub fn interfunc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = report(output, adaptor2d(output, &mut r)) else {
        return;
    };
    let (v, fix) = (r.real(), r.integer());
    let x = r.reals();
    match Adaptor3dInterFunc::new(&c, v, fix) {
        Ok(mut f) => {
            for t in x {
                interfunc_at(output, &mut f, t);
            }
        }
        Err(e) => replay_print!(output, "{} ", e.exception_type()),
    }
    end(output);
}

// One parameter: a failed evaluation prints the kept exception, as the retained throw.
fn interfunc_at(output: &Output, f: &mut Adaptor3dInterFunc, t: f64) {
    let (mut a, mut b) = (0., 0.);
    let ok = f.value(t, &mut a);
    print_or_failure(output, f, || {
        flag(output, ok);
        real(output, a);
    });
    let ok = f.derivative(t, &mut b);
    print_or_failure(output, f, || {
        flag(output, ok);
        real(output, b);
    });
    (a, b) = (0., 0.);
    let ok = f.values(t, &mut a, &mut b);
    print_or_failure(output, f, || {
        flag(output, ok);
        real(output, a);
        real(output, b);
    });
}

fn print_or_failure(output: &Output, f: &mut Adaptor3dInterFunc, print: impl FnOnce()) {
    match f.take_failure() {
        Some(failure) => replay_print!(output, "{} ", failure.exception_type()),
        None => print(),
    }
}

// Every continuity: NbU/VIntervals and U/VIntervals.
fn tool_intervals(output: &Output, s: &Handle<dyn Adaptor3dSurface>) {
    for k in SHAPES {
        attempt(output, || {
            let n = tool::nb_u_intervals(s, k)?;
            int(output, n);
            let mut t: Array1<'static, f64> = Array1::with_bounds(1, n + 1).unwrap();
            tool::u_intervals(s, &mut t, k)?;
            t.as_slice().iter().for_each(|v| real(output, *v));
            Ok(())
        });
        attempt(output, || {
            let n = tool::nb_v_intervals(s, k)?;
            int(output, n);
            let mut t: Array1<'static, f64> = Array1::with_bounds(1, n + 1).unwrap();
            tool::v_intervals(s, &mut t, k)?;
            t.as_slice().iter().for_each(|v| real(output, *v));
            Ok(())
        });
    }
}

fn tool_evaluations(output: &Output, s: &Handle<dyn Adaptor3dSurface>, u: f64, v: f64) {
    show(output, tool::value(s, u, v), |value| pnt(output, value));
    show(output, tool::d0(s, u, v), |value| pnt(output, value));
    show(output, tool::d1(s, u, v), |r| {
        pnt(output, r.point);
        vec(output, r.d1u);
        vec(output, r.d1v);
    });
    show(output, tool::d2(s, u, v), |r| {
        pnt(output, r.point);
        [r.d1u, r.d1v, r.d2u, r.d2v, r.d2uv]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, tool::d3(s, u, v), |r| {
        pnt(output, r.point);
        [
            r.d1u, r.d1v, r.d2u, r.d2v, r.d2uv, r.d3u, r.d3v, r.d3uuv, r.d3uvv,
        ]
        .into_iter()
        .for_each(|value| vec(output, value));
    });
    show(output, tool::dn(s, u, v, 1, 1), |value| vec(output, value));
}

fn tool_kinds(output: &Output, s: &Handle<dyn Adaptor3dSurface>) {
    show(output, tool::get_type(s), |k| int(output, k as i32));
    show(output, tool::plane(s), |p| ax3(output, p.position()));
    show(output, tool::cylinder(s), |c| {
        ax3(output, c.position());
        real(output, c.radius());
    });
    show(output, tool::cone(s), |c| {
        ax3(output, c.position());
        real(output, c.ref_radius());
        real(output, c.semi_angle());
    });
    show(output, tool::torus(s), |c| {
        ax3(output, c.position());
        real(output, c.major_radius());
        real(output, c.minor_radius());
    });
    show(output, tool::sphere(s), |c| {
        ax3(output, c.position());
        real(output, c.radius());
    });
    show(output, tool::bezier(s), |b| name(output, &b));
    show(output, tool::bspline(s), |b| name(output, &b));
    show(output, tool::axe_of_revolution(s), |value| {
        ax1(output, value)
    });
    show(output, tool::direction(s), |value| dir(output, value));
    show(output, tool::basis_curve(s), |c| name(output, &c));
    show(output, tool::basis_surface(s), |c| name(output, &c));
    show(output, tool::offset_value(s), |value| real(output, value));
}

/// `a3_hstool`: every Adaptor3dHSurfaceTool function.
pub fn hstool_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = report(output, surface(output, &mut r)) else {
        return;
    };
    let (uv, ranges) = (r.reals(), r.reals());
    show(output, tool::first_u_parameter(&s), |value| {
        real(output, value)
    });
    show(output, tool::last_u_parameter(&s), |value| {
        real(output, value)
    });
    show(output, tool::first_v_parameter(&s), |value| {
        real(output, value)
    });
    show(output, tool::last_v_parameter(&s), |value| {
        real(output, value)
    });
    tool_intervals(output, &s);
    show(
        output,
        tool::u_trim(&s, 0., 0.5, 1e-7).and_then(|t| t.last_u_parameter()),
        |value| real(output, value),
    );
    show(
        output,
        tool::v_trim(&s, 0., 0.5, 1e-7).and_then(|t| t.last_v_parameter()),
        |value| real(output, value),
    );
    show(output, tool::is_u_closed(&s), |value| flag(output, value));
    show(output, tool::is_v_closed(&s), |value| flag(output, value));
    show(output, tool::is_u_periodic(&s), |value| flag(output, value));
    show(output, tool::u_period(&s), |value| real(output, value));
    show(output, tool::is_v_periodic(&s), |value| flag(output, value));
    show(output, tool::v_period(&s), |value| real(output, value));
    for pair in uv.as_chunks::<2>().0 {
        tool_evaluations(output, &s, pair[0], pair[1]);
    }
    for x in [1e-7, 0.5] {
        attempt(output, || {
            real(output, tool::u_resolution(&s, x)?);
            real(output, tool::v_resolution(&s, x)?);
            Ok(())
        });
    }
    tool_kinds(output, &s);
    show(output, tool::nb_samples_u(&s), |value| int(output, value));
    show(output, tool::nb_samples_v(&s), |value| int(output, value));
    for pair in ranges.as_chunks::<2>().0 {
        show(
            output,
            tool::nb_samples_u_range(&s, pair[0], pair[1]),
            |value| int(output, value),
        );
        show(
            output,
            tool::nb_samples_v_range(&s, pair[0], pair[1]),
            |value| int(output, value),
        );
    }
    for t in [precision::angular(), 1e-3, 0.5] {
        show(output, tool::is_surf_g1(&s, true, t), |value| {
            flag(output, value)
        });
        show(output, tool::is_surf_g1(&s, false, t), |value| {
            flag(output, value)
        });
    }
    end(output);
}
