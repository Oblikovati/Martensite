//! Curve and surface adaptor summaries.
use super::common::{
    attempt, ax1, ax2, ax3, curve_summary, dir, flag, int, name, pnt, real, show, surface_summary,
    vec,
};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::geom_adaptor::{make_curve, make_surface};
use martensite_tkmath::geom_abs::Shape;

pub const SHAPES: [Shape; 7] = [
    Shape::C0,
    Shape::G1,
    Shape::C1,
    Shape::G2,
    Shape::C2,
    Shape::C3,
    Shape::CN,
];
const RESOLUTIONS: [f64; 4] = [1e-7, 1e-3, 0.5, 10.0];

/// a3_curve_intervals.
pub fn curve_intervals(output: &Output, a: &dyn Adaptor3dCurve) {
    for shape in SHAPES {
        let n = match a.nb_intervals(shape) {
            Ok(n) => n,
            Err(error) => {
                replay_print!(output, "{} ", error.exception_type());
                continue;
            }
        };
        int(output, n);
        let mut t = Array1::<f64>::with_bounds(1, n + 1).unwrap();
        attempt(output, || {
            a.intervals(&mut t, shape)?;
            t.iter().for_each(|v| real(output, *v));
            Ok(())
        });
    }
}

fn named<T: ?Sized + martensite_tkernel::standard::RuntimeTyped>(
    output: &Output,
    handle: Result<Handle<T>, Failure>,
    tail: impl FnOnce(&T),
) {
    show(output, handle, |h| {
        name(output, &h);
        if let Some(object) = h.get() {
            tail(object);
        }
    });
}

/// a3_curve_kinds.
pub fn curve_kinds(output: &Output, a: &dyn Adaptor3dCurve) {
    show(output, a.get_type(), |t| int(output, t as i32));
    show(output, a.line(), |l| ax1(output, l.position()));
    show(output, a.circle(), |c| {
        ax2(output, c.position());
        real(output, c.radius());
    });
    show(output, a.ellipse(), |e| {
        ax2(output, e.position());
        real(output, e.major_radius());
        real(output, e.minor_radius());
    });
    show(output, a.hyperbola(), |h| {
        ax2(output, h.position());
        real(output, h.major_radius());
        real(output, h.minor_radius());
    });
    show(output, a.parabola(), |p| {
        ax2(output, p.position());
        real(output, p.focal());
    });
    show(output, a.degree(), |value| int(output, value));
    show(output, a.is_rational(), |value| flag(output, value));
    show(output, a.nb_poles(), |value| int(output, value));
    show(output, a.nb_knots(), |value| int(output, value));
    named(output, a.bezier(), |b| int(output, b.nb_poles()));
    named(output, a.bspline(), |b| int(output, b.nb_poles()));
    named(output, a.offset_curve(), |o| real(output, o.offset()));
}

fn curve_evaluations(output: &Output, a: &dyn Adaptor3dCurve, u: f64) {
    show(output, a.eval_d0(u), |value| pnt(output, value));
    show(output, a.eval_d1(u), |e| {
        pnt(output, e.point);
        vec(output, e.d1);
    });
    show(output, a.eval_d2(u), |e| {
        pnt(output, e.point);
        [e.d1, e.d2]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.eval_d3(u), |e| {
        pnt(output, e.point);
        [e.d1, e.d2, e.d3]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    for n in 0..=5 {
        show(output, a.eval_dn(u, n), |value| vec(output, value));
    }
    show(output, a.value(u), |value| pnt(output, value));
    show(output, a.d0(u), |value| pnt(output, value));
    show(output, a.d1(u), |(p, v1)| {
        pnt(output, p);
        vec(output, v1);
    });
    show(output, a.d2(u), |(p, v1, v2)| {
        pnt(output, p);
        [v1, v2].into_iter().for_each(|value| vec(output, value));
    });
    show(output, a.d3(u), |(p, v1, v2, v3)| {
        pnt(output, p);
        [v1, v2, v3]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.dn(u, 2), |value| vec(output, value));
}

/// a3_curve_summary.
pub fn curve_adaptor_summary(output: &Output, a: &Handle<dyn Adaptor3dCurve>, params: &[f64]) {
    name(output, a);
    show(output, a.first_parameter(), |value| real(output, value));
    show(output, a.last_parameter(), |value| real(output, value));
    show(output, a.continuity(), |s| int(output, s as i32));
    curve_intervals(output, &**a);
    show(output, a.is_closed(), |value| flag(output, value));
    show(output, a.is_periodic(), |value| flag(output, value));
    show(output, a.period(), |value| real(output, value));
    for r in RESOLUTIONS {
        show(output, a.resolution(r), |value| real(output, value));
    }
    curve_kinds(output, &**a);
    for &u in params {
        curve_evaluations(output, &**a, u);
    }
}

/// a3_make_curve.
pub fn make_curve_summary(output: &Output, a: &dyn Adaptor3dCurve, params: &[f64]) {
    show(output, make_curve(a), |c| {
        name(output, &c);
        if let Some(curve) = c.get() {
            curve_summary(output, curve, params);
        }
    });
}

fn one_direction_intervals(
    output: &Output,
    count: Result<i32, Failure>,
    fill: impl FnOnce(&mut Array1<'_, f64>) -> Result<(), Failure>,
) {
    show(output, count, |n| {
        int(output, n);
        let mut t = Array1::<f64>::with_bounds(1, n + 1).unwrap();
        attempt(output, || {
            fill(&mut t)?;
            t.iter().for_each(|v| real(output, *v));
            Ok(())
        });
    });
}

/// a3_surface_intervals.
pub fn surface_intervals(output: &Output, a: &dyn Adaptor3dSurface) {
    for shape in SHAPES {
        one_direction_intervals(output, a.nb_u_intervals(shape), |t| a.u_intervals(t, shape));
        one_direction_intervals(output, a.nb_v_intervals(shape), |t| a.v_intervals(t, shape));
    }
}

fn elementary_kinds(output: &Output, a: &dyn Adaptor3dSurface) {
    show(output, a.get_type(), |t| int(output, t as i32));
    show(output, a.plane(), |p| ax3(output, p.position()));
    show(output, a.cylinder(), |c| {
        ax3(output, c.position());
        real(output, c.radius());
    });
    show(output, a.cone(), |c| {
        ax3(output, c.position());
        real(output, c.ref_radius());
        real(output, c.semi_angle());
    });
    show(output, a.sphere(), |s| {
        ax3(output, s.position());
        real(output, s.radius());
    });
    show(output, a.torus(), |t| {
        ax3(output, t.position());
        real(output, t.major_radius());
        real(output, t.minor_radius());
    });
}

/// a3_surface_kinds.
pub fn surface_kinds(output: &Output, a: &dyn Adaptor3dSurface) {
    elementary_kinds(output, a);
    for count in [a.u_degree(), a.nb_u_poles(), a.v_degree(), a.nb_v_poles()] {
        show(output, count, |value| int(output, value));
    }
    show(output, a.nb_u_knots(), |value| int(output, value));
    show(output, a.nb_v_knots(), |value| int(output, value));
    show(output, a.is_u_rational(), |value| flag(output, value));
    show(output, a.is_v_rational(), |value| flag(output, value));
    named(output, a.bezier(), |b| int(output, b.nb_u_poles()));
    named(output, a.bspline(), |b| int(output, b.nb_u_poles()));
    show(output, a.axe_of_revolution(), |value| ax1(output, value));
    show(output, a.direction(), |value| dir(output, value));
    named(output, a.basis_curve(), |c| {
        attempt(output, || {
            real(output, c.first_parameter()?);
            real(output, c.last_parameter()?);
            int(output, c.get_type()? as i32);
            Ok(())
        })
    });
    show(output, a.basis_surface(), |s| {
        name(output, &s);
        attempt(output, || {
            real(output, s.first_u_parameter()?);
            real(output, s.last_u_parameter()?);
            real(output, s.first_v_parameter()?);
            real(output, s.last_v_parameter()?);
            int(output, s.get_type()? as i32);
            Ok(())
        })
    });
    show(output, a.offset_value(), |value| real(output, value));
}

fn surface_evaluations(output: &Output, a: &dyn Adaptor3dSurface, u: f64, v: f64) {
    show(output, a.eval_d0(u, v), |value| pnt(output, value));
    show(output, a.eval_d1(u, v), |r| {
        pnt(output, r.point);
        [r.d1u, r.d1v]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.eval_d2(u, v), |r| {
        pnt(output, r.point);
        [r.d1u, r.d1v, r.d2u, r.d2v, r.d2uv]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.eval_d3(u, v), |r| {
        pnt(output, r.point);
        let d = [
            r.d1u, r.d1v, r.d2u, r.d2v, r.d2uv, r.d3u, r.d3v, r.d3uuv, r.d3uvv,
        ];
        d.into_iter().for_each(|value| vec(output, value));
    });
    for nu in 0..=3 {
        for nv in 0..=3 {
            if nu + nv > 0 {
                show(output, a.eval_dn(u, v, nu, nv), |value| vec(output, value));
            }
        }
    }
    surface_wrappers(output, a, u, v);
}

// The non-virtual Value, D0..D3 and DN.
fn surface_wrappers(output: &Output, a: &dyn Adaptor3dSurface, u: f64, v: f64) {
    show(output, a.value(u, v), |value| pnt(output, value));
    show(output, a.d0(u, v), |value| pnt(output, value));
    show(output, a.d1(u, v), |r| {
        pnt(output, r.point);
        [r.d1u, r.d1v]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.d2(u, v), |r| {
        pnt(output, r.point);
        [r.d2u, r.d2v, r.d2uv]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.d3(u, v), |r| {
        pnt(output, r.point);
        [r.d3u, r.d3v, r.d3uuv, r.d3uvv]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.dn(u, v, 1, 1), |value| vec(output, value));
}

/// a3_surface_summary.
pub fn surface_adaptor_summary(output: &Output, a: &Handle<dyn Adaptor3dSurface>, uv: &[f64]) {
    name(output, a);
    let bounds = [
        a.first_u_parameter(),
        a.last_u_parameter(),
        a.first_v_parameter(),
        a.last_v_parameter(),
    ];
    bounds
        .into_iter()
        .for_each(|b| show(output, b, |value| real(output, value)));
    show(output, a.u_continuity(), |s| int(output, s as i32));
    show(output, a.v_continuity(), |s| int(output, s as i32));
    surface_intervals(output, &**a);
    for closure in [
        a.is_u_closed(),
        a.is_v_closed(),
        a.is_u_periodic(),
        a.is_v_periodic(),
    ] {
        show(output, closure, |value| flag(output, value));
    }
    show(output, a.u_period(), |value| real(output, value));
    show(output, a.v_period(), |value| real(output, value));
    for r in RESOLUTIONS {
        show(output, a.u_resolution(r), |value| real(output, value));
        show(output, a.v_resolution(r), |value| real(output, value));
    }
    surface_kinds(output, &**a);
    for pair in uv.as_chunks::<2>().0 {
        surface_evaluations(output, &**a, pair[0], pair[1]);
    }
}

/// a3_make_surface.
pub fn make_surface_summary(output: &Output, a: &dyn Adaptor3dSurface, uv: &[f64]) {
    for trim in [true, false] {
        show(output, make_surface(a, trim), |s| {
            name(output, &s);
            if let Some(surface) = s.get() {
                surface_summary(output, surface, uv);
            }
        });
    }
}
