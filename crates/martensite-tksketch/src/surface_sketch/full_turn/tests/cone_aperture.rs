// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const DEPTHS: [f64; 2] = [-0.25, 0.5];

fn aperture(normal: NormalSense, seam: f64) -> (FullTurnBand, ConeBandHole) {
    let mut band = cone(normal, seam);
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [4., 8.];
    let hole = seam_crossing_hole();
    (
        band,
        ConeBandHole {
            developed_angle: [-0.125, 0.25],
            slant: [5., 7.],
            corners: hole.corners,
            boundaries: hole.boundaries,
        },
    )
}

fn build(band: &FullTurnBand, hole: &ConeBandHole) -> CrossedBandResult {
    normal_extrude_cone_band_at_authored_seam(band, hole, DEPTHS, options()).unwrap()
}

#[test]
fn conical_aperture_splits_both_depth_cuts_and_keeps_signed_volume() {
    for (normal, sign) in [(NormalSense::Outward, 1.), (NormalSense::Inward, -1.)] {
        let (band, hole) = aperture(normal, 0.);
        let result = build(&band, &hole);
        assert_conical_geometry(&result, sign, [1. / 3., 2. / 3.]);
        for rim in band.rims {
            let seam = PeriodicImageKey::Retained(HistoryKey {
                source: HistorySource::Curve(rim),
                role: HistoryRole::NormalEdge,
            });
            assert!(result.geometry().history().binary_search(&seam).is_ok());
        }
    }
}

#[test]
fn conical_native_seam_relocation_preserves_geometry_and_crossing_identity() {
    let (before, hole) = aperture(NormalSense::Outward, 0.);
    let (after, mut lifted) = aperture(NormalSense::Outward, 0.25);
    lifted.developed_angle = [-0.25, 0.125];
    let a = build(&before, &hole);
    let b = build(&after, &lifted);
    assert_conical_geometry(&a, 1., [1. / 3., 2. / 3.]);
    assert_conical_geometry(&b, 1., [2. / 3., 1. / 3.]);
    assert_eq!(a.geometry().history(), b.geometry().history());
    assert_ne!(a.geometry().bytes, b.geometry().bytes);
    assert_ne!(a.crossings(), b.crossings());
}

#[test]
fn conical_aperture_replays_in_fresh_processes() {
    const CHILD: &str = "MARTENSITE_CONICAL_APERTURE_REPLAY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (band, hole) = aperture(NormalSense::Outward, 0.2);
        let result = build(&band, &hole);
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}",
            result.geometry().bytes,
            result.geometry().history(),
            result.crossings()
        );
        return;
    }
    let test = "surface_sketch::full_turn::tests::cone_aperture::conical_aperture_replays_in_fresh_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

#[test]
fn separated_conical_apertures_can_cover_every_angular_cut() {
    let (mut band, seed) = aperture(NormalSense::Outward, 0.);
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [4., 10.];
    let lower = ConeBandHole {
        developed_angle: [-0.75, 1.625],
        slant: [5., 6.],
        ..seed
    };
    let upper = ConeBandHole {
        developed_angle: [-1.625, 0.75],
        slant: [8., 9.],
        corners: [131, 122, 115, 107].map(SurfaceVertexId),
        boundaries: [183, 161, 142, 117].map(SurfaceCurveId),
    };
    assert!(lower.developed_angle[0] < upper.developed_angle[1]);
    assert!(lower.developed_angle[1] - upper.developed_angle[0] > PI);
    let build = |holes: &[ConeBandHole]| {
        normal_extrude_cone_band_holes_at_authored_seam(&band, holes, DEPTHS, options()).unwrap()
    };
    let a = build(&[lower, upper]);
    let b = build(&[upper, lower]);
    assert_eq!(a.geometry().bytes, b.geometry().bytes);
    assert_eq!(a.geometry().history(), b.geometry().history());
    assert_eq!(a.crossings(), b.crossings());
    assert_cone_batch(&a, band.region);
}

fn assert_cone_batch(result: &CrossedBandResult, region: SurfaceRegionId) {
    let geometry = result.geometry();
    let volume = geometry.volume();
    let expected = (9. * PI - 57. / 8.) * (3.5 + 3f64.sqrt() / 16.);
    assert!(
        (volume.value - expected).abs() <= volume.error_bound,
        "{volume:?}"
    );
    assert_census(result, [28, 44, 12, 12, 1], 3);
    check_roots(result, [6. / 19., 13. / 19., 13. / 19., 6. / 19.]);
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let key = PeriodicImageKey::SeamSegment {
            region,
            depth,
            ends: [1, 2].map(|i| PeriodicSeamEnd::Crossing(result.crossings()[i].id)),
        };
        assert!(geometry.history().binary_search(&key).is_ok());
    }
}

#[test]
fn translated_oblique_conical_support_keeps_its_periodic_aperture() {
    let (mut band, hole) = aperture(NormalSense::Outward, 0.3);
    let FullTurnSupport::Cone { support, .. } = &mut band.support else {
        panic!("conical stock");
    };
    support.apex = [2., -4., 8.];
    support.axis = [0.6, 0., 0.8];
    support.radial = [0.8, 0., -0.6];
    let result = build(&band, &hole);
    assert_conical_geometry(&result, 1., [1. / 3., 2. / 3.]);
}

#[test]
fn a_conical_aperture_must_cross_transversely_and_clear_both_rims() {
    let (band, hole) = aperture(NormalSense::Inward, 0.);
    for (edited, field) in [
        (
            ConeBandHole {
                developed_angle: [0., 0.25],
                ..hole
            },
            "authored seam crossing clearance",
        ),
        (
            ConeBandHole {
                slant: [4., 5.],
                ..hole
            },
            "lower aperture clearance",
        ),
    ] {
        let error = normal_extrude_cone_band_at_authored_seam(&band, &edited, DEPTHS, options())
            .unwrap_err();
        assert_eq!(error.stage, SurfaceStage::Admission);
        assert!(matches!(error.cause, SurfaceFailureCause::InvalidInput {
            field: actual, ..
        } if actual == field));
    }
}

fn assert_conical_geometry(result: &CrossedBandResult, sign: f64, roots: [f64; 2]) {
    let volume = result.geometry().volume();
    let expected = (6. * PI - 9. / 8.) * (3. + sign * 3f64.sqrt() / 16.);
    assert!(volume.value > 0.);
    assert!(
        (volume.value - expected).abs() <= volume.error_bound,
        "{volume:?}"
    );
    assert_census(result, [16, 26, 8, 8, 1], 2);
    check_roots(result, roots);
}

fn assert_census(result: &CrossedBandResult, expected: [usize; 5], genus: i64) {
    let [census] = result.geometry().validation().census.as_slice() else {
        panic!("one conical perforated solid");
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        expected
    );
    assert_eq!(census.genus, Some(genus));
}

fn check_roots<const N: usize>(result: &CrossedBandResult, roots: [f64; N]) {
    assert_eq!(result.crossings().len(), N);
    for pair in result.crossings().as_chunks::<2>().0 {
        assert_eq!(pair[0].id.transition, SheetTransition::Increasing);
        assert_eq!(pair[1].id.transition, SheetTransition::Decreasing);
    }
    for (root, target) in result.crossings().iter().zip(roots) {
        let [a, b] = root.normalized_parameter;
        assert!(a <= target && target <= b, "{root:?} excludes {target}");
        assert_history_interval(result.geometry().history(), root.id);
    }
}
