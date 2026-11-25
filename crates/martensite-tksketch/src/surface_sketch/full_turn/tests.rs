// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
mod cone_aperture;
mod conic_aperture;
pub(super) mod curved_aperture;
mod curved_batch;
mod curved_cone_aperture;
mod multiple;
mod placed;
mod rational_aperture;
mod repeated_crossing;

use std::f64::consts::{FRAC_PI_6, PI, TAU};

static_assertions::assert_impl_all!(FullTurnBand: Send, Sync);
static_assertions::assert_impl_all!(CylinderBandHole: Send, Sync);
static_assertions::assert_impl_all!(ConeBandHole: Send, Sync);
static_assertions::assert_impl_all!(CrossedBandResult: Send, Sync);
static_assertions::assert_impl_all!(PeriodicImageKey: Send, Sync);

pub(super) fn options() -> NormalExtrusionOptions {
    NormalExtrusionOptions {
        resolution: Resolution::new(
            1e-6,
            1e-7,
            1e-9,
            DisplayTolerance {
                chordal: 1e-3,
                angular: 0.1,
            },
            ToleranceCap::Refuse(1e-5),
        ),
        volume_relative_error: 1e-9,
    }
}

fn cylinder(normal: NormalSense, seam: f64) -> FullTurnBand {
    FullTurnBand {
        support: FullTurnSupport::Cylinder {
            support: CylinderSupport {
                origin: [0.; 3],
                axis: [0., 0., 1.],
                radial: [1., 0., 0.],
                radius: 5.,
                chart_origin: seam,
                normal,
            },
            axial: [1., 4.],
        },
        region: SurfaceRegionId(103),
        rims: [SurfaceCurveId(41), SurfaceCurveId(3)],
    }
}

fn cone(normal: NormalSense, seam: f64) -> FullTurnBand {
    FullTurnBand {
        support: FullTurnSupport::Cone {
            support: ConeSupport {
                apex: [0.; 3],
                axis: [0., 0., 1.],
                radial: [1., 0., 0.],
                semi_angle: FRAC_PI_6,
                chart_origin: seam,
                normal,
            },
            slant: [4., 6.],
        },
        ..cylinder(normal, seam)
    }
}

#[test]
fn cylinder_band_is_one_solid_with_the_annular_volume_in_both_normal_senses() {
    for (sense, expected) in [
        (NormalSense::Outward, 99. * PI),
        (NormalSense::Inward, 81. * PI),
    ] {
        let band = cylinder(sense, 0.);
        let result = normal_extrude_full_turn_band(&band, [-1., 2.], options()).unwrap();
        check_analytic_band(&result, expected, [4, 6, 4, 6, 1]);
        assert_authored_images(&band, &result);
    }
}

#[test]
fn conical_band_keeps_the_analytic_jacobian_and_a_one_period_boundary() {
    for (sense, sign) in [(NormalSense::Outward, 1.), (NormalSense::Inward, -1.)] {
        let band = cone(sense, 0.);
        let result = normal_extrude_full_turn_band(&band, [-0.25, 0.5], options()).unwrap();
        let expected = 3. * PI * (2.5 + sign * 3_f64.sqrt() / 16.);
        check_analytic_band(&result, expected, [4, 8, 4, 4, 1]);
        assert_authored_images(&band, &result);
    }
}

fn check_analytic_band(result: &NormalExtrusionResult, volume: f64, counts: [usize; 5]) {
    let actual = result.volume();
    assert!(
        (actual.value - volume).abs() <= actual.error_bound,
        "{actual:?} expected {volume}"
    );
    let [census] = result.validation().census.as_slice() else {
        panic!("expected one solid, got {:?}", result.validation().census);
    };
    assert_eq!(
        counts,
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ]
    );
    assert_eq!(census.genus, Some(1));
    assert!(result.validation().is_valid());
}

fn assert_authored_images(band: &FullTurnBand, result: &NormalExtrusionResult) {
    for rim in band.rims {
        for role in [
            HistoryRole::SideFace,
            HistoryRole::FirstDepthEdge,
            HistoryRole::LastDepthEdge,
        ] {
            let key = HistoryKey {
                source: HistorySource::Curve(rim),
                role,
            };
            assert!(result.history().binary_search(&key).is_ok(), "{key:?}");
        }
    }
    assert_eq!(
        result
            .history()
            .iter()
            .filter(|key| key.role == HistoryRole::SideFace)
            .count(),
        2
    );
    assert!(
        result
            .export_brep()
            .unwrap()
            .entities
            .iter()
            .all(|(_, image)| !image.is_null())
    );
}

#[test]
fn seam_relocation_preserves_authored_keys_and_byte_replay_keeps_detached_owners() {
    let band = cylinder(NormalSense::Outward, 0.4);
    let request = || normal_extrude_full_turn_band(&band, [-1., 2.], options()).unwrap();
    let first = request();
    let second = std::thread::scope(|scope| scope.spawn(request).join().unwrap());
    assert_eq!(first.bytes, second.bytes);
    assert_eq!(first.history(), second.history());
    let first_export = first.export_brep().unwrap();
    let second_export = first.export_brep().unwrap();
    assert!(!first_export.solid.is_partner(&second_export.solid));
    let moved =
        normal_extrude_full_turn_band(&cylinder(NormalSense::Outward, -0.6), [-1., 2.], options())
            .unwrap();
    assert_eq!(first.history(), moved.history());
    check_analytic_band(&moved, 99. * PI, [4, 6, 4, 6, 1]);
}

#[test]
fn full_turn_refuses_ambiguous_inputs_without_admitting_near_closed_sectors() {
    let band = cylinder(NormalSense::Outward, 0.);
    let duplicate = FullTurnBand {
        rims: [band.rims[0]; 2],
        ..band
    };
    let refused = normal_extrude_full_turn_band(&duplicate, [-1., 2.], options()).unwrap_err();
    assert!(matches!(
        refused.cause,
        SurfaceFailureCause::DuplicateIdentity {
            kind: "rim",
            value: 41
        }
    ));
    let refused = normal_extrude_full_turn_band(&band, [-5., 2.], options()).unwrap_err();
    assert!(matches!(
        refused.cause,
        SurfaceFailureCause::InvalidInput {
            field: "offset radius",
            value: 0.,
            ..
        }
    ));
    assert_partial_refusals(&band);
}

fn assert_partial_refusals(band: &FullTurnBand) {
    let FullTurnSupport::Cylinder { support, axial } = band.support else {
        panic!("expected cylinder")
    };
    for width in [TAU - 0.5e-9, TAU, TAU + 0.5e-9] {
        let rectangle = CylinderRectangle {
            support,
            region: band.region,
            s: [0., width * support.radius],
            z: axial,
            corners: [1, 2, 3, 4].map(SurfaceVertexId),
            boundaries: [1, 2, 3, 4].map(SurfaceCurveId),
        };
        let refused = normal_extrude(&rectangle, [-1., 2.], options()).unwrap_err();
        assert_eq!(refused.stage, SurfaceStage::Admission);
    }
}

#[test]
fn oblique_full_turn_retains_analytic_shape_and_trim_correspondence() {
    let mut band = cylinder(NormalSense::Outward, 0.3);
    if let FullTurnSupport::Cylinder { support, .. } = &mut band.support {
        support.origin = [2., -4., 8.];
        support.axis = [0.6, 0., 0.8];
        support.radial = [0.8, 0., -0.6];
    }
    let result = normal_extrude_full_turn_band(&band, [-1., 2.], options()).unwrap();
    check_analytic_band(&result, 99. * PI, [4, 6, 4, 6, 1]);
    assert_authored_images(&band, &result);
}

#[test]
fn moved_seam_binding_refuses_before_accepting_an_unchanged_valid_solid() {
    let band = cylinder(NormalSense::Outward, 0.3);
    let section = admission::admit(band, [-1., 2.], options()).unwrap();
    let (meridian, mut maker, solid) = construction::construct(&section.sweep).unwrap();
    let mut bound =
        crate::surface_sketch::history::collect(&section.sweep, &meridian, &mut maker, &solid)
            .unwrap();
    swap_seam_images(&mut bound, band.region);
    assert!(verification::validate(&section, band.rims, &bound).is_err());
    assert!(
        martensite_tkbo::body_check::validate_body(&solid, &options().resolution)
            .unwrap()
            .is_valid()
    );
}

fn swap_seam_images(
    bindings: &mut crate::surface_sketch::history::Bindings,
    region: SurfaceRegionId,
) {
    let source = HistorySource::Region(region);
    let [first, last] = [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge].map(|role| {
        bindings
            .iter()
            .position(|(key, _)| *key == HistoryKey { source, role })
            .unwrap()
    });
    let other = bindings[first].1.clone();
    bindings[first].1 = bindings[last].1.clone();
    bindings[last].1 = other;
}

#[test]
fn full_turn_archives_match_across_fresh_processes() {
    const CHILD: &str = "MARTENSITE_FULL_TURN_REPLAY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let request = cone(NormalSense::Outward, 0.3);
        let result = normal_extrude_full_turn_band(&request, [-0.25, 0.5], options()).unwrap();
        println!(
            "FULL_TURN={:x?}|{:?}|{:?}",
            result.bytes,
            result.history(),
            result.validation()
        );
        return;
    }
    let test = "surface_sketch::full_turn::tests::full_turn_archives_match_across_fresh_processes";
    assert_eq!(fresh_archive(CHILD, test), fresh_archive(CHILD, test));
}

fn fresh_archive(child: &str, test: &str) -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test, "--nocapture"])
        .env(child, "1")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .lines()
        .find(|line| line.starts_with("FULL_TURN="))
        .unwrap()
        .to_owned()
}

fn seam_crossing_hole() -> CylinderBandHole {
    CylinderBandHole {
        arc_length: [-0.625, 0.625],
        axial: [2., 3.],
        corners: [31, 22, 15, 7].map(SurfaceVertexId),
        boundaries: [83, 61, 42, 17].map(SurfaceCurveId),
    }
}

#[test]
fn seam_crossing_aperture_preserves_its_authored_ids_after_native_cut_relocation() {
    let band = cylinder(NormalSense::Outward, 0.);
    let hole = seam_crossing_hole();
    let depths = [-1., 2.];
    let original = normal_extrude_cylinder_band_with_hole(&band, &hole, depths, options()).unwrap();
    let repeated = normal_extrude_cylinder_band_with_hole(&band, &hole, depths, options()).unwrap();
    assert_eq!(original.bytes, repeated.bytes);
    let relocated = super::hole::build(&band, &hole, depths, options(), Some(2.5)).unwrap();
    assert_eq!(original.history(), relocated.history());
    assert_aperture(&original, 99. * PI - 33. / 8.);
    assert_aperture(&relocated, 99. * PI - 33. / 8.);
    for curve in hole.boundaries {
        let wall = HistoryKey {
            source: HistorySource::Curve(curve),
            role: HistoryRole::SideFace,
        };
        assert!(original.history().binary_search(&wall).is_ok());
    }
}

fn assert_aperture(result: &NormalExtrusionResult, expected: f64) {
    let volume = result.volume();
    assert!((volume.value - expected).abs() <= volume.error_bound);
    let [census] = result.validation().census.as_slice() else {
        panic!("one solid expected, got {:?}", result.validation().census);
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        [12, 18, 8, 12, 1]
    );
    assert_eq!(census.genus, Some(2));
}

#[test]
fn inward_normal_preserves_the_through_hole_and_reverses_requested_depth_roles() {
    let band = cylinder(NormalSense::Inward, 0.);
    let hole = seam_crossing_hole();
    let result =
        normal_extrude_cylinder_band_with_hole(&band, &hole, [-1., 2.], options()).unwrap();
    assert_aperture(&result, 81. * PI - 27. / 8.);
    assert!(result.validation().is_valid());
}

#[test]
fn authored_hole_cannot_erase_a_band_rim_or_force_an_unqualified_native_cut() {
    let band = cylinder(NormalSense::Outward, 0.);
    let hole = seam_crossing_hole();
    let bad = CylinderBandHole {
        axial: [1., 2.],
        ..hole
    };
    let refused =
        normal_extrude_cylinder_band_with_hole(&band, &bad, [-1., 2.], options()).unwrap_err();
    assert!(matches!(
        refused.cause,
        SurfaceFailureCause::InvalidInput {
            field: "lower aperture clearance",
            ..
        }
    ));
    let refused = super::hole::build(&band, &hole, [-1., 2.], options(), Some(0.)).unwrap_err();
    assert!(matches!(
        refused.cause,
        SurfaceFailureCause::InvalidInput {
            field: "aperture to native seam clearance",
            ..
        }
    ));
}

#[test]
fn actual_native_seam_cut_publishes_two_lifts_with_signed_source_addresses() {
    let band = cylinder(NormalSense::Outward, 0.);
    let hole = seam_crossing_hole();
    let run = || {
        normal_extrude_cylinder_band_at_authored_seam(&band, &hole, [-1., 2.], options()).unwrap()
    };
    let first = run();
    let second = std::thread::scope(|scope| scope.spawn(run).join().unwrap());
    let geometry = first.geometry();
    assert_eq!(geometry.bytes, second.geometry().bytes);
    assert_eq!(geometry.history(), second.geometry().history());
    assert_eq!(
        first
            .crossings()
            .iter()
            .map(|crossing| crossing.id.transition)
            .collect::<Vec<_>>(),
        [SheetTransition::Increasing, SheetTransition::Decreasing,]
    );
    assert_crossed(&first, 99. * PI - 33. / 8., [0.5, 0.5]);
}

#[test]
fn asymmetric_crossing_preserves_the_entire_oriented_source_interval() {
    let band = cylinder(NormalSense::Inward, 0.);
    let hole = CylinderBandHole {
        arc_length: [-0.25, 0.75],
        ..seam_crossing_hole()
    };
    let result =
        normal_extrude_cylinder_band_at_authored_seam(&band, &hole, [-1., 2.], options()).unwrap();
    assert_crossed(&result, 81. * PI - 2.7, [0.25, 0.75]);
}

#[test]
fn rotated_crossing_keeps_non_dyadic_source_roots() {
    let band = cylinder(NormalSense::Outward, 0.2);
    let hole = CylinderBandHole {
        arc_length: [-1., 2.],
        ..seam_crossing_hole()
    };
    let result =
        normal_extrude_cylinder_band_at_authored_seam(&band, &hole, [-1., 2.], options()).unwrap();
    assert_crossed(&result, 99. * PI - 9.9, [1. / 3., 2. / 3.]);
}

#[test]
fn touching_or_disjoint_aperture_cannot_claim_a_transverse_crossing() {
    let band = cylinder(NormalSense::Outward, 0.);
    for interval in [[-0.625, 0.], [0.1, 1.]] {
        let hole = CylinderBandHole {
            arc_length: interval,
            ..seam_crossing_hole()
        };
        let error =
            normal_extrude_cylinder_band_at_authored_seam(&band, &hole, [-1., 2.], options())
                .unwrap_err();
        assert!(matches!(
            error.cause,
            SurfaceFailureCause::InvalidInput {
                field: "authored seam crossing clearance",
                ..
            }
        ));
    }
}

fn assert_crossed(result: &CrossedBandResult, expected: f64, ratios: [f64; 2]) {
    let geometry = result.geometry();
    let volume = geometry.volume();
    assert!((volume.value - expected).abs() <= volume.error_bound);
    let [census] = geometry.validation().census.as_slice() else {
        panic!("one periodic solid expected");
    };
    assert_eq!(
        [
            census.vertices,
            census.edges,
            census.faces,
            census.loops,
            census.shells
        ],
        [16, 24, 8, 10, 1]
    );
    assert_eq!(census.genus, Some(2));
    for (root, ratio) in result.crossings().iter().zip(ratios) {
        let [a, b] = root.normalized_parameter;
        assert!(a <= ratio && ratio <= b);
        assert_history_interval(geometry.history(), root.id);
    }
}

fn assert_history_interval(keys: &[PeriodicImageKey], id: PeriodicCrossingId) {
    let cut = PeriodicSourceEnd::Cut(id);
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let root = PeriodicImageKey::CrossingVertex {
            depth,
            crossing: id,
        };
        assert!(keys.binary_search(&root).is_ok());
        for interval in [
            [PeriodicSourceEnd::Start, cut],
            [cut, PeriodicSourceEnd::End],
        ] {
            let piece = PeriodicImageKey::AperturePiece {
                curve: id.curve,
                depth,
                interval,
            };
            assert!(keys.binary_search(&piece).is_ok());
        }
    }
}
