// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::surface_sketch::full_turn::tests::{
    conic_aperture::{arc_profile, native_short_area, short_arc},
    curved_aperture::fixture,
    curved_batch::{add_rectangle, count_cap_wires},
    curved_cone_aperture, options, surface_failure,
};
use martensite_tkbrep::topods::{Builder, Compound};

#[test]
fn clear_analytic_quarter_retains_its_complete_wire_through_public_cylinder_cut() {
    let (band, mut request) = fixture();
    let points = [[1.75, 2.], [2.25, 2.5], [2.125, 3.], [1.75, 3.]];
    arc_profile(&mut request.profile, points, [1.75, 2.5]);
    let result = normal_extrude_cylinder_band_with_rational_hole(&band, &request, options(), 4)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let cut = result.publication();
    let area = (7. / 8. + PI / 4.) / 4.;
    assert_clear_result(&band, cut, &result);
    assert_arc_keys(cut, result.tool().history(), quarter_key);
    assert_volume(
        cut,
        198. * PI - 3.3 * area,
        result.tool().volume_discrepancy().upper,
        2,
    );
}

#[test]
fn clear_analytic_short_arc_retains_its_complete_wire_through_public_cone_cut() {
    let (mut band, mut request) = curved_cone_aperture::fixture();
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("cone fixture");
    };
    *slant = [7., 12.];
    short_arc(&mut request.profile, |[x, y]| {
        [7.5 + y / 32., 0.25 - x / 32.]
    });
    let result = normal_extrude_cone_band_with_rational_hole(&band, &request, options(), 3)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    let cut = result.publication();
    assert_clear_result(&band, cut, &result);
    assert_arc_keys(cut, result.tool().history(), short_arc_key);
    let expected = 95. * PI / 4. - native_short_area() / 2048.;
    assert_volume(cut, expected, result.tool().volume_discrepancy().upper, 2);
}

#[test]
fn mixed_clear_analytic_and_crossed_rectangle_keep_the_complete_actual_wires() {
    let (band, mut clear) = fixture();
    short_arc(&mut clear.profile, |[x, y]| {
        [2. + x / 4., 2. + (y - 2.) / 4.]
    });
    let crossed = crossed_rectangle(&clear);
    let request = mixed_request(&clear, &crossed);
    let plan = count_and_admit_mixed(&band, &request);
    let analytic = normal_extrude_rational_cylinder(&clear, 4, || false).unwrap();
    let rectangle = normal_extrude_profile(&crossed).unwrap();
    let report = &rectangle.reports()[&crossed.selections[0].region];
    let evidence = mixed_evidence(&plan, &analytic, report, clear.support.radius);
    let tool = combine_cutter_exports([
        analytic.export_native().unwrap(),
        rectangle.export_native().unwrap(),
    ]);
    let result = finish(plan, (), evidence, &tool).unwrap();
    let cut = result.publication();
    count_cap_wires(cut, band.region, 2);
    assert_crossed_rectangle_roots(cut);
    assert_arc_keys(cut, analytic.history(), short_arc_key);
    let expected = 198. * PI - 3.3 * (0.75 + native_short_area() / 16.);
    let discrepancy =
        analytic.volume_discrepancy().upper + report.measures.retained_ideal_discrepancy.upper;
    assert_volume(cut, expected, discrepancy, 3);
    assert!(result.trace_bound() > 0. && result.trace_bound() <= 1e-7);
}

fn count_and_admit_mixed(band: &FullTurnBand, request: &WrappedNormalRequest) -> AperturePlan {
    let failure = normal_extrude_cylinder_band_with_rational_hole(band, request, options(), 4)
        .err()
        .unwrap();
    let failure = surface_failure(failure);
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "one selected fitted aperture",
            ..
        }
    ));
    AperturePlan::admit(
        band,
        request,
        options(),
        Scope::Batch,
        SourceModel::Rational(options().resolution),
    )
    .unwrap()
}

fn crossed_rectangle(clear: &WrappedNormalRequest) -> WrappedNormalRequest {
    let mut crossed = clear.clone();
    crossed.profile.curves.clear();
    crossed.profile.vertices.clear();
    let points = [[-0.25, 4.], [0.5, 4.], [0.5, 5.], [-0.25, 5.]];
    crossed.selections = vec![add_rectangle(&mut crossed.profile, points)];
    crossed
}

fn mixed_request(
    clear: &WrappedNormalRequest,
    crossed: &WrappedNormalRequest,
) -> WrappedNormalRequest {
    let mut request = clear.clone();
    request
        .profile
        .curves
        .extend(crossed.profile.curves.clone());
    request
        .profile
        .vertices
        .extend(crossed.profile.vertices.iter().cloned());
    request.selections.extend(crossed.selections.clone());
    request
}

fn mixed_evidence(
    plan: &AperturePlan,
    analytic: &RationalWrappedResult,
    rectangle: &WrappedMemberReport,
    radius: f64,
) -> FitEvidence {
    let rect = FitEvidence::cylinder(plan, rectangle, radius).unwrap();
    let volume = analytic.authored_volume();
    let angle = angle_from_chord(plan.minimum_radius(), analytic.spatial_error().upper).unwrap();
    FitEvidence {
        authored_volume: WrappedCertifiedRange {
            lower: (rect.authored_volume.lower + volume.lower).next_down(),
            upper: (rect.authored_volume.upper + volume.upper).next_up(),
        },
        angular_error: rect.angular_error.max(angle),
    }
}

fn combine_cutter_exports(tools: [DetachedWrappedBodies; 2]) -> DetachedWrappedBodies {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut bodies = compound.into_shape();
    let mut entities = BTreeMap::new();
    for tool in tools {
        Builder.add(&mut bodies, &tool.bodies).unwrap();
        for (key, shape) in tool.entities {
            assert!(entities.insert(key, shape).is_none());
        }
    }
    DetachedWrappedBodies { bodies, entities }
}

fn assert_clear_result(
    band: &FullTurnBand,
    cut: &CrossedBandResult,
    result: &CurvedBandResult<RationalWrappedResult>,
) {
    assert!(cut.crossings().is_empty());
    assert_eq!(result.native_parameter_error(), 0.);
    assert!(result.trace_bound() > 0. && result.trace_bound() <= 1e-7);
    count_cap_wires(cut, band.region, 2);
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let seam = PeriodicImageKey::SeamSegment {
            region: band.region,
            depth,
            ends: band.rims.map(PeriodicSeamEnd::Rim),
        };
        assert!(cut.geometry().history().binary_search(&seam).is_ok());
    }
}

fn assert_crossed_rectangle_roots(cut: &CrossedBandResult) {
    let [first, last] = cut.crossings() else {
        panic!("two actual rectangle crossings");
    };
    for (root, id, t) in [(first, 11, 1. / 3.), (last, 13, 2. / 3.)] {
        assert_eq!(root.id.curve, SurfaceCurveId(id));
        assert!(root.normalized_parameter[0] <= t && t <= root.normalized_parameter[1]);
    }
}

fn assert_arc_keys(
    cut: &CrossedBandResult,
    original: &[WrappedHistoryRecord],
    identify: impl Fn(WrappedEntitySource) -> bool,
) {
    for role in [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge] {
        let mut count = 0;
        for key in original.iter().map(|record| record.result) {
            if key.role == role && identify(key.source) {
                let retained = PeriodicImageKey::WrappedRetained(key);
                assert!(cut.geometry().history().binary_search(&retained).is_ok());
                count += 1;
            }
        }
        assert!(
            count >= 2,
            "expected distinct mapped source spans at {role:?}"
        );
    }
}

fn quarter_key(source: WrappedEntitySource) -> bool {
    matches!(
        source,
        WrappedEntitySource::MappedCurve {
            curve: SurfaceCurveId(1),
            ..
        }
    )
}

fn short_arc_key(source: WrappedEntitySource) -> bool {
    matches!(
        source,
        WrappedEntitySource::AnalyticArcCurve {
            curve: SurfaceCurveId(1),
            ..
        }
    )
}

fn assert_volume(cut: &CrossedBandResult, expected: f64, fit: f64, genus: i64) {
    let geometry = cut.geometry();
    assert_eq!(geometry.validation().census[0].genus, Some(genus));
    let volume = geometry.volume();
    assert!((volume.value - expected).abs() <= volume.error_bound + fit);
}
