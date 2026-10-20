//! The HelixGeom builder operations of the numeric corpus.
use super::common::{Reader, Sink, attempt, dir, end, flag, int, pnt, real};
use super::helix_geom::{curve, helix_fields, shape};
use martensite_tkernel::standard::Failure;
use martensite_tkhelix::helix_geom::{
    HelixGeomBuilderHelix, HelixGeomBuilderHelixCoil, HelixGeomBuilderHelixGen,
};
use martensite_tkmath::gp::Axis2;

/// `hx_builder_state`: statuses, settings, reached tolerance and curves.
fn builder_state(out: &mut Sink, b: &dyn HelixGeomBuilderHelixGen) -> Result<(), Failure> {
    let (cont, deg, seg) = b.approx_parameters();
    let p = b.curve_parameters();
    int(out, b.error_status());
    int(out, b.warning_status());
    int(out, cont as i32);
    int(out, deg);
    int(out, seg);
    real(out, b.tolerance());
    [p.t1, p.t2, p.pitch, p.r_start, p.taper_angle]
        .into_iter()
        .for_each(|value| real(out, value));
    flag(out, p.is_cw);
    real(out, b.tolerance_reached());
    int(out, b.curves().length());
    for c in b.curves().iter() {
        curve(out, c)?;
    }
    Ok(())
}

/// `hx_builder_run`: `set tol cont maxdeg maxseg helix`, then Perform and the state.
fn builder_run(out: &mut Sink, r: &mut Reader<'_>, b: &mut dyn HelixGeomBuilderHelixGen) {
    let (set, tol, cont, deg, seg) = (r.flag(), r.real(), r.integer(), r.integer(), r.integer());
    let f = helix_fields(r);
    if set {
        b.set_tolerance(tol);
        b.set_approx_parameters(shape(cont), deg, seg);
        b.set_curve_parameters(f.t1, f.t2, f.pitch, f.r_start, f.taper, f.cw);
    }
    attempt(out, |_| b.perform());
    attempt(out, |out| builder_state(out, b));
}

/// `nruns (run)...`: HelixGeom_BuilderHelixCoil.
pub fn coil_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut b = HelixGeomBuilderHelixCoil::new();
    attempt(out, |out| builder_state(out, &b));
    for _ in 0..r.integer() {
        builder_run(out, &mut r, &mut b);
    }
    end(out);
}

/// `P N X nruns (run)...`: HelixGeom_BuilderHelix at gp_Ax2(P, N, X).
pub fn helix_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let (o, n, x) = (r.pnt(), r.dir(), r.dir());
    let mut b = HelixGeomBuilderHelix::new();
    attempt(out, |out| builder_state(out, &b));
    // The corpus only spells valid frames; gp_Ax2 would raise for a parallel X.
    b.set_position(Axis2::with_x_direction(o, n, x).expect("probe frame"));
    pnt(out, b.position().location());
    dir(out, b.position().direction());
    dir(out, b.position().x_direction());
    for _ in 0..r.integer() {
        builder_run(out, &mut r, &mut b);
    }
    end(out);
}
