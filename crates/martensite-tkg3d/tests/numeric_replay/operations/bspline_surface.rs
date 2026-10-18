//! B-spline surface evaluation and mutation fixtures.
use super::common::{
    Reader, attempt, curve_summary, dump, end, flag, int, name, pnt, real, show, surface_summary,
    trsf, vec,
};
use super::output::Output;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomGeometry, GeomSurface};
use martensite_tkmath::bsplslib::MoveRange;
use martensite_tkmath::gp::Point3;

fn knot_vector(r: &mut Reader) -> (Array1<'static, f64>, Array1<'static, i32>) {
    let n = r.integer();
    let mut k = Array1::<f64>::with_bounds(1, n).unwrap();
    let mut m = Array1::<i32>::with_bounds(1, n).unwrap();
    for i in 1..=n {
        k[i] = r.real();
        m[i] = r.integer();
    }
    (k, m)
}

/// Kind 8: bspline rational uperiodic vperiodic udeg vdeg nuk (k m)* nvk (k m)* nu nv
/// (x y z w)*(nu*nv), as g3_bspline_surface reads it.
pub fn spec(_output: &Output, r: &mut Reader) -> Result<Handle<dyn GeomSurface>, Failure> {
    let (rational, up, vp) = (r.flag(), r.flag(), r.flag());
    let (ud, vd) = (r.integer(), r.integer());
    let (uk, um) = knot_vector(r);
    let (vk, vm) = knot_vector(r);
    let (nu, nv) = (r.integer(), r.integer());
    let mut p = Array2::<Point3>::with_bounds(1, nu, 1, nv).unwrap();
    let mut w = Array2::<f64>::with_bounds(1, nu, 1, nv).unwrap();
    for i in 1..=nu {
        for j in 1..=nv {
            p.set_value(i, j, r.pnt()).unwrap();
            w.set_value(i, j, r.real()).unwrap();
        }
    }
    let surface = if rational {
        GeomBSplineSurface::with_weights(&p, &w, (&uk, &um), (&vk, &vm), (ud, vd), (up, vp))?
    } else {
        GeomBSplineSurface::new(&p, (&uk, &um), (&vk, &vm), (ud, vd), (up, vp))?
    };
    Ok(Handle::new(surface).cast())
}

fn int_array(output: &Output, a: &Array1<'_, i32>) {
    int(output, a.lower());
    int(output, a.upper());
    a.as_slice()
        .iter()
        .copied()
        .for_each(|value| int(output, value));
}
fn real_array(output: &Output, a: &Array1<'_, f64>) {
    int(output, a.lower());
    int(output, a.upper());
    a.as_slice()
        .iter()
        .copied()
        .for_each(|value| real(output, value));
}

fn state(output: &Output, s: &GeomBSplineSurface) {
    [
        s.nb_u_poles(),
        s.nb_v_poles(),
        s.nb_u_knots(),
        s.nb_v_knots(),
        s.u_degree(),
        s.v_degree(),
    ]
    .into_iter()
    .for_each(|value| int(output, value));
    [
        s.is_u_rational(),
        s.is_v_rational(),
        s.is_u_periodic(),
        s.is_v_periodic(),
    ]
    .into_iter()
    .for_each(|value| flag(output, value));
    int(output, s.u_knot_distribution() as i32);
    int(output, s.v_knot_distribution() as i32);
    int(output, s.continuity() as i32);
    show(output, s.first_u_knot_index(), |value| int(output, value));
    show(output, s.last_u_knot_index(), |value| int(output, value));
    show(output, s.first_v_knot_index(), |value| int(output, value));
    show(output, s.last_v_knot_index(), |value| int(output, value));
    real_array(output, &s.u_knots());
    real_array(output, &s.v_knots());
    int_array(output, &s.u_multiplicities());
    int_array(output, &s.v_multiplicities());
    real_array(output, &s.u_knot_sequence());
    real_array(output, &s.v_knot_sequence());
    let p = s.poles();
    for i in p.lower_row()..=p.upper_row() {
        for j in p.lower_col()..=p.upper_col() {
            pnt(output, *p.value(i, j).unwrap());
            real(output, s.weight(i, j).unwrap());
        }
    }
    flag(output, s.weights().is_some());
    flag(output, s.has_eval_representation());
    let (u1, u2, v1, v2) = s.bounds();
    [u1, u2, v1, v2]
        .into_iter()
        .for_each(|value| real(output, value));
}

fn one_based<T: Default + Clone>(n: i32) -> Array1<'static, T> {
    Array1::with_bounds(1, n).unwrap()
}

fn getters(output: &Output, s: &GeomBSplineSurface, i: i32, j: i32) {
    show(output, s.pole(i, j), |value| pnt(output, value));
    show(output, s.weight(i, j), |value| real(output, value));
    show(output, s.u_knot(i), |value| real(output, value));
    show(output, s.v_knot(j), |value| real(output, value));
    show(output, s.u_multiplicity(i), |value| int(output, value));
    show(output, s.v_multiplicity(j), |value| int(output, value));
    let extra = |cond: bool| i32::from(cond);
    let mut p =
        Array2::<Point3>::with_bounds(1, s.nb_u_poles() + extra(i > 2), 1, s.nb_v_poles()).unwrap();
    show(output, s.poles_into(&mut p), |()| {
        pnt(output, *p.value(1, 1).unwrap())
    });
    let mut w =
        Array2::<f64>::with_bounds(1, s.nb_u_poles(), 1, s.nb_v_poles() + extra(j > 2)).unwrap();
    show(output, s.weights_into(&mut w), |()| {
        real(output, *w.value(1, 1).unwrap());
        real(output, *w.value(w.upper_row(), w.upper_col()).unwrap());
    });
    let mut ku = one_based::<f64>(s.nb_u_knots() + extra(i > 2));
    show(output, s.u_knots_into(&mut ku), |()| {
        real_array(output, &ku)
    });
    let mut kv = one_based::<f64>(s.nb_v_knots());
    show(output, s.v_knots_into(&mut kv), |()| {
        real_array(output, &kv)
    });
    let mut su = one_based::<f64>(s.u_knot_sequence().len() as i32);
    show(output, s.u_knot_sequence_into(&mut su), |()| {
        real_array(output, &su)
    });
    let mut sv = one_based::<f64>(s.v_knot_sequence().len() as i32 + extra(j > 2));
    show(output, s.v_knot_sequence_into(&mut sv), |()| {
        real_array(output, &sv)
    });
    let mut mu = one_based::<i32>(s.nb_u_knots());
    show(output, s.u_multiplicities_into(&mut mu), |()| {
        int_array(output, &mu)
    });
    let mut mv = one_based::<i32>(s.nb_v_knots() + extra(i > 2));
    show(output, s.v_multiplicities_into(&mut mv), |()| {
        int_array(output, &mv)
    });
}

fn local(output: &Output, s: &GeomBSplineSurface, u: f64, v: f64, knots: ((i32, i32), (i32, i32))) {
    show(output, s.local_value(u, v, knots), |value| {
        pnt(output, value)
    });
    show(output, s.local_d1(u, v, knots), |d| {
        pnt(output, d.point);
        [d.d1u, d.d1v]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, s.local_d2(u, v, knots), |d| {
        pnt(output, d.point);
        [d.d1u, d.d1v, d.d2u, d.d2v, d.d2uv]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, s.local_d3(u, v, knots), |d| {
        pnt(output, d.point);
        [
            d.d1u, d.d1v, d.d2u, d.d2v, d.d2uv, d.d3u, d.d3v, d.d3uuv, d.d3uvv,
        ]
        .into_iter()
        .for_each(|value| vec(output, value));
    });
    for nu in 0..=2 {
        for nv in 0..=2 {
            show(output, s.local_dn(u, v, knots, (nu, nv)), |value| {
                vec(output, value)
            });
        }
    }
}

fn queries(output: &Output, s: &GeomBSplineSurface, r: &mut Reader) {
    let uv = r.reals();
    let knots = ((r.integer(), r.integer()), (r.integer(), r.integer()));
    let tol = r.reals();
    for pair in uv.chunks(2).filter(|pair| pair.len() == 2) {
        let (u, v) = (pair[0], pair[1]);
        local(output, s, u, v, knots);
        show(output, s.periodic_normalization(u, v), |(a, b)| {
            real(output, a);
            real(output, b);
        });
        for &t in &tol {
            for rep in [false, true] {
                show(output, s.locate_u(u, t, rep), |(a, b)| {
                    [a, b].into_iter().for_each(|value| int(output, value))
                });
                show(output, s.locate_v(v, t, rep), |(a, b)| {
                    [a, b].into_iter().for_each(|value| int(output, value))
                });
            }
        }
        for check in [false, true] {
            for (iso, is_u) in [(u, true), (v, false)] {
                let curve = if is_u {
                    s.u_iso_checked(iso, check)
                } else {
                    s.v_iso_checked(iso, check)
                };
                show(output, curve, |c| {
                    let c = Handle::new(c);
                    name(output, &c);
                    curve_summary(output, &*c, &[0.0, 0.5, 1.0]);
                });
            }
        }
    }
}

pub fn bspline_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(base) = super::common::built_surface(output, &mut r) else {
        return;
    };
    let s: Handle<GeomBSplineSurface> = base.cast();
    let (i, j, tol) = (r.integer(), r.integer(), r.real());
    state(output, &s);
    getters(output, &s, i, j);
    queries(output, &s, &mut r);
    int(output, GeomBSplineSurface::max_degree());
    show(output, s.resolution(tol), |(a, b)| {
        [a, b].into_iter().for_each(|value| real(output, value))
    });
    show(output, s.resolution(2. * tol), |(a, b)| {
        [a, b].into_iter().for_each(|value| real(output, value))
    });
    dump(output, &*s, -1);
    state(output, &s.copy().unwrap().cast::<GeomBSplineSurface>());
    end(output);
}

include!("bspline_surface/editing.rs");
