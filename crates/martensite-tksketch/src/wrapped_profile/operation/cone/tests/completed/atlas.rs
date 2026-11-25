//! One published directed family spans separate lifts while its stored solids stay disjoint.
use super::*;
use crate::surface_sketch::normal_extrude_cone_profile_with_selection;
use martensite_cmath::{cos, sin};
use martensite_tkbo::body_check::{CheckMode, ValidationStage};

const SELECTED: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedRegionAtlasRequireCertifiedV4 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
const ATLAS_WORKER: &str =
    "surface_sketch::wrapped_profile::operation::cone::tests::completed::atlas::process_worker";

#[test]
fn disconnected_cone_regions_publish_across_more_than_one_lifted_revolution() {
    let request = separated_request();
    let common = WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let common_failure = normal_extrude_cone_profile_with_selection(&request, common)
        .err()
        .unwrap();
    assert!(causes(&common_failure).any(|cause| cause.is::<ConicalChartDomainError>()));
    let result = accepted(&request);
    check_result(&request, &result);
    assert_eq!(result_signature(&result), process_signature(ATLAS_WORKER));
}
#[test]
fn disconnected_cone_regions_keep_inward_roles_and_independent_geometry() {
    let mut request = separated_request();
    request.support.normal = NormalSense::Inward;
    let via_policy = crate::surface_sketch::normal_extrude_cone_profile_with_domain_policy(
        &request,
        WrappedConeDomainPolicy::PositiveXRegionAtlasV5,
    )
    .unwrap_or_else(|cause| panic!("inward cone atlas refused: {cause:?}"));
    check_result(&request, &via_policy);
}
#[test]
fn a_curved_letter_and_dot_wrap_on_separate_region_charts() {
    let request = text_request();
    let result = accepted(&request);
    publication::assert_publication_core(&result, &request);
    let reports = result.reports();
    let letter = &reports[&SurfaceRegionId(1)];
    let dot = &reports[&SurfaceRegionId(2)];
    assert!(letter.planar_area.encloses(366. / 20.));
    assert!(dot.planar_area.encloses(4.));
    assert_eq!(letter.coverage.euler_characteristic, 0);
    assert_eq!(dot.coverage.euler_characteristic, 2);
    assert!(
        result
            .actual_self_intersection_coverage()
            .unwrap()
            .side_self_domains
            > 12
    );
    recentered::check_depth_vertices(&request, &result);
}
fn text_request() -> WrappedConeNormalRequest {
    use super::super::narrow::fixture::{Quadrant, request};
    let mut request = request(NormalSense::Outward, Quadrant::PositiveX);
    for vertex in &mut request.profile.vertices {
        let y = &mut vertex.point[1];
        *y = if vertex.id.0 <= 8 {
            *y / 4. - 50.
        } else {
            *y + 27.
        };
    }
    for curve in &mut request.profile.curves {
        if let ProfileCurve::Spline { poles, .. } = curve {
            for point in poles {
                point[1] = point[1] / 4. - 50.;
            }
        }
    }
    request
}
#[test]
fn different_developed_loops_that_coincide_after_one_turn_are_refused() {
    let mut request = shifted(-82.5);
    let mut image = request.clone();
    let turn = std::f64::consts::TAU * sin(request.support.semi_angle);
    let (sine, cosine) = (sin(turn), cos(turn));
    for vertex in &mut image.profile.vertices {
        let [x, y] = vertex.point;
        vertex.point = [x * cosine - y * sine, x * sine + y * cosine];
    }
    append_image(&mut request, image);
    let failure = normal_extrude_cone_profile_with_selection(&request, SELECTED)
        .err()
        .unwrap();
    assert_eq!(
        failure.class(),
        WrappedFailureClass::Incomplete,
        "{failure:?}"
    );
    let pair = causes(&failure)
        .find_map(|cause| cause.downcast_ref::<ConicalActualPatchError>())
        .unwrap_or_else(|| panic!("expected stored family refusal: {failure:?}"));
    assert!(matches!(
        pair.stage,
        ConicalActualPatchStage::Hierarchy | ConicalActualPatchStage::Cap
    ));
    for region in [SurfaceRegionId(1), SurfaceRegionId(2)] {
        assert!(
            failure
                .entities()
                .contains(&crate::surface_sketch::HistorySource::Region(region))
        );
    }
}

fn causes(
    failure: &WrappedNormalFailure,
) -> impl Iterator<Item = &(dyn std::error::Error + 'static)> {
    std::iter::successors(Some(failure as &dyn std::error::Error), |cause| {
        cause.source()
    })
}

#[test]
#[ignore = "subprocess worker invoked by atlas process replay"]
fn process_worker() {
    let result = accepted(&separated_request());
    println!("{PREFIX}{}", result_signature(&result));
}
fn accepted(request: &WrappedConeNormalRequest) -> WrappedConeNormalResult {
    normal_extrude_cone_profile_with_selection(request, SELECTED)
        .unwrap_or_else(|cause| panic!("cone atlas refused: {cause:?}"))
}
fn check_result(request: &WrappedConeNormalRequest, result: &WrappedConeNormalResult) {
    publication::assert_publication_core(result, request);
    assert_eq!(result.execution_selection(), SELECTED);
    assert_eq!(
        result.domain_policy(),
        WrappedConeDomainPolicy::PositiveXRegionAtlasV5
    );
    assert!(result.chart_domain_receipt().is_none());
    let [first, second] = [SurfaceRegionId(1), SurfaceRegionId(2)]
        .map(|region| result.region_chart_domain_receipt(region).unwrap());
    assert_eq!(first.version(), ConicalChartDomainVersion::AtlasMemberV5);
    assert_eq!(second.version(), first.version());
    assert!(second.fit_center() - first.fit_center() > std::f64::consts::TAU);
    assert_eq!(result.reports().len(), 2);
    check_members(result);
    recentered::check_depth_vertices(request, result);
}
fn check_members(result: &WrappedConeNormalResult) {
    for report in result.reports().values() {
        assert!(report.planar_area.encloses(0.75));
        assert_eq!(report.coverage.euler_characteristic, 0);
        assert!(report.validation.is_valid());
        assert_eq!(
            report
                .validation
                .stage(ValidationStage::SelfIntersection)
                .mode,
            CheckMode::Exact,
        );
    }
    let coverage = result.actual_self_intersection_coverage().unwrap();
    assert_eq!(coverage.cap_self_domains, 4);
    assert_eq!(coverage.cap_pairs, 6);
}
pub(in crate::surface_sketch::wrapped_profile) fn separated_request() -> WrappedConeNormalRequest {
    let mut source = shifted(-82.5);
    append_image(&mut source, shifted(16.5));
    source
}
fn shifted(y: f64) -> WrappedConeNormalRequest {
    let mut source = recentered::crossing_request();
    for vertex in &mut source.profile.vertices {
        vertex.point[1] += y;
    }
    source
}
fn append_image(source: &mut WrappedConeNormalRequest, mut image: WrappedConeNormalRequest) {
    for vertex in &mut image.profile.vertices {
        vertex.id.0 += 10;
    }
    for curve in &mut image.profile.curves {
        let ProfileCurve::Line { id, start, end } = curve else {
            panic!("expected line fixture");
        };
        id.0 += 10;
        start.0 += 10;
        end.0 += 10;
    }
    let member = &mut image.selections[0];
    member.region = SurfaceRegionId(2);
    offset_uses(member);
    source.profile.vertices.extend(image.profile.vertices);
    source.profile.curves.extend(image.profile.curves);
    source.selections.extend(image.selections);
}
fn offset_uses(member: &mut WrappedRegionSelection) {
    let contours = std::iter::once(&mut member.outer).chain(&mut member.holes);
    for contour in contours {
        for edge in &mut contour.uses {
            edge.curve.0 += 10;
        }
    }
}
