// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkbo::profile_extrusion::{PlanarProofVersion, ProfileCurve};

#[test]
fn curved_cone_aperture_uses_both_signed_developed_rays() {
    for (turn, parameters) in [
        (1, [0.571_428_571_428_572_7, 0.428_571_428_571_427_9]),
        (-1, [0.571_428_571_428_571_4, 0.428_571_428_571_428_3]),
    ] {
        let (band, request) = lifted_fixture(turn);
        let result = normal_extrude_cone_band_with_curved_hole(&band, &request, options())
            .unwrap_or_else(|cause| panic!("{cause:?}"));
        check_lift(&result, turn, parameters);
    }
}

#[test]
fn lifted_cone_weighted_aperture_keeps_original_rational_parameter() {
    let (band, mut request) = lifted_fixture(1);
    compact_rational_controls(&mut request);
    let result = normal_extrude_cone_band_with_rational_hole(&band, &request, options(), 3)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    assert_eq!(
        result.tool().cone_execution_selection(),
        Some(
            WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 {
                arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
            }
        )
    );
    let root = result.publication().crossings()[0];
    assert_eq!(root.id.turn, 1);
    let [lo, hi] = root.normalized_parameter;
    let parameter = 0.474_809_633_632_684_1;
    assert!(lo <= parameter && parameter <= hi, "{root:?}");
    let area = (105. / 128. + 7. * PI / 256.) / 64.;
    let volume = result.publication().geometry().volume();
    let expected = stock_volume() - 0.25 * area;
    assert!(
        (volume.value - expected).abs()
            <= volume.error_bound + result.tool().volume_discrepancy().upper
    );
    assert!(result.trace_bound() <= 1e-7);
}

fn compact_rational_controls(request: &mut WrappedConeNormalRequest) {
    let center = [14.783_637_142_285_01, 6.119_156_220_039_747];
    let compact = |point: [f64; 2]| std::array::from_fn(|i| 0.5 * point[i] + 0.5 * center[i]);
    for vertex in &mut request.profile.vertices {
        vertex.point = compact(vertex.point);
    }
    if let ProfileCurve::Spline { poles, weights, .. } = &mut request.profile.curves[0] {
        for pole in poles {
            *pole = compact(*pole);
        }
        *weights = Some(vec![1., 1., 2.]);
    }
}

#[test]
fn lifted_cone_body_and_source_roots_match_across_processes() {
    const CHILD: &str = "MARTENSITE_LIFTED_CONE_PERIODIC_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (band, request) = lifted_fixture(-1);
        let result = normal_extrude_cone_band_with_curved_hole(&band, &request, options()).unwrap();
        let cut = result.publication();
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}",
            cut.geometry().bytes,
            cut.geometry().history(),
            cut.crossings()
        );
        return;
    }
    let test = "surface_sketch::full_turn::tests::lifted_cone::lifted_cone_body_and_source_roots_match_across_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

#[test]
fn curved_cone_refuses_uncertain_contact_with_a_nonzero_cut_ray() {
    let (band, mut request) = lifted_fixture(1);
    let [a, b] = [
        request.profile.vertices[0].point,
        request.profile.vertices[1].point,
    ];
    let point = std::array::from_fn(|i| (3. * a[i] + 4. * b[i]) / 7.);
    request.profile.vertices[0].point = point;
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        poles[0] = point;
    }
    let failure =
        normal_extrude_cone_band_with_curved_hole(&band, &request, options()).unwrap_err();
    assert_eq!(failure.stage, SurfaceStage::Admission);
    assert!(matches!(
        failure.cause,
        SurfaceFailureCause::Incomplete {
            context: "transverse crossing clear of authored corners",
            ..
        }
    ));
}

fn lifted_fixture(turn: i64) -> (FullTurnBand, WrappedConeNormalRequest) {
    let (mut band, mut request) = curved_cone_aperture::fixture();
    let FullTurnSupport::Cone { support, slant } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [12., 24.];
    support.semi_angle = 1. / 16.;
    request.support = *support;
    request.depths = [-0.125, 0.125];
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    lifted_controls(&mut request, turn);
    (band, request)
}

fn lifted_controls(request: &mut WrappedConeNormalRequest, turn: i64) {
    // The rounded controls are fixed inputs. The ray and root oracles use the real
    // phase ±2π sin(1/16) evaluated independently against these literal coordinates.
    let [a, b, c, d, p] = if turn == 1 {
        [
            [14.759_734_188_300_48, 6.176_904_802_626_797],
            [14.801_564_357_773_406, 6.075_844_783_099_458],
            [15.263_553_018_469_814, 6.267_068_414_975_701],
            [15.221_722_848_996_885, 6.368_128_434_503_039],
            [14.722_900_690_449_892, 6.102_471_838_878_598],
        ]
    } else {
        [
            [14.807_540_096_269_54, -6.061_407_637_452_696],
            [14.765_709_926_796_612, -6.162_467_656_980_035],
            [15.227_698_587_493_018, -6.353_691_288_856_277],
            [15.269_528_756_965_947, -6.252_631_269_328_938],
            [14.728_876_428_946_025, -6.088_034_693_231_835],
        ]
    };
    for (vertex, point) in request.profile.vertices.iter_mut().zip([a, b, c, d]) {
        vertex.point = point;
    }
    if let ProfileCurve::Spline { poles, .. } = &mut request.profile.curves[0] {
        *poles = vec![a, p, b];
    }
}

fn check_lift(result: &CurvedBandResult<WrappedConeMemberReport>, turn: i64, parameters: [f64; 2]) {
    assert_eq!(
        result
            .tool()
            .validation
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact
    );
    let publication = result.publication();
    let roots = publication.crossings();
    assert_eq!(roots.len(), 2);
    for (root, expected) in roots.iter().zip(parameters) {
        assert_eq!(root.id.turn, turn);
        assert_eq!(root.id.ordinal, 0);
        let [lo, hi] = root.normalized_parameter;
        assert!(lo <= expected && expected <= hi, "{root:?}");
    }
    let measured = publication.geometry().volume();
    let analytic = stock_volume() - 175. / 12_288.;
    assert!(
        (measured.value - analytic).abs()
            <= measured.error_bound + result.tool().volume.discrepancy.upper
    );
    assert_eq!(publication.geometry().validation().census[0].genus, Some(2));
    assert!(result.trace_bound() > 0. && result.trace_bound() <= 1e-7);
}

fn stock_volume() -> f64 {
    108. * PI * 0.062_459_317_842_380_2
}
