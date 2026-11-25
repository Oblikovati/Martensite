// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn mixed_cylinder_keeps_a_clear_wire_beside_a_positive_lifted_crossing() {
    let (band, [crossed, clear]) = cylinder_apertures(1.);
    let result = normal_extrude_cylinder_band_holes_at_authored_seam(
        &band,
        &[crossed, clear],
        [-1., 2.],
        options(),
    )
    .unwrap();
    let permuted = normal_extrude_cylinder_band_holes_at_authored_seam(
        &band,
        &[clear, crossed],
        [-1., 2.],
        options(),
    )
    .unwrap();
    assert_eq!(result.geometry().bytes, permuted.geometry().bytes);
    assert_eq!(result.geometry().history(), permuted.geometry().history());
    assert_eq!(result.crossings(), permuted.crossings());
    check_measure(&result, 198. * PI - 11.55, [24, 36, 12, 16, 1], 3);
    check_roots(
        &result,
        1,
        [0.333_333_333_333_334_15, 0.666_666_666_666_665_9],
    );
    check_clear_history(&result, clear.boundaries);
}

#[test]
fn negative_cylindrical_lift_keeps_authored_addresses_and_proves_the_cut_parameter() {
    let (band, [crossed, _]) = cylinder_apertures(-1.);
    let result =
        normal_extrude_cylinder_band_at_authored_seam(&band, &crossed, [-1., 2.], options())
            .unwrap();
    check_measure(&result, 198. * PI - 4.95, [16, 24, 8, 10, 1], 2);
    check_roots(
        &result,
        -1,
        [0.333_333_333_333_332_54, 0.666_666_666_666_667_5],
    );
}

#[test]
fn mixed_cone_keeps_an_inner_loop_and_a_negative_lifted_cut() {
    let mut band = cone(NormalSense::Outward, 0.);
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [4., 10.];
    let crossed = ConeBandHole {
        developed_angle: [-PI - 0.125, -PI + 0.25],
        slant: [5., 6.],
        corners: [31, 22, 15, 7].map(SurfaceVertexId),
        boundaries: [83, 61, 42, 17].map(SurfaceCurveId),
    };
    let clear = ConeBandHole {
        developed_angle: [0.625, 1.],
        slant: [8., 9.],
        corners: [131, 122, 115, 107].map(SurfaceVertexId),
        boundaries: [183, 161, 142, 117].map(SurfaceCurveId),
    };
    let result = normal_extrude_cone_band_holes_at_authored_seam(
        &band,
        &[crossed, clear],
        [-0.25, 0.5],
        options(),
    )
    .unwrap();
    let expected = (9. * PI - 9. / 8.) * (3.5 + 3f64.sqrt() / 16.);
    check_measure(&result, expected, [24, 38, 12, 14, 1], 3);
    check_roots(
        &result,
        -1,
        [0.333_333_333_333_332_2, 0.666_666_666_666_667_7],
    );
    check_clear_history(&result, clear.boundaries);
}

#[test]
fn clear_only_placement_keeps_the_original_unsplit_stock_seam() {
    let (band, [_, clear]) = cylinder_apertures(1.);
    let result =
        normal_extrude_cylinder_band_holes_at_authored_seam(&band, &[clear], [-1., 2.], options())
            .unwrap();
    check_measure(&result, 198. * PI - 6.6, [12, 18, 8, 12, 1], 2);
    assert!(result.crossings().is_empty());
    check_clear_history(&result, clear.boundaries);
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let seam = PeriodicImageKey::SeamSegment {
            region: band.region,
            depth,
            ends: band.rims.map(PeriodicSeamEnd::Rim),
        };
        assert!(result.geometry().history().binary_search(&seam).is_ok());
    }
}

#[test]
fn width_overlap_and_unresolved_lifted_contact_refuse_in_admission() {
    let (band, [sample, _]) = cylinder_apertures(1.);
    for (arc_length, field) in [
        ([4., 4. + 10. * PI + 0.25], "periodic aperture overlap"),
        (
            [10. * PI, 10. * PI + 1.],
            "authored seam crossing clearance",
        ),
    ] {
        let hole = CylinderBandHole {
            arc_length,
            ..sample
        };
        let failure = normal_extrude_cylinder_band_holes_at_authored_seam(
            &band,
            &[hole],
            [-1., 2.],
            options(),
        )
        .unwrap_err();
        assert_eq!(failure.stage, SurfaceStage::Admission);
        assert!(matches!(failure.cause, SurfaceFailureCause::InvalidInput {
            field: role, ..
        } if role == field));
    }
}

#[test]
fn cylinder_cut_near_the_seam_requires_physical_fragment_clearance() {
    let band = cylinder(NormalSense::Outward, 0.);
    let near = CylinderBandHole {
        arc_length: [-5e-8, 1.],
        ..seam_crossing_hole()
    };
    let too_short =
        normal_extrude_cylinder_band_at_authored_seam(&band, &near, [-1., 2.], options())
            .unwrap_err();
    check_fragment_refusal(too_short);
    let resolved = CylinderBandHole {
        arc_length: [-5e-6, 1.],
        ..near
    };
    let result =
        normal_extrude_cylinder_band_at_authored_seam(&band, &resolved, [-1., 2.], options())
            .unwrap();
    assert_eq!(result.crossings().len(), 2);
}

#[test]
fn cone_cut_near_the_seam_requires_clearance_at_its_smallest_offset_rim() {
    let mut band = cone(NormalSense::Outward, 0.);
    let FullTurnSupport::Cone { slant, .. } = &mut band.support else {
        panic!("conical stock");
    };
    *slant = [4., 8.];
    let source = seam_crossing_hole();
    let near = ConeBandHole {
        developed_angle: [-5e-9, 0.125],
        slant: [5., 7.],
        corners: source.corners,
        boundaries: source.boundaries,
    };
    let too_short =
        normal_extrude_cone_band_at_authored_seam(&band, &near, [-0.25, 0.5], options())
            .unwrap_err();
    check_fragment_refusal(too_short);
    let resolved = ConeBandHole {
        developed_angle: [-1e-6, 0.125],
        ..near
    };
    let result =
        normal_extrude_cone_band_at_authored_seam(&band, &resolved, [-0.25, 0.5], options())
            .unwrap();
    assert_eq!(result.crossings().len(), 2);
}

fn check_fragment_refusal(error: SurfaceSketchFailure) {
    assert_eq!(error.stage, SurfaceStage::Admission);
    assert!(matches!(error.cause, SurfaceFailureCause::InvalidInput {
        field: "aperture seam fragment clearance", value, ..
    } if 0. < value && value < 1e-6));
}

fn cylinder_apertures(sign: f64) -> (FullTurnBand, [CylinderBandHole; 2]) {
    let mut band = cylinder(NormalSense::Outward, 0.);
    let FullTurnSupport::Cylinder { axial, .. } = &mut band.support else {
        panic!("cylindrical stock");
    };
    *axial = [0., 6.];
    let shift = sign * 10. * PI;
    let crossed = CylinderBandHole {
        arc_length: [shift - 0.5, shift + 1.],
        axial: [1., 2.],
        ..seam_crossing_hole()
    };
    let clear = CylinderBandHole {
        arc_length: [3., 5.],
        axial: [4., 5.],
        corners: [131, 122, 115, 107].map(SurfaceVertexId),
        boundaries: [183, 161, 142, 117].map(SurfaceCurveId),
    };
    (band, [crossed, clear])
}

fn check_roots(result: &CrossedBandResult, turn: i64, targets: [f64; 2]) {
    // Oracles use exact binary64 authored endpoints and an independent
    // high-precision evaluation of π and sin(semi_angle).
    let [before, after] = result.crossings() else {
        panic!("two crossings required");
    };
    for (root, target) in [before, after].into_iter().zip(targets) {
        assert_eq!(root.id.turn, turn);
        let [lo, hi] = root.normalized_parameter;
        assert!(lo <= target && target <= hi, "{root:?} excludes {target}");
        assert_history_interval(result.geometry().history(), root.id);
    }
    assert_eq!(before.id.transition, SheetTransition::Increasing);
    assert_eq!(after.id.transition, SheetTransition::Decreasing);
}

fn check_measure(result: &CrossedBandResult, expected: f64, counts: [usize; 5], genus: i64) {
    let geometry = result.geometry();
    let volume = geometry.volume();
    assert!(
        (volume.value - expected).abs() <= volume.error_bound,
        "{volume:?}"
    );
    let [census] = geometry.validation().census.as_slice() else {
        panic!("one perforated solid required");
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        counts
    );
    assert_eq!(census.genus, Some(genus));
}

fn check_clear_history(result: &CrossedBandResult, curves: [SurfaceCurveId; 4]) {
    let keys = result.geometry().history();
    for curve in curves {
        for role in [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge] {
            let retained = PeriodicImageKey::Retained(HistoryKey {
                source: HistorySource::Curve(curve),
                role,
            });
            assert!(keys.binary_search(&retained).is_ok());
        }
        assert!(!keys.iter().any(|key| matches!(
            key, PeriodicImageKey::AperturePiece { curve: id, .. } if *id == curve
        )));
    }
}
