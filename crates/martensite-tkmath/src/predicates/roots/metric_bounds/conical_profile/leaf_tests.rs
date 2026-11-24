// SPDX-License-Identifier: GPL-2.0-only
//! Closed-form controls exercise the exported non-cap certificate boundary.
use super::*;
use std::cmp::Ordering;

fn request(poles: &[[f64; 2]]) -> ConeRequest<'_> {
    ConeRequest {
        poles,
        semi_angle: 0.5,
        depths: [-0.125, 0.125],
        outward: true,
        clearance: 0.,
        max_depth: 4,
        options: ConeClearanceOptions::default(),
    }
}
#[test]
fn clearance_preserves_source_bits_and_distinguishes_cancellation() {
    let poles = [[3., 0.], [4., 0.]];
    let evidence = certify_cone_request(request(&poles), || false).unwrap();
    assert_eq!(
        evidence.result().source_bits(),
        &poles.map(|point| point.map(f64::to_bits))
    );
    assert_eq!(evidence.result().depths(), [-0.125, 0.125]);
    let error = certify_cone_request(request(&poles), || true).unwrap_err();
    assert!(error.is_cancelled());
    let invalid = [[f64::NAN, 0.], [4., 0.]];
    let error = certify_cone_request(request(&invalid), || false).unwrap_err();
    assert!(!error.is_cancelled());
}
fn contours() -> [ConicalAuthoredContour<'static>; 2] {
    [outer_contour(), hole_contour()]
}
fn outer_contour() -> ConicalAuthoredContour<'static> {
    ConicalAuthoredContour {
        reverse: false,
        spans: &[
            &[[3., 0.], [5., 0.]],
            &[[5., 0.], [5., 0.25]],
            &[[5., 0.25], [3., 0.25]],
            &[[3., 0.25], [3., 0.]],
        ],
    }
}
fn hole_contour() -> ConicalAuthoredContour<'static> {
    ConicalAuthoredContour {
        reverse: false,
        spans: &[
            &[[3.5, 0.0625], [3.5, 0.1875]],
            &[[3.5, 0.1875], [4.5, 0.1875]],
            &[[4.5, 0.1875], [4.5, 0.0625]],
            &[[4.5, 0.0625], [3.5, 0.0625]],
        ],
    }
}
#[test]
fn symmetric_normal_depth_volume_is_holed_developed_area_times_depth() {
    let contours = contours();
    let expected = (2. * 0.25 - 1. * 0.125) * 0.25;
    let query = volume_query(&contours, expected);
    let evidence = certify_conical_authored_flux(query, || false).unwrap();
    let [lower, upper] = evidence.volume();
    assert_ne!(lower.compare_f64(expected).unwrap(), Ordering::Greater);
    assert_ne!(upper.compare_f64(expected).unwrap(), Ordering::Less);
    assert!(evidence.matches(query));
}

fn volume_query<'a>(
    contours: &'a [ConicalAuthoredContour<'a>],
    expected: f64,
) -> ConicalAuthoredFluxQuery<'a> {
    ConicalAuthoredFluxQuery {
        contours,
        semi_angle: 0.5,
        depths: [-0.125, 0.125],
        outward: true,
        level: 4,
        clearance_level: 4,
        volume_width: 1e-6,
        flux: [expected, expected],
        arithmetic: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 1_000_000,
        },
    }
}
