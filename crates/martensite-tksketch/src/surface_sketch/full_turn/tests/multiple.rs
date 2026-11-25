// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const DEPTHS: [f64; 2] = [-1., 2.];
const VOLUME: f64 = 198. * PI - 151.8;

fn apertures(seam: f64) -> (FullTurnBand, [CylinderBandHole; 2]) {
    let mut band = cylinder(NormalSense::Outward, seam);
    let FullTurnSupport::Cylinder { axial, .. } = &mut band.support else {
        panic!("cylindrical stock");
    };
    *axial = [0., 6.];
    let first = CylinderBandHole {
        arc_length: [-7., 16.],
        axial: [1., 2.],
        ..seam_crossing_hole()
    };
    let second = CylinderBandHole {
        arc_length: [-16., 7.],
        axial: [4., 5.],
        corners: [131, 122, 115, 107].map(SurfaceVertexId),
        boundaries: [183, 161, 142, 117].map(SurfaceCurveId),
    };
    (band, [first, second])
}

fn build(band: &FullTurnBand, holes: &[CylinderBandHole]) -> CrossedBandResult {
    normal_extrude_cylinder_band_holes_at_authored_seam(band, holes, DEPTHS, options()).unwrap()
}

#[test]
fn two_seam_crossing_apertures_cover_the_period_without_intersecting() {
    let (band, [first, second]) = apertures(0.);
    assert!(first.arc_length[0] < second.arc_length[1]);
    assert!(first.arc_length[1] - second.arc_length[0] > 5. * TAU);
    let result = build(&band, &[first, second]);
    assert_geometry(&result, VOLUME, [7, 16, 16, 7]);
    assert_middle_segment(&result, band.region);
    let reversed = build(&band, &[second, first]);
    assert_eq!(result.geometry().bytes, reversed.geometry().bytes);
    assert_eq!(result.geometry().history(), reversed.geometry().history());
    assert_eq!(result.crossings(), reversed.crossings());
}

#[test]
fn aperture_batch_replays_in_fresh_processes() {
    const CHILD: &str = "MARTENSITE_PERIODIC_APERTURES_REPLAY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let (band, holes) = apertures(0.);
        let result = build(&band, &holes);
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}",
            result.geometry().bytes,
            result.geometry().history(),
            result.crossings()
        );
        return;
    }
    let test =
        "surface_sketch::full_turn::tests::multiple::aperture_batch_replays_in_fresh_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

#[test]
fn moving_the_actual_seam_preserves_apertures_and_updates_crossing_roots() {
    let (original, holes) = apertures(0.);
    let (rotated, mut lifted) = apertures(0.2);
    // Adjusting both endpoints compensates the native seam rotation on the same cylinder.
    for hole in &mut lifted {
        hole.arc_length = hole.arc_length.map(|s| s - 1.);
    }
    let before = build(&original, &holes);
    let after = build(&rotated, &lifted);
    assert_geometry(&after, VOLUME, [8, 15, 17, 6]);
    assert_middle_segment(&after, rotated.region);
    assert_eq!(before.geometry().history(), after.geometry().history());
    assert_ne!(before.geometry().bytes, after.geometry().bytes);
    assert_ne!(before.crossings(), after.crossings());
}

#[test]
fn changing_aperture_size_changes_volume_with_the_same_authored_piece_addresses() {
    let (band, mut holes) = apertures(0.);
    let before = build(&band, &holes);
    holes[0].arc_length[1] = 16.5;
    let after = build(&band, &holes);
    assert_geometry(&after, VOLUME - 1.65, [14, 33, 16, 7]);
    assert_middle_segment(&after, band.region);
    assert_eq!(before.geometry().history(), after.geometry().history());
    assert_ne!(before.geometry().bytes, after.geometry().bytes);
    assert_ne!(before.crossings(), after.crossings());
}

#[test]
fn aperture_batch_refuses_overlap_touching_and_aliased_authorship() {
    let (band, [first, second]) = apertures(0.);
    for axial in [[1.5, 3.], [2., 3.]] {
        let edited = CylinderBandHole { axial, ..second };
        let error = normal_extrude_cylinder_band_holes_at_authored_seam(
            &band,
            &[first, edited],
            DEPTHS,
            options(),
        )
        .unwrap_err();
        assert_eq!(error.stage, SurfaceStage::Admission);
        assert!(matches!(
            error.cause,
            SurfaceFailureCause::InvalidInput {
                field: "aperture axial separation",
                ..
            }
        ));
    }
    let aliased = CylinderBandHole {
        corners: first.corners,
        ..second
    };
    let error = normal_extrude_cylinder_band_holes_at_authored_seam(
        &band,
        &[first, aliased],
        DEPTHS,
        options(),
    )
    .unwrap_err();
    assert_eq!(error.stage, SurfaceStage::Admission);
    assert!(matches!(
        error.cause,
        SurfaceFailureCause::DuplicateIdentity {
            kind: "aperture corner",
            value: 31,
        }
    ));
}

fn assert_geometry(result: &CrossedBandResult, expected: f64, numerators: [u32; 4]) {
    let geometry = result.geometry();
    let volume = geometry.volume();
    assert!(
        (volume.value - expected).abs() <= volume.error_bound,
        "{volume:?}"
    );
    let [census] = geometry.validation().census.as_slice() else {
        panic!("one perforated periodic solid");
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        [28, 42, 12, 14, 1]
    );
    assert_eq!(census.genus, Some(3));
    assert_eq!(result.crossings().len(), numerators.len());
    for (i, (root, numerator)) in result.crossings().iter().zip(numerators).enumerate() {
        let denominator = if i < 2 {
            numerators[0] + numerators[1]
        } else {
            23
        };
        let ratio = f64::from(numerator) / f64::from(denominator);
        let [a, b] = root.normalized_parameter;
        assert!(a <= ratio && ratio <= b, "{root:?} excludes {ratio}");
        assert_history_interval(geometry.history(), root.id);
    }
}

fn assert_middle_segment(result: &CrossedBandResult, region: SurfaceRegionId) {
    let ends = [
        (42, SheetTransition::Decreasing),
        (183, SheetTransition::Increasing),
    ]
    .map(|(curve, transition)| {
        PeriodicSeamEnd::Crossing(PeriodicCrossingId {
            curve: SurfaceCurveId(curve),
            turn: 0,
            ordinal: 0,
            transition,
        })
    });
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let key = PeriodicImageKey::SeamSegment {
            region,
            depth,
            ends,
        };
        assert!(
            result.geometry().history().binary_search(&key).is_ok(),
            "{key:?}"
        );
    }
}
