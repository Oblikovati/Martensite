//! Owned geometric receipts survive a later native refusal without a fabricated report (#527).
use super::super::*;

#[test]
fn geometric_receipts_remain_owned_after_native_success_and_refusal() {
    let (request, geometry, shape) = certified_geometry_fixture();
    let _: &(dyn Send + Sync) = &geometry;
    let span_count = geometry.storage.len();
    assert!(geometry.coverage.binding_count > 0);
    assert!(span_count > 0 && !geometry.cones.pieces.is_empty());
    let native = qualify_native(&request, &shape).unwrap();
    let _: &(dyn Send + Sync) = &native;
    assert!(native.report.is_valid());
    assert_eq!(native.report.census[0].faces, geometry.coverage.faces);
    assert_eq!(native.report.tolerance_budget, Some(request.tolerance));
    assert!((native.volume_estimate - 11.10375).abs() < request.tolerance);
    assert!((0. ..=1e-8).contains(&native.integration_estimate));
    let mut invalid = request.clone();
    invalid.tolerance = 0.;
    assert!(qualify_native(&invalid, &shape).is_err());
    assert_eq!(geometry.storage.len(), span_count);
}

#[test]
fn invalid_resolution_keeps_the_native_validation_refusal_before_measurement() {
    let mut request = family::tests::requests().remove(0);
    request.tolerance = 0.;
    let refusal = qualify_native(&request, &Shape::default()).err().unwrap();
    assert_eq!(refusal.region, request.region);
    assert_eq!(refusal.stage, Stage::Validation);
    assert!(matches!(refusal.cause, RefusalCause::Native(_)));
    assert!(refusal.cause.to_string().contains("=0"));
}

fn certified_geometry_fixture() -> (Request, MemberGeometry, Shape) {
    let request = family::tests::requests().remove(0);
    let prepared = subdivision::prepare(&request).unwrap();
    let (_, _, shape, bindings) = construct_frozen(&prepared).unwrap();
    let stored = store_member(&request, &prepared, shape, bindings).unwrap();
    let (geometry, shape) = certify_geometry(&request, &prepared, stored, &mut || false).unwrap();
    (request, geometry, shape)
}
