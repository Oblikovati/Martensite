//! Full coverage and world-space oracles for a chart crossing the cardinal angular cut.
use super::*;
use crate::surface_sketch::normal_extrude_cone_profile_with_selection;
use martensite_cmath::{hypot, sincos};
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkbo::profile_extrusion::PlanarProofVersion;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkmath::gp::Point3;

const RECENTERED_WORKER: &str = "surface_sketch::wrapped_profile::operation::cone::tests::completed::recentered::process_worker";
const SELECTED: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };

#[test]
fn recentered_cone_crosses_the_old_cut_with_exact_si_and_repeatable_history() {
    let source = crossing_request();
    let rejected = normal_extrude_cone_profile_with_selection(
        &source,
        WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 {
            arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
        },
    );
    assert!(rejected.is_err());
    let result = complete(&source, SELECTED);
    check_crossing(&source, &result);
    assert_eq!(result.execution_selection(), SELECTED);
    assert_eq!(
        result_signature(&result),
        process_signature(RECENTERED_WORKER)
    );
}

#[test]
#[ignore = "subprocess worker invoked by recentered cone process replay"]
fn process_worker() {
    let result = complete(&crossing_request(), SELECTED);
    println!("{PREFIX}{}", result_signature(&result));
}

#[test]
fn recentered_directed_cone_rebinds_exact_constant_native_weights() {
    let mut source = request();
    source.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    let result = complete(&source, SELECTED);
    let member = &result.family.family.members[&SurfaceRegionId(1)];
    let radial = member
        .geometry
        .spans
        .iter()
        .find(|span| span.span.curve == SurfaceCurveId(1))
        .unwrap();
    for row in &radial.curves {
        assert!(row.weights.iter().all(|weight| *weight == 1.));
    }
    assert!(result.reports()[&SurfaceRegionId(1)].validation.is_valid());
}

#[test]
fn recentered_inward_normal_preserves_world_positions_and_depth_roles() {
    let mut source = crossing_request();
    source.support.normal = NormalSense::Inward;
    let result = complete(&source, SELECTED);
    check_crossing(&source, &result);
}

#[test]
fn recentered_rounded_chart_keeps_the_explicit_native_search_mode() {
    let source = crossing_request();
    let selected = WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 {
        domain: WrappedConeDomainPolicy::PositiveXRecenteredChartV4,
    };
    let result = complete(&source, selected);
    publication::assert_publication_core(&result, &source);
    assert_eq!(result.execution_selection(), selected);
    assert!(result.actual_self_intersection_coverage().is_none());
    assert!(result.reports()[&SurfaceRegionId(1)].validation.is_valid());
    check_depth_vertices(&source, &result);
}

fn check_crossing(source: &WrappedConeNormalRequest, result: &WrappedConeNormalResult) {
    publication::assert_publication_core(result, source);
    assert_eq!(result.support(), source.support);
    let chart = result.chart_domain_receipt().unwrap();
    assert_eq!(chart.version(), ConicalChartDomainVersion::RecenteredV4);
    assert_eq!(chart.chart_origin().to_bits(), 0);
    let [lower, upper] = chart.relative_physical_bounds();
    assert!(lower.binary64_bounds().unwrap()[1] < std::f64::consts::PI);
    assert!(upper.binary64_bounds().unwrap()[0] > std::f64::consts::PI);
    check_member(result);
    check_authored_volume(source, result);
    check_depth_vertices(source, result);
}

fn check_member(result: &WrappedConeNormalResult) {
    let report = &result.reports()[&SurfaceRegionId(1)];
    assert!(report.planar_area.encloses(0.75));
    assert_eq!(report.coverage.euler_characteristic, 0);
    assert!(report.validation.is_valid());
    assert_eq!(
        report
            .validation
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact
    );
    let coverage = result.actual_self_intersection_coverage().unwrap();
    assert_eq!(coverage.cap_self_domains, 2);
    assert_eq!(coverage.side_self_domains, report.coverage.faces - 2);
}

fn check_authored_volume(source: &WrappedConeNormalRequest, result: &WrappedConeNormalResult) {
    let area = 0.75;
    let [first, last] = source.depths;
    let (sine, cosine) = sincos(source.support.semi_angle);
    let offset = 0.5 * (last * last - first * first) * cosine / sine;
    let correction = match source.support.normal {
        NormalSense::Outward => offset,
        NormalSense::Inward => -offset,
    };
    // The Jacobian in the developed plane is 1 + depth*cot(alpha)/slant.
    let base = area * (last - first);
    let endpoints =
        [hypot(80., 32.5), hypot(81., 33.5)].map(|slant| base + area * correction / slant);
    let volume = result.reports()[&SurfaceRegionId(1)].authored_volume;
    assert!(volume.lower >= endpoints[0].min(endpoints[1]));
    assert!(volume.upper <= endpoints[0].max(endpoints[1]));
}

pub(super) fn check_depth_vertices(
    source: &WrappedConeNormalRequest,
    result: &WrappedConeNormalResult,
) {
    let detached = result.export_native().unwrap();
    let mut checked = 0;
    for (key, shape) in &detached.entities {
        let (WrappedEntitySource::Vertex(id), Some(depth)) =
            (key.source, physical_depth(source, key.role))
        else {
            continue;
        };
        let vertex = source
            .profile
            .vertices
            .iter()
            .find(|p| p.id.0 == id.0)
            .unwrap();
        let expected =
            super::super::inward::expected_point(vertex.point, depth, source.support.semi_angle);
        let point = BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
        assert!(point.distance(&Point3::new(expected[0], expected[1], expected[2])) <= 1e-4);
        checked += 1;
    }
    assert_eq!(checked, source.profile.vertices.len() * 2);
}

fn physical_depth(source: &WrappedConeNormalRequest, role: HistoryRole) -> Option<f64> {
    let depth = match role {
        HistoryRole::FirstDepthVertex => source.depths[0],
        HistoryRole::LastDepthVertex => source.depths[1],
        _ => return None,
    };
    Some(match source.support.normal {
        NormalSense::Outward => depth,
        NormalSense::Inward => -depth,
    })
}

pub(super) fn crossing_request() -> WrappedConeNormalRequest {
    let mut source = request();
    source.support.semi_angle = 0.125;
    source.profile.curves[2] = ProfileCurve::Line {
        id: ProfileCurveId(3),
        start: ProfileVertexId(3),
        end: ProfileVertexId(4),
    };
    for vertex in &mut source.profile.vertices {
        vertex.point[0] += 76.;
        vertex.point[1] += 32.5;
    }
    source.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    source
}

fn complete(
    source: &WrappedConeNormalRequest,
    selected: WrappedConeExecutionSelection,
) -> WrappedConeNormalResult {
    normal_extrude_cone_profile_with_selection(source, selected)
        .unwrap_or_else(|cause| panic!("recentered cone refused: {cause:?}"))
}
