use super::*;

#[test]
fn restored_selected_face_supplies_geometry_and_keeps_the_requested_seam() {
    let source = super::super::tests::capture();
    let restored = CylinderFaceAttachment::restore(source.record().clone()).unwrap();
    let chart = restored
        .extrusion_support(SupportRevision(3), 0.125, NormalSense::Inward)
        .unwrap();
    assert_eq!(chart.origin, [0.; 3]);
    assert_eq!(chart.axis, [0., 0., 1.]);
    assert_eq!(chart.radius, 10.);
    assert_eq!(chart.chart_origin, 0.125);
    assert_eq!(chart.normal, NormalSense::Inward);
    let hit = restored
        .develop_parameters(
            SupportRevision(3),
            CylinderParameterRequest {
                uv: [0.25, 2.],
                seam_angle: chart.chart_origin,
                outside: OutOfSupportPolicy::Refuse,
            },
        )
        .unwrap();
    assert!(hit.coordinates[0].encloses(1.25));
    assert_eq!(source.record(), restored.record());
}

#[test]
fn support_chart_checks_revision_before_seam() {
    let source = super::super::tests::capture();
    let stale = source
        .extrusion_support(SupportRevision(4), f64::NAN, NormalSense::Outward)
        .unwrap_err();
    assert_eq!(stale.class(), AttachmentFailureClass::StaleRevision);
    let invalid = source
        .extrusion_support(SupportRevision(3), f64::NAN, NormalSense::Outward)
        .unwrap_err();
    assert!(matches!(
        invalid.cause,
        AttachmentFailureCause::ChartValue {
            field: "seam angle",
            ..
        }
    ));
}
