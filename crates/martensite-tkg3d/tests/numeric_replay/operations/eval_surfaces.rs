//! Analytic surface descriptors and derivative requests.
use super::common::{
    Reader, built_surface, dump, end, int, name, real, show, surface_summary, vec,
};
use super::output::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::geom_eval::{
    GeomEvalCircularHelicoidSurface, GeomEvalEllipsoidSurface, GeomEvalHypParaboloidSurface,
    GeomEvalHyperboloidSurface, GeomEvalParaboloidSurface, HyperboloidSheetMode,
};

// The corpus passes only 0 (OneSheet) and 1 (TwoSheets).
fn sheet_mode(value: f64) -> HyperboloidSheetMode {
    if value as i32 == 0 {
        HyperboloidSheetMode::OneSheet
    } else {
        HyperboloidSheetMode::TwoSheets
    }
}

/// One eval surface specification of kind 20 to 24, as g3_eval_surface reads it.
pub fn spec(
    _output: &Output,
    kind: i32,
    r: &mut Reader,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    let (f, a) = (r.ax3(), r.real());
    if kind == 22 {
        return Ok(Handle::new(GeomEvalParaboloidSurface::new(f, a)?).cast());
    }
    if kind == 24 {
        return Ok(Handle::new(GeomEvalCircularHelicoidSurface::new(f, a)?).cast());
    }
    let b = r.real();
    if kind == 23 {
        return Ok(Handle::new(GeomEvalHypParaboloidSurface::new(f, a, b)?).cast());
    }
    let c = r.real();
    match kind {
        20 => Ok(Handle::new(GeomEvalEllipsoidSurface::new(f, a, b, c)?).cast()),
        21 => Ok(Handle::new(GeomEvalHyperboloidSurface::new(f, a, b, sheet_mode(c))?).cast()),
        _ => panic!("eval surface kind {kind} has no Rust builder in the probe"),
    }
}

fn surface_state(output: &Output, s: &Handle<dyn GeomSurface>) {
    if let Some(e) = s.cast::<GeomEvalEllipsoidSurface>().get() {
        [e.semi_axis_a(), e.semi_axis_b(), e.semi_axis_c()]
            .into_iter()
            .for_each(|value| real(output, value));
        e.coefficients()
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(h) = s.cast::<GeomEvalHyperboloidSurface>().get() {
        real(output, h.r1());
        real(output, h.r2());
        int(output, h.mode() as i32);
        h.coefficients()
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(p) = s.cast::<GeomEvalParaboloidSurface>().get() {
        real(output, p.focal());
        p.coefficients()
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(h) = s.cast::<GeomEvalHypParaboloidSurface>().get() {
        real(output, h.semi_axis_a());
        real(output, h.semi_axis_b());
        h.coefficients()
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(h) = s.cast::<GeomEvalCircularHelicoidSurface>().get() {
        real(output, h.pitch());
    }
}

fn surface_set(output: &Output, s: &Handle<dyn GeomSurface>, [a, b, c]: [f64; 3]) {
    if let Some(e) = s.cast::<GeomEvalEllipsoidSurface>().get() {
        show(output, e.set_semi_axis_a(a), |()| {});
        show(output, e.set_semi_axis_b(b), |()| {});
        show(output, e.set_semi_axis_c(c), |()| {});
    }
    if let Some(h) = s.cast::<GeomEvalHyperboloidSurface>().get() {
        show(output, h.set_r1(a), |()| {});
        show(output, h.set_r2(b), |()| {});
        h.set_mode(sheet_mode(c));
    }
    if let Some(p) = s.cast::<GeomEvalParaboloidSurface>().get() {
        show(output, p.set_focal(a), |()| {});
    }
    if let Some(h) = s.cast::<GeomEvalHypParaboloidSurface>().get() {
        show(output, h.set_semi_axis_a(a), |()| {});
        show(output, h.set_semi_axis_b(b), |()| {});
    }
    if let Some(h) = s.cast::<GeomEvalCircularHelicoidSurface>().get() {
        show(output, h.set_pitch(a), |()| {});
    }
}

// EvalDN of total order 4 to 6 at each (u, v).
fn high_orders(output: &Output, s: &dyn GeomSurface, uv: &[f64]) {
    for pair in uv.as_chunks::<2>().0 {
        for nu in 0..=6 {
            for nv in (4 - nu).max(0)..=6 - nu {
                show(output, s.eval_dn(pair[0], pair[1], nu, nv), |value| {
                    vec(output, value)
                });
            }
        }
    }
}

/// g3_eval_surface: <surface> uv-list a b c.
pub fn eval_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let uv = r.reals();
    let setters = [r.real(), r.real(), r.real()];
    name(output, &s);
    surface_state(output, &s);
    surface_summary(output, &*s, &uv);
    let copy: Handle<dyn GeomSurface> = s.copy().expect("GeomEval copy").cast();
    name(output, &copy);
    surface_state(output, &copy);
    high_orders(output, &*s, &uv);
    surface_set(output, &s, setters);
    surface_state(output, &s);
    surface_summary(output, &*s, &uv);
    surface_state(output, &copy);
    dump(output, &*s, -1);
    dump(output, &*s, 0);
    dump(output, &*copy, 1);
    end(output);
}
