use super::*;
fn request() -> WrappedConeNormalRequest {
    operation::cone::tests::request()
}
#[test]
fn explicit_domain_policy_is_shared_and_defaults_to_legacy() {
    fn shared<T: Send + Sync>() {}
    shared::<WrappedConeDomainPolicy>();
    assert_eq!(
        WrappedConeDomainPolicy::default(),
        WrappedConeDomainPolicy::LegacySectorV1
    );
}
#[test]
fn both_domain_behaviors_cancel_before_source_capture() {
    for policy in [
        WrappedConeDomainPolicy::LegacySectorV1,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
    ] {
        let error =
            normal_extrude_cone_profile_with_domain_policy_and_cancel(&request(), policy, || true)
                .err()
                .unwrap();
        assert_eq!(error.class(), WrappedFailureClass::Cancelled);
        assert_eq!(error.stage(), WrappedStage::Admission);
    }
}
#[test]
fn explicit_legacy_preserves_admission_failure_before_a_late_cancel() {
    let mut input = request();
    input.support.semi_angle = 0.;
    let legacy = normal_extrude_cone_profile(&input).err().unwrap();
    for policy in [
        WrappedConeDomainPolicy::LegacySectorV1,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
    ] {
        assert_late_cancel(&input, policy, &legacy);
    }
}

fn assert_late_cancel(
    input: &WrappedConeNormalRequest,
    policy: WrappedConeDomainPolicy,
    legacy: &WrappedNormalFailure,
) {
    let mut polls = 0;
    let error = normal_extrude_cone_profile_with_domain_policy_and_cancel(input, policy, || {
        polls += 1;
        polls > 1
    })
    .err()
    .unwrap();
    assert_eq!(error.class(), legacy.class());
    assert_eq!(error.stage(), legacy.stage());
    assert_eq!(error.entities(), legacy.entities());
    assert_eq!(error.to_string(), legacy.to_string());
    assert_eq!(polls, 1);
}
