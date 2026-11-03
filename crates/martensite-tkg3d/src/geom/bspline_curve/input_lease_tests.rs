//! Lease controls use actual mutable cells rather than alternate lock fixtures.
use super::super::*;
use crate::geom::ClosedSplineExtrusionInput;
use martensite_tkernel::standard::SharedReadFailure;

fn open_curve() -> GeomBSplineCurve {
    let coordinates = [Point3::new(-0.0, 2., 3.), Point3::new(4., 5., 6.)];
    let poles = Array1::from_shared(&coordinates, 1);
    let knots = Array1::from_shared(&[0., 1.], 1);
    let mults = Array1::from_shared(&[2, 2], 1);
    GeomBSplineCurve::new(&poles, &knots, &mults, 1, false).unwrap()
}

#[test]
fn stored_fields_preserve_open_linear_input_without_admission() {
    let curve = open_curve();
    let lease = curve.try_input_lease().unwrap();
    let input = lease.stored_input();
    assert_eq!(lease.stored_sizes(), [2, 2, 2, 2, 4]);
    assert_eq!(input.poles()[0][0].to_bits(), (-0.0f64).to_bits());
    assert_eq!(input.poles()[1], [4., 5., 6.]);
    assert_eq!(
        (input.degree(), input.periodic(), input.rational()),
        (1, false, false)
    );
    assert_eq!(input.weights(), &[1., 1.]);
    assert_eq!(input.knots(), &[0., 1.]);
    assert_eq!(input.multiplicities(), &[2, 2]);
    assert_eq!(input.flat_knots(), &[0., 0., 1., 1.]);
    assert_eq!(input.lower_bounds(), [1; 5]);
    assert_eq!(input.domain(), [0., 1.]);
    assert!(!input.evaluation_override());
}

#[test]
fn stored_copy_isolated_after_drop_and_actual_mutation() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<StoredSplineInput>();
    let curve = open_curve();
    let lease = curve.try_input_lease().unwrap();
    let input = lease.stored_input();
    assert!(curve.state.try_write().is_none());
    drop(lease);
    curve
        .state
        .write()
        .poles
        .set_value(1, Point3::new(7., 8., 9.))
        .unwrap();
    assert_eq!(input.poles()[0], [-0., 2., 3.]);
    assert_eq!(
        curve.try_input_lease().unwrap().stored_input().poles()[0],
        [7., 8., 9.]
    );
}

#[test]
fn spline_lease_refuses_real_writer_without_recovery() {
    let curve = open_curve();
    let writer = curve.state.write();
    assert!(matches!(
        curve.try_input_lease(),
        Err(SharedReadFailure::Busy)
    ));
    drop(writer);
    assert!(curve.try_input_lease().is_ok());
}

#[test]
fn spline_lease_reports_actual_poison() {
    let curve = open_curve();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _writer = curve.state.write();
        panic!("NamedSplineWriterPoison: expected typed poisoned read refusal");
    }));
    assert!(panic.is_err());
    assert!(matches!(
        curve.try_input_lease(),
        Err(SharedReadFailure::Poisoned)
    ));
}

#[test]
fn legacy_closed_admission_still_refuses_open_linear_layout() {
    let curve = open_curve();
    let lease = curve.try_input_lease().unwrap();
    assert!(matches!(
        lease
            .closed_extrusion_input([0., 0., 1.], [-1., 1.])
            .unwrap(),
        ClosedSplineExtrusionInput::Inconclusive(_)
    ));
}

#[test]
fn raw_capture_preserves_override_without_admitting_it() {
    use crate::geom_eval::RepCurveDescFull;
    use martensite_tkernel::standard::Handle;
    let curve = open_curve();
    let replacement = Handle::new(crate::geom::GeomLine::new(
        martensite_tkmath::gp::Axis1::default(),
    ));
    curve
        .set_eval_representation(Handle::new(RepCurveDescFull::new(replacement.cast())).cast())
        .unwrap();
    let lease = curve.try_input_lease().unwrap();
    assert!(lease.stored_input().evaluation_override());
    assert_eq!(lease.stored_input().poles()[1], [4., 5., 6.]);
    assert!(matches!(
        lease
            .closed_extrusion_input([0., 0., 1.], [-1., 1.])
            .unwrap(),
        ClosedSplineExtrusionInput::Inconclusive(_)
    ));
}

#[test]
fn raw_capture_keeps_nonuniform_weights_and_source_bits() {
    let curve = open_curve();
    {
        let mut fields = curve.state.write();
        fields.weights.set_value(2, 2.).unwrap();
        fields.rational = true;
        fields.knots.set_value(1, -0.).unwrap();
    }
    let input = curve.try_input_lease().unwrap().stored_input();
    assert_eq!(input.weights(), &[1., 2.]);
    assert!(input.rational());
    assert_eq!(input.knots()[0].to_bits(), (-0.0f64).to_bits());
}
