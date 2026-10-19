//! Every constructor rejection branch, its precedence, and IEEE comparison boundaries.
use super::fixtures::{curve_poles, surface_poles};
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Failure,
};
use martensite_tkg3d::geom_eval::*;
use martensite_tkmath::gp::{Axis2, Axis3};

fn rejected<T>(result: Result<T, Failure>, class: &str, reason: &str) {
    let error = result.err().expect("invalid parameters must fail");
    assert_eq!(error.exception_type(), "Standard_ConstructionError");
    assert_eq!(error.message(), format!("GeomEval_{class}: {reason}"));
}
fn weights(count: i32, value: f64) -> Array1<'static, f64> {
    let mut weights = Array1::with_bounds(-2, count - 3).unwrap();
    weights.as_mut_slice().fill(value);
    weights
}
fn grid_weights(rows: i32, cols: i32, value: f64) -> Array2<'static, f64> {
    let mut weights = Array2::with_bounds(-2, rows - 3, 5, cols + 4).unwrap();
    weights.as_mut_slice().fill(value);
    weights
}

mod curve_branches;
mod surface_branches;
type RejectedCase = (&'static str, &'static str, fn() -> Result<(), Failure>);
fn check_cases(cases: &[RejectedCase]) {
    for &(class, reason, build) in cases {
        rejected(build(), class, reason);
    }
}

#[test]
fn positive_parameter_ieee_comparisons() {
    for accepted in [f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
        assert!(GeomEvalCircularHelixCurve::new(Axis2::default(), accepted, 0.).is_ok());
        assert!(GeomEvalSineWaveCurve::new(Axis2::default(), accepted, accepted, 0.).is_ok());
        assert!(GeomEvalParaboloidSurface::new(Axis3::default(), accepted).is_ok());
    }
    for rejected in [-1., -0., 0., f64::NEG_INFINITY] {
        assert!(GeomEvalCircularHelixCurve::new(Axis2::default(), rejected, 0.).is_err());
        assert!(GeomEvalSineWaveCurve::new(Axis2::default(), 1., rejected, 0.).is_err());
        assert!(GeomEvalParaboloidSurface::new(Axis3::default(), rejected).is_err());
    }
}
#[test]
fn helicoid_pitch_accepts_negative_and_unordered_values() {
    for accepted in [-1., f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
        let surface = GeomEvalCircularHelicoidSurface::new(Axis3::default(), accepted).unwrap();
        assert!(surface.set_pitch(accepted).is_ok());
    }
    for pitch in [-0., 0.] {
        rejected(
            GeomEvalCircularHelicoidSurface::new(Axis3::default(), pitch),
            "CircularHelicoidSurface",
            "pitch must be != 0",
        );
    }
}
