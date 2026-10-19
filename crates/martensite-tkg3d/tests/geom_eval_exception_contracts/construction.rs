//! Exact construction refusals and unchanged rejected-setter state.
use super::fixtures::{curve_poles, surface_poles};
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Failure,
};
use martensite_tkg3d::geom_eval::*;
use martensite_tkmath::gp::{Axis2, Axis3, Point3};
fn report<T>(label: &str, expected: &str, result: Result<T, Failure>) {
    let failure = result.err().expect("expected rejected input");
    assert_eq!(failure.exception_type(), expected);
    let message = EXPECTED.iter().find(|(name, _)| *name == label).unwrap().1;
    assert_eq!(failure.message(), message, "{label}");
}
fn short_curve_poles() -> Array1<'static, Point3> {
    Array1::with_bounds(1, 2).unwrap()
}
fn short_surface_poles() -> Array2<'static, Point3> {
    Array2::with_bounds(1, 2, 1, 3).unwrap()
}
fn curve_weights() -> Array1<'static, f64> {
    let mut weights = Array1::with_bounds(1, 3).unwrap();
    weights.as_mut_slice().fill(1.);
    weights
}
fn surface_weights() -> Array2<'static, f64> {
    let mut weights = Array2::with_bounds(1, 3, 1, 3).unwrap();
    weights.as_mut_slice().fill(1.);
    weights
}
#[test]
fn observe_circularhelixcurve_constructor() {
    report(
        "CircularHelixCurve_constructor",
        "Standard_ConstructionError",
        GeomEvalCircularHelixCurve::new(Axis2::default(), 0., 3.),
    );
}
#[test]
fn observe_sinewavecurve_constructor() {
    report(
        "SineWaveCurve_constructor",
        "Standard_ConstructionError",
        GeomEvalSineWaveCurve::new(Axis2::default(), 0., 3., 0.),
    );
}
#[test]
fn observe_tbeziercurve_constructor() {
    report(
        "TBezierCurve_constructor",
        "Standard_ConstructionError",
        GeomEvalTBezierCurve::new(&short_curve_poles(), 1.),
    );
}
#[test]
fn observe_tbeziercurve_constructor_weighted() {
    report(
        "TBezierCurve_constructor_weighted",
        "Standard_ConstructionError",
        GeomEvalTBezierCurve::with_weights(&short_curve_poles(), &curve_weights(), 1.),
    );
}
#[test]
fn observe_ahtbeziercurve_constructor() {
    report(
        "AHTBezierCurve_constructor",
        "Standard_ConstructionError",
        GeomEvalAHTBezierCurve::new(&curve_poles(), -1, 0., 0.),
    );
}
#[test]
fn observe_ahtbeziercurve_constructor_weighted() {
    report(
        "AHTBezierCurve_constructor_weighted",
        "Standard_ConstructionError",
        GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &curve_weights(), -1, 0., 0.),
    );
}
#[test]
fn observe_tbeziersurface_constructor() {
    report(
        "TBezierSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalTBezierSurface::new(&short_surface_poles(), 1., 1.),
    );
}
#[test]
fn observe_tbeziersurface_constructor_weighted() {
    report(
        "TBezierSurface_constructor_weighted",
        "Standard_ConstructionError",
        GeomEvalTBezierSurface::with_weights(&short_surface_poles(), &surface_weights(), 1., 1.),
    );
}
#[test]
fn observe_ahtbeziersurface_constructor() {
    report(
        "AHTBezierSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalAHTBezierSurface::new(&surface_poles(), [-1, 2], [0.; 2], [0.; 2]),
    );
}
#[test]
fn observe_ahtbeziersurface_constructor_weighted() {
    report(
        "AHTBezierSurface_constructor_weighted",
        "Standard_ConstructionError",
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &surface_weights(),
            [-1, 2],
            [0.; 2],
            [0.; 2],
        ),
    );
}
#[test]
fn observe_ellipsoidsurface_constructor() {
    report(
        "EllipsoidSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalEllipsoidSurface::new(Axis3::default(), 0., 3., 4.),
    );
}
#[test]
fn observe_hyperboloidsurface_constructor() {
    report(
        "HyperboloidSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalHyperboloidSurface::new(Axis3::default(), 0., 3., HyperboloidSheetMode::OneSheet),
    );
}
#[test]
fn observe_hypparaboloidsurface_constructor() {
    report(
        "HypParaboloidSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalHypParaboloidSurface::new(Axis3::default(), 0., 3.),
    );
}
#[test]
fn observe_paraboloidsurface_constructor() {
    report(
        "ParaboloidSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalParaboloidSurface::new(Axis3::default(), 0.),
    );
}
#[test]
fn observe_circularhelicoidsurface_constructor() {
    report(
        "CircularHelicoidSurface_constructor",
        "Standard_ConstructionError",
        GeomEvalCircularHelicoidSurface::new(Axis3::default(), 0.),
    );
}
#[test]
fn observe_ellipsoidsurface_setsemiaxisa() {
    let geometry = GeomEvalEllipsoidSurface::new(Axis3::default(), 2., 3., 4.).unwrap();
    report(
        "EllipsoidSurface_SetSemiAxisA",
        "Standard_ConstructionError",
        geometry.set_semi_axis_a(0.),
    );
    assert_eq!(geometry.semi_axis_a(), 2.0);
}
#[test]
fn observe_ellipsoidsurface_setsemiaxisb() {
    let geometry = GeomEvalEllipsoidSurface::new(Axis3::default(), 2., 3., 4.).unwrap();
    report(
        "EllipsoidSurface_SetSemiAxisB",
        "Standard_ConstructionError",
        geometry.set_semi_axis_b(0.),
    );
    assert_eq!(geometry.semi_axis_b(), 3.0);
}
#[test]
fn observe_ellipsoidsurface_setsemiaxisc() {
    let geometry = GeomEvalEllipsoidSurface::new(Axis3::default(), 2., 3., 4.).unwrap();
    report(
        "EllipsoidSurface_SetSemiAxisC",
        "Standard_ConstructionError",
        geometry.set_semi_axis_c(0.),
    );
    assert_eq!(geometry.semi_axis_c(), 4.0);
}
#[test]
fn observe_hyperboloidsurface_setr1() {
    let geometry =
        GeomEvalHyperboloidSurface::new(Axis3::default(), 2., 3., HyperboloidSheetMode::OneSheet)
            .unwrap();
    report(
        "HyperboloidSurface_SetR1",
        "Standard_ConstructionError",
        geometry.set_r1(0.),
    );
    assert_eq!(geometry.r1(), 2.0);
}
#[test]
fn observe_hyperboloidsurface_setr2() {
    let geometry =
        GeomEvalHyperboloidSurface::new(Axis3::default(), 2., 3., HyperboloidSheetMode::OneSheet)
            .unwrap();
    report(
        "HyperboloidSurface_SetR2",
        "Standard_ConstructionError",
        geometry.set_r2(0.),
    );
    assert_eq!(geometry.r2(), 3.0);
}
#[test]
fn observe_hypparaboloidsurface_setsemiaxisa() {
    let geometry = GeomEvalHypParaboloidSurface::new(Axis3::default(), 2., 3.).unwrap();
    report(
        "HypParaboloidSurface_SetSemiAxisA",
        "Standard_ConstructionError",
        geometry.set_semi_axis_a(0.),
    );
    assert_eq!(geometry.semi_axis_a(), 2.0);
}
#[test]
fn observe_hypparaboloidsurface_setsemiaxisb() {
    let geometry = GeomEvalHypParaboloidSurface::new(Axis3::default(), 2., 3.).unwrap();
    report(
        "HypParaboloidSurface_SetSemiAxisB",
        "Standard_ConstructionError",
        geometry.set_semi_axis_b(0.),
    );
    assert_eq!(geometry.semi_axis_b(), 3.0);
}
#[test]
fn observe_paraboloidsurface_setfocal() {
    let geometry = GeomEvalParaboloidSurface::new(Axis3::default(), 2.).unwrap();
    report(
        "ParaboloidSurface_SetFocal",
        "Standard_ConstructionError",
        geometry.set_focal(0.),
    );
    assert_eq!(geometry.focal(), 2.0);
}
#[test]
fn observe_circularhelicoidsurface_setpitch() {
    let geometry = GeomEvalCircularHelicoidSurface::new(Axis3::default(), 2.).unwrap();
    report(
        "CircularHelicoidSurface_SetPitch",
        "Standard_ConstructionError",
        geometry.set_pitch(0.),
    );
    assert_eq!(geometry.pitch(), 2.0);
}

const EXPECTED: &[(&str, &str)] = &[
    (
        "CircularHelixCurve_constructor",
        "GeomEval_CircularHelixCurve: radius must be > 0",
    ),
    (
        "SineWaveCurve_constructor",
        "GeomEval_SineWaveCurve: amplitude must be > 0",
    ),
    (
        "TBezierCurve_constructor",
        "GeomEval_TBezierCurve: number of poles must be odd and >= 3",
    ),
    (
        "TBezierCurve_constructor_weighted",
        "GeomEval_TBezierCurve: number of poles must be odd and >= 3",
    ),
    (
        "AHTBezierCurve_constructor",
        "GeomEval_AHTBezierCurve: algebraic degree must be >= 0",
    ),
    (
        "AHTBezierCurve_constructor_weighted",
        "GeomEval_AHTBezierCurve: algebraic degree must be >= 0",
    ),
    (
        "TBezierSurface_constructor",
        "GeomEval_TBezierSurface: U pole count must be odd and >= 3",
    ),
    (
        "TBezierSurface_constructor_weighted",
        "GeomEval_TBezierSurface: U pole count must be odd and >= 3",
    ),
    (
        "AHTBezierSurface_constructor",
        "GeomEval_AHTBezierSurface: algebraic degree must be >= 0",
    ),
    (
        "AHTBezierSurface_constructor_weighted",
        "GeomEval_AHTBezierSurface: algebraic degree must be >= 0",
    ),
    (
        "EllipsoidSurface_constructor",
        "GeomEval_EllipsoidSurface: semi-axis must be > 0",
    ),
    ("HyperboloidSurface_constructor", ""),
    (
        "HypParaboloidSurface_constructor",
        "GeomEval_HypParaboloidSurface: semi-axis A must be > 0",
    ),
    (
        "ParaboloidSurface_constructor",
        "GeomEval_ParaboloidSurface: focal distance must be > 0",
    ),
    (
        "CircularHelicoidSurface_constructor",
        "GeomEval_CircularHelicoidSurface: pitch must be != 0",
    ),
    (
        "EllipsoidSurface_SetSemiAxisA",
        "GeomEval_EllipsoidSurface::SetSemiAxisA: value must be > 0",
    ),
    (
        "EllipsoidSurface_SetSemiAxisB",
        "GeomEval_EllipsoidSurface::SetSemiAxisB: value must be > 0",
    ),
    (
        "EllipsoidSurface_SetSemiAxisC",
        "GeomEval_EllipsoidSurface::SetSemiAxisC: value must be > 0",
    ),
    ("HyperboloidSurface_SetR1", ""),
    ("HyperboloidSurface_SetR2", ""),
    (
        "HypParaboloidSurface_SetSemiAxisA",
        "GeomEval_HypParaboloidSurface::SetSemiAxisA: semi-axis A must be > 0",
    ),
    (
        "HypParaboloidSurface_SetSemiAxisB",
        "GeomEval_HypParaboloidSurface::SetSemiAxisB: semi-axis B must be > 0",
    ),
    (
        "ParaboloidSurface_SetFocal",
        "GeomEval_ParaboloidSurface::SetFocal: focal distance must be > 0",
    ),
    (
        "CircularHelicoidSurface_SetPitch",
        "GeomEval_CircularHelicoidSurface::SetPitch: pitch must be != 0",
    ),
];
