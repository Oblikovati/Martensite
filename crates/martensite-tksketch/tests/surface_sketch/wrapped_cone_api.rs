//! Cone consumers import the operation, proof policies and actual reports from the public module.
use martensite_tksketch::surface_sketch::{
    ConeSupport, NormalSense, WrappedConeAuthoredVolumeOptions, WrappedConeBudgetReport,
    WrappedConeExecutionSelection, WrappedConeMaterialReport, WrappedConeMemberReport,
    WrappedConeNormalRequest, WrappedConeNormalResult, WrappedConeProofOptions,
    WrappedConeVolumeBounds, WrappedFailureClass, WrappedStage, normal_extrude_cone_profile,
    normal_extrude_cone_profile_with_cancel, normal_extrude_cone_profile_with_selection,
    normal_extrude_cone_profile_with_selection_and_cancel,
};

#[test]
fn additive_public_selection_preserves_admission_and_cancellation() {
    let request = invalid_cone_request();
    let selection = WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1;
    let refused = normal_extrude_cone_profile_with_selection(&request, selection)
        .err()
        .unwrap();
    assert_eq!(refused.class(), WrappedFailureClass::InvalidInput);
    assert_eq!(refused.stage(), WrappedStage::Admission);
    let cancelled =
        normal_extrude_cone_profile_with_selection_and_cancel(&request, selection, || true)
            .err()
            .unwrap();
    assert_eq!(cancelled.class(), WrappedFailureClass::Cancelled);
    assert_eq!(cancelled.stage(), WrappedStage::Admission);
}

#[test]
fn public_cone_entry_points_preserve_typed_admission_and_cancellation() {
    let request = invalid_cone_request();
    let refused = normal_extrude_cone_profile(&request).err().unwrap();
    assert_eq!(refused.class(), WrappedFailureClass::InvalidInput);
    assert_eq!(refused.stage(), WrappedStage::Admission);
    let cancelled = normal_extrude_cone_profile_with_cancel(&request, || true)
        .err()
        .unwrap();
    assert_eq!(cancelled.class(), WrappedFailureClass::Cancelled);
    assert_eq!(cancelled.stage(), WrappedStage::Admission);
}

#[test]
fn public_cone_reports_and_proof_settings_are_shared_owners() {
    fn require_shared<T: Send + Sync>() {}
    require_shared::<WrappedConeNormalRequest>();
    require_shared::<WrappedConeExecutionSelection>();
    require_shared::<WrappedConeNormalResult>();
    require_shared::<WrappedConeMemberReport>();
    require_shared::<WrappedConeMaterialReport>();
    require_shared::<WrappedConeBudgetReport>();
    require_shared::<WrappedConeVolumeBounds>();
    require_shared::<WrappedConeProofOptions>();
    require_shared::<WrappedConeAuthoredVolumeOptions>();
}

fn invalid_cone_request() -> WrappedConeNormalRequest {
    let cylinder = super::wrapped_profile::request();
    WrappedConeNormalRequest {
        profile: cylinder.profile,
        selections: cylinder.selections,
        support: invalid_support(),
        depths: cylinder.depths,
        options: cylinder.options,
        proof: WrappedConeProofOptions::default(),
    }
}

fn invalid_support() -> ConeSupport {
    ConeSupport {
        apex: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        semi_angle: 0.,
        chart_origin: 0.,
        normal: NormalSense::Outward,
    }
}
