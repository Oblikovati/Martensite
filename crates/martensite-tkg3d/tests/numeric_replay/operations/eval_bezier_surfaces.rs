//! Trigonometric and polynomial Bezier patch requests.
use super::common::{
    built_surface, dump, end, flag, int, name, pnt, real, show, surface_summary, trsf, vec,
};
use super::output::Output;
use super::print::Reader;
use martensite_tkernel::ncollection::Array2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::geom_eval::{GeomEvalAHTBezierSurface, GeomEvalTBezierSurface};
use martensite_tkmath::gp::{Point3, Transform3};

// The pole and weight grids of a spec, both starting at (lr, lc).
fn grids(
    _output: &Output,
    r: &mut Reader,
    lr: i32,
    lc: i32,
) -> (Array2<'static, Point3>, Array2<'static, f64>) {
    let (nu, nv) = (r.integer(), r.integer());
    let mut poles = Array2::with_bounds(lr, lr + nu - 1, lc, lc + nv - 1).unwrap();
    for i in lr..lr + nu {
        for j in lc..lc + nv {
            poles.set_value(i, j, r.pnt()).unwrap();
        }
    }
    let (nwu, nwv) = (r.integer(), r.integer());
    let mut weights = Array2::with_bounds(lr, lr + nwu - 1, lc, lc + nwv - 1).unwrap();
    for i in lr..lr + nwu {
        for j in lc..lc + nwv {
            weights.set_value(i, j, r.real()).unwrap();
        }
    }
    (poles, weights)
}

/// One eval Bezier surface specification: 30 T-Bezier, 31 AHT-Bezier.
pub fn spec(
    output: &Output,
    kind: i32,
    r: &mut Reader,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    let (rational, lr, lc) = (r.flag(), r.integer(), r.integer());
    if kind == 30 {
        let (p, w) = grids(output, r, lr, lc);
        let (au, av) = (r.real(), r.real());
        let surface = if rational {
            GeomEvalTBezierSurface::with_weights(&p, &w, au, av)?
        } else {
            GeomEvalTBezierSurface::new(&p, au, av)?
        };
        return Ok(Handle::new(surface).cast());
    }
    assert_eq!(kind, 31, "unknown eval Bezier surface kind {kind}");
    let degrees = [r.integer(), r.integer()];
    let alphas = [r.real(), r.real()];
    let betas = [r.real(), r.real()];
    let (p, w) = grids(output, r, lr, lc);
    let surface = if rational {
        GeomEvalAHTBezierSurface::with_weights(&p, &w, degrees, alphas, betas)?
    } else {
        GeomEvalAHTBezierSurface::new(&p, degrees, alphas, betas)?
    };
    Ok(Handle::new(surface).cast())
}

fn grid<T: Copy>(output: &Output, a: &Array2<'_, T>, values: bool, print: impl Fn(T)) {
    int(output, a.lower_row());
    int(output, a.lower_col());
    int(output, a.nb_rows() as i32);
    int(output, a.nb_columns() as i32);
    if values {
        a.as_slice().iter().for_each(|&x| print(x));
    }
}

fn state(output: &Output, s: &Handle<dyn GeomSurface>) {
    let t: Handle<GeomEvalTBezierSurface> = s.cast();
    if !t.is_null() {
        [t.nb_u_poles(), t.nb_v_poles(), t.order_u(), t.order_v()]
            .into_iter()
            .for_each(|value| int(output, value));
        [t.alpha_u(), t.alpha_v()]
            .into_iter()
            .for_each(|value| real(output, value));
        flag(output, t.is_rational());
        grid(output, &t.poles(), true, |value| pnt(output, value));
        grid(output, &t.weights(), true, |value| real(output, value));
    }
    let a: Handle<GeomEvalAHTBezierSurface> = s.cast();
    if !a.is_null() {
        [
            a.nb_poles_u(),
            a.nb_poles_v(),
            a.alg_degree_u(),
            a.alg_degree_v(),
        ]
        .into_iter()
        .for_each(|value| int(output, value));
        [a.alpha_u(), a.alpha_v(), a.beta_u(), a.beta_v()]
            .into_iter()
            .for_each(|value| real(output, value));
        flag(output, a.is_u_rational());
        flag(output, a.is_v_rational());
        grid(output, &a.poles(), true, |value| pnt(output, value));
        grid(output, &a.weights(), a.is_u_rational(), |value| {
            real(output, value)
        });
    }
}

// EvalDN of total orders 4 to 6, beyond the summary's 0..3 per direction.
fn high_orders(output: &Output, s: &Handle<dyn GeomSurface>, uv: &[f64]) {
    for pair in uv.as_chunks::<2>().0 {
        for nu in 0..=5 {
            for nv in (4 - nu).max(0)..=6 - nu {
                show(output, s.eval_dn(pair[0], pair[1], nu, nv), |value| {
                    vec(output, value)
                });
            }
        }
    }
}

fn refusals(output: &Output, s: &Handle<dyn GeomSurface>, t: Transform3) {
    show(output, s.u_reverse(), |()| {});
    show(output, s.v_reverse(), |()| {});
    show(output, s.u_iso(0.5), |c| name(output, &c));
    show(output, s.v_iso(0.5), |c| name(output, &c));
    show(output, s.transform(t), |()| {});
    show(output, s.transformed(t), |g| name(output, &g));
    show(output, s.u_reversed(), |c| name(output, &c));
}

/// g3_eval_bezier_surface: accessors of the surface and its copy, the copy's summary,
/// high EvalDN orders, the refused members and the dumps.
pub fn eval_bezier_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let uv = r.reals();
    let t = trsf(&mut r);
    name(output, &s);
    state(output, &s);
    let copy: Handle<dyn GeomSurface> = s.copy().unwrap().cast();
    name(output, &copy);
    state(output, &copy);
    surface_summary(output, &*copy, &uv);
    high_orders(output, &s, &uv);
    show(output, s.eval_dn(0.5, 0.5, 2, -1), |value| {
        vec(output, value)
    });
    show(output, s.eval_dn(0.5, 0.5, 0, 0), |value| {
        vec(output, value)
    });
    refusals(output, &s, t);
    for depth in [-1, 0, 1, 2] {
        dump(output, &*s, depth);
    }
    end(output);
}
