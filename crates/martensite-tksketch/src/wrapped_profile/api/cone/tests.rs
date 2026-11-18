use super::*;

#[test]
fn published_cone_owners_are_send_and_sync() {
    fn require_shared<T: Send + Sync>() {}
    require_shared::<WrappedConeNormalRequest>();
    require_shared::<WrappedConeNormalResult>();
    require_shared::<WrappedConeMemberReport>();
    require_shared::<WrappedConeBudgetReport>();
    require_shared::<WrappedConeProofOptions>();
    require_shared::<WrappedConeAuthoredVolumeOptions>();
    require_shared::<WrappedConeMaterialReport>();
}

#[test]
fn cone_export_failure_keeps_the_detached_export_operation() {
    let region = SurfaceRegionId(17);
    let cause = missing_cone(region);
    let failure = export_failure(
        WrappedFailureClass::Incomplete,
        error::Cause::Cone(Box::new(cause)),
    );
    assert_eq!(failure.operation(), WrappedOperation::DetachedExport);
    assert_eq!(failure.stage(), WrappedStage::Export);
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
}

fn missing_cone(region: SurfaceRegionId) -> operation::cone::ConeFailure {
    operation::cone::ConeFailure {
        stage: WrappedStage::Export,
        class: WrappedFailureClass::Incomplete,
        region: Some(region),
        span: None,
        cause: operation::cone::ConeCause::Missing(region),
    }
}

#[test]
fn cone_default_proof_policies_are_explicit_and_finite() {
    let proof = WrappedConeProofOptions::default();
    assert_eq!(
        proof.planar_proof,
        martensite_tkbo::profile_extrusion::PlanarProofVersion::FilteredBernsteinV4
    );
    assert_eq!(proof.arithmetic.fractional_bits, 128);
    assert_eq!(proof.arithmetic.max_operations, 1_000_000);
    let integration =
        martensite_tkmath::predicates::roots::metric_bounds::CylindricalFluxOptions::default();
    assert_eq!(proof.integration, integration);
    assert!(proof.integration.absolute_error_limit.is_finite());
    assert!(proof.integration.max_total_subintervals > 0);
    assert_eq!(proof.authored_volume.subdivision_level, 6);
    assert_eq!(proof.authored_volume.clearance_level, 0);
    assert_eq!(proof.authored_volume.enclosure_width, 1e-7);
}
