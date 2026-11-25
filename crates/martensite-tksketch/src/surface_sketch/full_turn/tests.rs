// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use std::f64::consts::{FRAC_PI_6, PI, TAU};

fn options() -> NormalExtrusionOptions {
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
