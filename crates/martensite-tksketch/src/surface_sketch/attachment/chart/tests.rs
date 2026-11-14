//! Exact arithmetic controls and actual selected-face trims remain separate evidence.
use super::*;
use crate::surface_sketch::attachment::tests::capture;
use martensite_tkbo::measured::Evidence;
fn request(uv: [f64; 2]) -> CylinderParameterRequest {
    CylinderParameterRequest {
        uv,
        seam_angle: 0.25,
        outside: OutOfSupportPolicy::Refuse,
    }
}
#[test]
fn native_parameters_retain_the_authored_seam_and_analytic_coordinates() {
    let result = capture()
        .develop_parameters(SupportRevision(3), request([0.75, 2.]))
        .unwrap();
    assert_eq!(result.uv, [0.75, 2.]);
    assert!(result.coordinates[0].encloses(5.));
    assert_eq!(result.coordinates[1].value, 2.);
    assert_eq!(result.coordinates[1].error_bound, 0.);
    assert_eq!(result.coordinates[0].evidence(), Evidence::Proven);
    assert_eq!(result.trim_state, State::In);
    assert_eq!(result.trim_check, CheckMode::NativeSearch);
}
#[test]
fn outside_parameters_require_explicit_report_policy() {
    let attachment = capture();
    let error = attachment
        .develop_parameters(SupportRevision(3), request([0.75, 6.]))
        .err()
        .unwrap();
    assert!(matches!(
        error.cause,
        AttachmentFailureCause::ChartTrim { state: State::Out }
    ));
    let mut options = request([0.75, 6.]);
    options.outside = OutOfSupportPolicy::Report;
    let result = attachment
        .develop_parameters(SupportRevision(3), options)
        .unwrap();
    assert_eq!(result.trim_state, State::Out);
    assert_eq!(result.coordinates[1].value, 6.);
}
#[test]
fn parameter_mapping_rejects_stale_revisions_and_nonfinite_inputs() {
    let attachment = capture();
    let error = attachment
        .develop_parameters(SupportRevision(4), request([0.75, 2.]))
        .err()
        .unwrap();
    assert_eq!(error.class(), AttachmentFailureClass::StaleRevision);
    for uv in [[f64::NAN, 2.], [0.75, f64::INFINITY]] {
        let error = attachment
            .develop_parameters(SupportRevision(3), request(uv))
            .err()
            .unwrap();
        assert!(matches!(
            error.cause,
            AttachmentFailureCause::ChartValue { .. }
        ));
    }
}
#[test]
fn directed_chart_arithmetic_covers_subnormal_products_and_refuses_overflow() {
    let attempt = Attempt::new(
        AttachmentOperation::Develop,
        FaceAttachmentId(7),
        SupportRevision(3),
    );
    let tiny = arithmetic::arc(&attempt, 0.5, f64::from_bits(1), 0.).unwrap();
    assert_eq!(tiny.value, 0.);
    assert!(tiny.error_bound >= f64::from_bits(1));
    assert!(arithmetic::arc(&attempt, f64::MAX, 2., 0.).is_err());
    assert!(arithmetic::arc(&attempt, 1., f64::MAX, -f64::MAX).is_err());
}
#[test]
fn chart_public_values_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CylinderParameterRequest>();
    send_sync::<DevelopedCylinderHit>();
    send_sync::<OutOfSupportPolicy>();
}
fn ray_hit(face: &Face) -> [f64; 2] {
    use martensite_tkmath::gp::{Direction3, Line3, Point3};
    use martensite_tktopalgo::int_curves_face::IntCurvesFaceIntersector;
    let ray = Line3::new(
        Point3::new(20., 0., 2.),
        Direction3::new(-1., 0., 0.).unwrap(),
    );
    let mut intersector = IntCurvesFaceIntersector::new(face, 1e-7, true, true).unwrap();
    intersector.perform_line(&ray, 0., 15.).unwrap();
    assert!(intersector.is_done());
    assert_eq!(intersector.nb_pnt(), 1);
    assert_eq!(*intersector.pnt(1).unwrap().coordinates(), [10., 0., 2.]);
    [
        intersector.u_parameter(1).unwrap(),
        intersector.v_parameter(1).unwrap(),
    ]
}
#[test]
fn actual_native_ray_hit_maps_without_a_second_world_projection() {
    let attachment = capture();
    let detached = attachment.export_native(SupportRevision(3)).unwrap();
    let uv = ray_hit(&detached.face);
    let result = attachment
        .develop_parameters(SupportRevision(3), request(uv))
        .unwrap();
    assert!(result.coordinates[0].encloses(-2.5));
    assert_eq!(result.coordinates[1].value, 2.);
    assert_eq!(result.trim_state, State::On);
}
