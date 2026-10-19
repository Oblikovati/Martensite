//! Inherited periods refuse nonperiodic geometry and preserve its authored state.
use super::fixtures::{curve_fixtures, surface_fixtures};
use martensite_tkernel::standard::Failure;

fn period_refusal(result: Result<f64, Failure>, message: &str) {
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NoSuchObject");
    assert_eq!(failure.message(), message);
}
#[test]
fn nonperiodic_curves_use_the_base_period_literal_without_mutation() {
    for (_, curve) in curve_fixtures() {
        let before = curve.eval_d1(0.25).unwrap();
        assert!(!curve.is_periodic());
        period_refusal(curve.period(), "Geom_Curve::Period");
        assert_eq!(curve.eval_d1(0.25).unwrap(), before);
    }
}
#[test]
fn surface_u_periods_preserve_periodic_success_and_nonperiodic_refusals() {
    for (_, surface) in surface_fixtures() {
        let before = surface.eval_d1(0.25, 0.5).unwrap();
        if surface.is_u_periodic() {
            assert_eq!(surface.u_period().unwrap(), 2. * std::f64::consts::PI);
        } else {
            period_refusal(surface.u_period(), "Geom_Surface::UPeriod");
        }
        assert_eq!(surface.eval_d1(0.25, 0.5).unwrap(), before);
    }
}
#[test]
fn surface_v_periods_use_the_base_literal_without_mutation() {
    for (_, surface) in surface_fixtures() {
        let before = surface.eval_d1(0.25, 0.5).unwrap();
        assert!(!surface.is_v_periodic());
        period_refusal(surface.v_period(), "Geom_Surface::VPeriod");
        assert_eq!(surface.eval_d1(0.25, 0.5).unwrap(), before);
    }
}
