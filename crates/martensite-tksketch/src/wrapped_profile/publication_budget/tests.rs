//! The budget gate retains honest native coverage and one aggregate volume allowance.
use super::super::family::tests::requests;
use super::*;
use martensite_tkbo::body_check::{CheckMode, StageVerdict, ValidationStage};
fn fixture() -> (Vec<Request>, family::UnqualifiedFamily) {
    let inputs = requests();
    let family = family::construct(&inputs).unwrap_or_else(|e| panic!("{e}"));
    (inputs, family)
}
#[test]
fn requested_effective_and_achieved_bounds_are_separate() {
    let (inputs, family) = fixture();
    let r = certify(&inputs, &family, CylindricalMeasureLimit::Refuse(1e-4)).unwrap();
    assert_eq!(r.limits.spatial, 1e-5);
    assert_eq!(r.limits.profile, (1e-5f64 / 4.).next_down());
    assert_eq!(r.limits.storage, (1e-5f64 / 1000.).next_down());
    assert_eq!(r.limits.clearance, 4e-5);
    assert_eq!(r.members.len(), 2);
    assert!(r.volume.discrepancy <= 1e-4 && r.volume.representative_error <= 1e-4);
    assert!(r.volume.integration_error > 0. && r.volume.profile_error > 0.);
    for member in r.members.values() {
        assert_member_limits(member, r.limits);
    }
    fn send_sync<T: Send + Sync>() {}
    send_sync::<BudgetEvidence>();
}
fn assert_member_limits(member: &MemberBounds, limits: BudgetLimits) {
    assert!(member.profile <= limits.profile);
    assert!(member.storage <= limits.storage);
    assert!(member.trace <= limits.trace);
    assert!(member.trace_tolerance <= limits.trace);
    assert_eq!(member.native_modes[1], CheckMode::Sampled);
    assert_eq!(member.native_modes[2], CheckMode::NativeSearch);
}
#[test]
fn the_allowance_is_not_reset_for_each_member() {
    let (inputs, family) = fixture();
    let maximum = family.members.values().map(member_error).fold(0., f64::max);
    let report = certify(&inputs, &family, CylindricalMeasureLimit::Report).unwrap();
    assert!(
        report
            .volume
            .discrepancy
            .max(report.volume.representative_error)
            > maximum
    );
    assert!(matches!(
        certify(&inputs, &family, CylindricalMeasureLimit::Refuse(maximum)),
        Err(BudgetFailure::Limit { region: None, .. })
    ));
}
fn member_error(member: &pipeline::RetainedMember) -> f64 {
    let p = &member.measures.comparison;
    assert!(p.discrepancy_bound() > 0.);
    p.discrepancy_bound().max(p.representative_error_bound())
}
#[test]
fn failed_inconclusive_or_unrun_native_stages_refuse() {
    let (inputs, mut family) = fixture();
    for index in 0..4 {
        check_verdicts(&inputs, &mut family, index);
    }
}
fn check_verdicts(inputs: &[Request], family: &mut family::UnqualifiedFamily, index: usize) {
    let original = first_member(family).report.clone();
    for verdict in [
        StageVerdict::Failed,
        StageVerdict::Inconclusive,
        StageVerdict::NotRun,
    ] {
        first_member(family).report.stages[index].verdict = verdict;
        assert_native_refusal(certify(inputs, family, CylindricalMeasureLimit::Report));
        first_member(family).report = original.clone();
    }
}
fn assert_native_refusal(result: Checked<BudgetEvidence>) {
    assert!(matches!(result, Err(BudgetFailure::Native { .. })));
}
fn first_member(family: &mut family::UnqualifiedFamily) -> &mut pipeline::RetainedMember {
    family.members.get_mut(&SurfaceRegionId(5)).unwrap()
}
#[test]
fn changed_sources_missing_proofs_and_promoted_modes_refuse() {
    let (mut inputs, mut family) = fixture();
    inputs[0].tolerance *= 2.;
    assert!(matches!(
        certify(&inputs, &family, CylindricalMeasureLimit::Report),
        Err(BudgetFailure::Source(_))
    ));
    inputs[0].tolerance /= 2.;
    check_promoted_mode(&inputs, &mut family);
    first_member(&mut family).storage.pop();
    assert!(matches!(
        certify(&inputs, &family, CylindricalMeasureLimit::Report),
        Err(BudgetFailure::Coverage {
            field: "storage spans",
            ..
        })
    ));
}
fn check_promoted_mode(inputs: &[Request], family: &mut family::UnqualifiedFamily) {
    first_member(family).report.stages[1].mode = CheckMode::Exact;
    assert!(matches!(
        certify(inputs, family, CylindricalMeasureLimit::Report),
        Err(BudgetFailure::Native {
            stage: ValidationStage::Geometry,
            ..
        })
    ));
    first_member(family).report.stages[1].mode = CheckMode::Sampled;
}
#[test]
fn input_order_does_not_change_budget_or_accumulation_order() {
    let (mut inputs, family) = fixture();
    let expected = certify(&inputs, &family, CylindricalMeasureLimit::Report).unwrap();
    inputs.reverse();
    assert_eq!(
        certify(&inputs, &family, CylindricalMeasureLimit::Report).unwrap(),
        expected
    );
}
#[test]
fn invalid_policies_and_unrepresentable_bounds_refuse() {
    for v in [f64::NAN, f64::INFINITY, -1.] {
        assert!(BudgetLimits::new(1e-5, CylindricalMeasureLimit::Refuse(v)).is_err());
    }
    for v in [0., -1., f64::NAN, f64::INFINITY, f64::MAX] {
        assert!(BudgetLimits::new(v, CylindricalMeasureLimit::Report).is_err());
    }
    assert!(volume::add_upper(f64::MAX, f64::MAX, Quantity::Integration).is_err());
    assert!(volume::add_upper(0., -1., Quantity::Integration).is_err());
    assert_eq!(
        volume::add_upper(0., 0., Quantity::Integration).unwrap(),
        0.
    );
}
#[test]
fn upward_sums_enclose_half_ulp_loss_and_subnormal_addition() {
    assert_eq!(
        volume::add_upper(
            1.,
            f64::from_bits(0x3ca0_0000_0000_0000),
            Quantity::Integration
        )
        .unwrap(),
        1f64.next_up()
    );
    let tiny = f64::from_bits(1);
    assert!(volume::add_upper(tiny, tiny, Quantity::Integration).unwrap() >= f64::from_bits(2));
    assert!(volume::add_upper(0.1, 0.2, Quantity::Integration).unwrap() >= 0.30000000000000004);
}

#[test]
fn budget_validation_checks_cone_sources_instead_of_only_counts() {
    let (inputs, mut family) = fixture();
    let cones = &mut first_member(&mut family).cones;
    cones.pieces[0].certificate = cones.pieces[1].certificate.clone();
    assert!(matches!(
        certify(&inputs, &family, CylindricalMeasureLimit::Report),
        Err(BudgetFailure::Cones {
            region: SurfaceRegionId(5),
            ..
        })
    ));
}
