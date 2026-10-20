//! `lowalgos_2ddeviation::A1` body: the
//! deviation of a trimmed p-curve from its chord (GeomLib_Tool::ComputeDeviation), before
//! and after `reverse`, each followed by `don cc`, `2dfit` and a screenshot.
use crate::support::{
    checks::checkreal,
    commands::pcurve,
    restore::restore_as,
    session::{Session, ViewStep, set},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d,
    geom2d::{Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkgeombase::geom_lib::{compute_deviation, compute_deviation_pso};
use martensite_tkmath::gp::Direction2;

/// GeomliteTest_API2dCommands.cxx `2ddeviation res cc -d 8`: the PSO search on 8
/// sub-intervals (10 iterations) seeds the exact search (100 iterations); the command
/// sets `res_lin` (the chord) and `res_norm` (the deviation segment).
fn deviation_2d(s: &mut dyn Session, curve: &Handle<dyn Geom2dCurve>) -> f64 {
    let adaptor = Geom2dAdaptorCurve::with_curve(curve).unwrap();
    let (u1, u2) = (
        adaptor.first_parameter().unwrap(),
        adaptor.last_parameter().unwrap(),
    );
    let (approx, start) = compute_deviation_pso(&adaptor, u1, u2, 8, 10).unwrap();
    assert!(
        approx >= 0.0,
        "Error: Cannot compute deviation on interval."
    );
    let exact = compute_deviation(&adaptor, u1, u2, start.unwrap_or(f64::MAX), 100).unwrap();
    assert!(exact.deviation >= 0.0, "Error: Cannot compute a deviation!");
    let chord: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::from_lin2d(exact.line.unwrap())).cast();
    set(s, "res_lin", chord);
    let (point, vector) = (exact.pt_on_curve.unwrap(), exact.vec_curv_line.unwrap());
    let normal: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        point,
        Direction2::from_vector(vector).unwrap(),
    ))
    .cast();
    let segment = Geom2dTrimmedCurve::new(&normal, 0.0, exact.deviation, true, true).unwrap();
    set(
        s,
        "res_norm",
        Handle::new(segment).cast::<dyn Geom2dCurve>(),
    );
    exact.deviation
}

pub fn a1(s: &mut dyn Session) {
    let expect_deviation = 0.36597801294402493;
    s.step(ViewStep::SmallView("-2D-"));
    let r = restore_as(s, "OCC538.brep", "r").shape();
    let pcurves = pcurve(&r);
    for (index, curve) in pcurves.iter().enumerate() {
        set(s, &format!("r_{}", index + 1), curve.clone());
    }
    let cc = Geom2dTrimmedCurve::new(
        &pcurves[2],
        1.5704826035188950,
        3.1409652070377900,
        true,
        true,
    );
    let cc = set(s, "cc", Handle::new(cc.unwrap()).cast::<dyn Geom2dCurve>());
    for image in ["A1_1", "A1_2"] {
        if image == "A1_2" {
            cc.get().unwrap().reverse().unwrap();
            s.step(ViewStep::Repaint);
        }
        s.step(ViewStep::Display(&["cc"]));
        s.step(ViewStep::Fit);
        let deviation = deviation_2d(s, &cc);
        s.step(ViewStep::Xwd(image));
        checkreal("FoundDeviation", deviation, expect_deviation, 1.0e-9, 0.0);
    }
}
