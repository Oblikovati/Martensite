//! A changed datum or changed evidence must refuse the placed exact stage.
use super::*;
use martensite_tkbo::body_check::{StageVerdict, validate_body_with_transported_conical};

const TRANSPORTED: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedTransportedPlacementV5 {
        domain: WrappedConeDomainPolicy::PositiveXRecenteredChartV4,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };

#[test]
fn oblique_transport_preserves_the_same_native_geometry_with_exact_coverage() {
    let request = fixture(NormalSense::Outward);
    let native = complete(&request);
    let transported = normal_extrude_cone_profile_on_chart(&request, TRANSPORTED).unwrap();
    verify(&request, &transported);
    assert_eq!(native.encoded_brep(), transported.encoded_brep());
    assert_eq!(
        native.canonical().history(),
        transported.canonical().history()
    );
    let compared = [
        (&native, CheckMode::NativeSearch),
        (&transported, CheckMode::Exact),
    ];
    for (outcome, mode) in compared {
        assert_modes(
            &outcome.placement().members[&SurfaceRegionId(1)].validation,
            mode,
        );
    }
}

#[test]
fn transported_members_reject_a_different_certified_frame_or_a_mixed_datum() {
    let request = fixture(NormalSense::Inward);
    let placed = normal_extrude_cone_profile_on_chart(&request, TRANSPORTED).unwrap();
    let family = &placed.base.family.family;
    let (_, bindings) = pipeline::decode(placed.encoded_brep(), &family.keys).unwrap();
    let mut members = operation_cone::placed_members(&placed.base, &bindings).unwrap();
    let member = members.get_mut(&SurfaceRegionId(1)).unwrap();
    let evidence = placed
        .base
        .family
        .budget
        .actual_certificate
        .as_deref()
        .unwrap();
    let unrelated = frame(operation_cone::tests::request().support).unwrap().1;
    let resolution = pipeline::resolution(request.options.spatial_error);
    let wrong_frame =
        validate_body_with_transported_conical(member, &resolution, evidence, &unrelated).unwrap();
    assert_rebind_refusal(&wrong_frame, "certified positive matrix");
    let frame = placed.placement().spatial_budget.frame();
    member.sides[0].1 = topods::face(
        &member.sides[0]
            .1
            .located(&Location::default(), false)
            .unwrap(),
    )
    .unwrap();
    let mixed =
        validate_body_with_transported_conical(member, &resolution, evidence, frame).unwrap();
    assert_rebind_refusal(&mixed, "does not share its root datum");
}

#[test]
fn identity_placement_rebinds_source_labels_to_the_canonical_certificate() {
    let request = operation_cone::tests::supported_common_chart_request();
    let selection = WrappedConeExecutionSelection::DirectedTransportedPlacementV5 {
        domain: WrappedConeDomainPolicy::PositiveXCommonChartV2,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let result = normal_extrude_cone_profile_on_chart(&request, selection).unwrap();
    assert_eq!(result.encoded_brep(), result.canonical().encoded_brep());
    assert_modes(
        &result.placement().members[&SurfaceRegionId(1)].validation,
        CheckMode::Exact,
    );
    reject_changed_side_label(&result, request.options.spatial_error);
}

fn reject_changed_side_label(result: &WrappedPlacedConeResult, tolerance: f64) {
    let (_, bindings) =
        pipeline::decode(result.encoded_brep(), &result.base.family.family.keys).unwrap();
    let mut members = operation_cone::placed_members(&result.base, &bindings).unwrap();
    let member = members.get_mut(&SurfaceRegionId(1)).unwrap();
    member.sides[0].0 = member.sides[1].0;
    let proof = result
        .base
        .family
        .budget
        .actual_certificate
        .as_deref()
        .unwrap();
    let report = validate_body_with_transported_conical(
        member,
        &pipeline::resolution(tolerance),
        proof,
        result.placement().spatial_budget.frame(),
    )
    .unwrap();
    assert_rebind_refusal(&report, "duplicate source side");
}

#[test]
fn placed_atlas_rebinds_each_region_and_keeps_its_chart() {
    let mut request = operation_cone::tests::atlas_request();
    apply_frame(&mut request, NormalSense::Outward);
    let selection = WrappedConeExecutionSelection::DirectedTransportedPlacementV5 {
        domain: WrappedConeDomainPolicy::PositiveXRegionAtlasV5,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let result = normal_extrude_cone_profile_on_chart(&request, selection).unwrap();
    assert_eq!(result.placement().members.len(), 2);
    for (region, report) in &result.placement().members {
        assert!(
            result
                .canonical()
                .region_chart_domain_receipt(*region)
                .is_some()
        );
        assert_modes(&report.validation, CheckMode::Exact);
    }
    assert_vertices(&request, &result.export_native().unwrap());
}

#[test]
fn the_transport_selector_requires_a_directed_domain() {
    let request = fixture(NormalSense::Outward);
    let selection = WrappedConeExecutionSelection::DirectedTransportedPlacementV5 {
        domain: WrappedConeDomainPolicy::LegacySectorV1,
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let failure = normal_extrude_cone_profile_on_chart(&request, selection)
        .err()
        .unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Unsupported);
    assert_eq!(failure.stage(), WrappedStage::Admission);
}

fn assert_modes(report: &martensite_tkbo::body_check::ValidationReport, intersection: CheckMode) {
    assert!(report.is_valid());
    assert_eq!(
        report.stage(ValidationStage::Topology).mode,
        CheckMode::Exact
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
    assert_eq!(
        report.stage(ValidationStage::SelfIntersection).mode,
        intersection
    );
    assert_eq!(
        report.stage(ValidationStage::Tolerance).verdict,
        StageVerdict::Passed
    );
}

fn assert_rebind_refusal(report: &martensite_tkbo::body_check::ValidationReport, detail: &str) {
    let si = report.stage(ValidationStage::SelfIntersection);
    assert_eq!(si.verdict, StageVerdict::Inconclusive);
    assert_eq!(si.mode, CheckMode::Exact);
    assert!(format!("{:?}", si.findings).contains(detail), "{si:?}");
    assert_eq!(
        report.stage(ValidationStage::Tolerance).verdict,
        StageVerdict::NotRun
    );
}
