//! Complete owned text uses the same bounded cone executor as the walking skeleton (#425).
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkmath::predicates::roots::metric_bounds::ConicalChartDomainVersion;
#[path = "conical_text/oracles.rs"]
mod oracles;

#[test]
fn outward_multiline_cone_text_completes_under_explicit_input_bound_and_original_spatial_cap() {
    complete(NormalSense::Outward);
}

#[test]
fn inward_multiline_cone_text_completes_under_explicit_input_bound_and_original_spatial_cap() {
    complete(NormalSense::Inward);
}

#[test]
fn original_fixed_allowance_still_refuses_full_cone_text_during_fitting() {
    let request = request(NormalSense::Outward);
    let before = request.clone();
    let selection = WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let error = normal_extrude_cone_profile_with_selection(&request, selection)
        .err()
        .expect("original one-million allowance must refuse full text");
    assert_eq!(error.stage(), WrappedStage::Construction);
    assert!(format!("{error:?}").contains("WorkLimit"));
    assert_eq!(request, before);
}

fn complete(normal: NormalSense) {
    let mut request = request(normal);
    request.proof.arithmetic.max_operations = 8_810_496;
    let before = request.clone();
    let selection = WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 {
        arithmetic: WrappedConeArithmeticGrantPolicy::InputBoundV1,
    };
    let started = std::time::Instant::now();
    let outcome = normal_extrude_cone_profile_with_selection(&request, selection);
    eprintln!(
        r#"{{"event":"conical_text_api","normal":"{normal:?}","seconds":{}}}"#,
        started.elapsed().as_secs_f64()
    );
    let result = outcome.unwrap_or_else(|cause| panic!("complete cone text refused: {cause:?}"));
    assert_eq!(request, before);
    check_policies(&result, &request, selection);
    check_reports(&result);
    oracles::check_volume(&result, normal);
    check_export_and_isolation(&result, &mut request);
}

fn request(normal: NormalSense) -> WrappedConeNormalRequest {
    let mut source = text_request();
    develop_profile(&mut source.profile);
    source.options.planar_work = PlanarProofWorkLimit::PairBisections(8192);
    assert_eq!(
        (source.profile.curves.len(), source.selections.len()),
        (239, 18)
    );
    WrappedConeNormalRequest {
        profile: source.profile,
        selections: source.selections,
        support: support(normal),
        depths: source.depths,
        options: source.options,
        proof: proof_options(),
    }
}

fn develop_profile(profile: &mut SketchProfile) {
    for vertex in &mut profile.vertices {
        vertex.point = developed_point(vertex.point);
    }
    for curve in &mut profile.curves {
        if let ProfileCurve::Spline { poles, .. } = curve {
            for point in poles {
                *point = developed_point(*point);
            }
        }
    }
}

fn developed_point([x, y]: [f64; 2]) -> [f64; 2] {
    let point = [100. + y, if x == 0. { 0. } else { -x }];
    assert!(
        (95. ..=105.).contains(&point[0]) && (-25. ..=5.).contains(&point[1]),
        "developed point={point:?}; expected oracle box"
    );
    point
}

fn support(normal: NormalSense) -> ConeSupport {
    ConeSupport {
        apex: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        semi_angle: 0.125,
        chart_origin: 0.,
        normal,
    }
}

fn proof_options() -> WrappedConeProofOptions {
    let mut proof = WrappedConeProofOptions::default();
    proof.integration.behavior_version = 2;
    proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: u64::from(proof.integration.max_total_subintervals),
    };
    assert_eq!(proof.arithmetic.max_operations, 1_000_000);
    proof
}

fn check_policies(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
) {
    assert_eq!(result.execution_selection(), selection);
    assert_eq!(
        result.domain_policy(),
        WrappedConeDomainPolicy::PositiveXFamilyChartV3
    );
    assert_eq!(result.proof_options(), request.proof);
    assert_eq!(result.options(), request.options);
    check_arithmetic_receipt(result);
    let domain = result.chart_domain_receipt().unwrap();
    assert_eq!(domain.version(), ConicalChartDomainVersion::FamilyV3);
    assert_eq!(
        (domain.source_span_count(), domain.span_limit()),
        (239, 4096)
    );
    assert!(result.budget().arithmetic_operations <= request.proof.arithmetic.max_operations);
    eprintln!(
        r#"{{"event":"conical_text_work","arithmetic_operations":{},"integration_subintervals":{}}}"#,
        result.budget().arithmetic_operations,
        result.budget().integration_subintervals
    );
}

fn check_arithmetic_receipt(result: &WrappedConeNormalResult) {
    let receipt = result.arithmetic_grant_receipt().unwrap();
    assert_eq!(
        receipt.policy,
        WrappedConeArithmeticGrantPolicy::InputBoundV1
    );
    assert_eq!(receipt.source_spans, 239);
    assert_eq!(receipt.maximum_fit_level, 8);
    assert_eq!(receipt.operations_per_span_level, 4096);
    assert_eq!(receipt.requested_operations, 8_810_496);
    assert_eq!(receipt.input_ceiling, 8_810_496);
    assert_eq!(receipt.hard_ceiling, 16_000_000);
    assert_eq!(receipt.primitive_ceiling, 1_000_000);
}

fn check_reports(result: &WrappedConeNormalResult) {
    assert_eq!(result.reports().len(), 18);
    assert_eq!(
        result
            .actual_self_intersection_coverage()
            .unwrap()
            .cap_self_domains,
        36
    );
    for report in result.reports().values() {
        assert!(report.validation.is_valid());
        assert!(!report.validation.is_exact());
        assert_eq!(
            report.validation.stage(ValidationStage::Geometry).mode,
            CheckMode::Sampled
        );
        assert_eq!(
            report
                .validation
                .stage(ValidationStage::SelfIntersection)
                .mode,
            CheckMode::Exact
        );
    }
}

fn check_export_and_isolation(
    result: &WrappedConeNormalResult,
    request: &mut WrappedConeNormalRequest,
) {
    let bytes = result.encoded_brep().to_vec();
    let first = result.export_native().unwrap();
    oracles::check_vertices(request, &first);
    request.profile.vertices.clear();
    request.profile.curves.clear();
    let second = result.export_native().unwrap();
    assert!(!first.bodies.is_partner(&second.bodies));
    assert_eq!(first.entities.len(), result.history().len());
    assert_eq!(
        first.entities.keys().collect::<Vec<_>>(),
        second.entities.keys().collect::<Vec<_>>()
    );
    assert_eq!(result.encoded_brep(), bytes);
}
