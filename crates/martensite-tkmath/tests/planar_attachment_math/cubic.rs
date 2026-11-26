// SPDX-License-Identifier: GPL-2.0-only
//! Independent exact whole-span controls in the original square-hole chart.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::{
    PlanarCubicOptions, PlanarCubicProofFailure as Failure, PlanarCurvePath as Path,
    PlanarMonotoneAxis,
};

fn trim() -> CertifiedPlanarAttachment {
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let map = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];
    CertifiedPlanarAttachment::capture(Axis3::default(), map, [&outer, &hole], || false).unwrap()
}

fn controls(minimum: f64) -> [[f64; 2]; 4] {
    [
        [-3., minimum + 1.5],
        [-1., minimum - 0.5],
        [1., minimum - 0.5],
        [3., minimum + 1.5],
    ]
}

fn world(uv: [[f64; 2]; 4]) -> [[f64; 3]; 4] {
    uv.map(|[u, v]| [u, v, 0.])
}

fn policy(distance_limit: f64) -> PlanarCubicOptions {
    PlanarCubicOptions {
        distance_limit,
        max_subdivisions: 16,
    }
}

#[test]
fn exact_half_subdivision_keeps_the_closed_hole_rim_and_reversed_orientation() {
    let chart = trim();
    let mut uv = controls(1.);
    for expected in [
        PlanarMonotoneAxis::IncreasingU,
        PlanarMonotoneAxis::DecreasingU,
    ] {
        let result = chart
            .certify_cubic_curve(world(uv), uv, policy(0.), || false)
            .unwrap();
        assert_eq!(result.subdivisions, [1, 1]);
        assert_eq!(result.monotone, expected);
        assert_eq!(result.distance_bound, 0.);
        uv.reverse();
    }
}

#[test]
fn either_of_the_two_literal_native_paths_can_prove_interior_hole_entry() {
    let chart = trim();
    let good = controls(1.25);
    let bad = controls(0.75);
    for (world, uv, expected) in [
        (world(good), bad, Path::UvLift),
        (world(bad), good, Path::WorldProjection),
    ] {
        assert_eq!(
            chart.certify_cubic_curve(world, uv, policy(1.), || false),
            Err(Failure::Trim {
                path: expected,
                location: PlanarAttachmentLocation::HoleInterior,
                parameter: 0.5
            })
        );
    }
}

#[test]
fn whole_span_error_cap_is_inclusive_and_independent_of_trim_work() {
    let chart = trim();
    let uv = controls(1.25);
    let mut xyz = world(uv);
    xyz[1][2] = 0.125;
    let result = chart
        .certify_cubic_curve(xyz, uv, policy(0.125), || false)
        .unwrap();
    assert_eq!(result.distance_bound, 0.125);
    assert!(matches!(
        chart.certify_cubic_curve(xyz, uv, policy(0.125_f64.next_down()), || false),
        Err(Failure::Attachment(PlanarAttachmentError::OffSupport {
            bound: Some(0.125),
            ..
        }))
    ));
}

#[test]
fn nonmonotone_storage_and_cancellation_remain_distinct_from_trim_failure() {
    let chart = trim();
    let uv = [[-2., 1.5], [2., 3.], [-2., 1.5], [3., 2.5]];
    assert_eq!(
        chart.certify_cubic_curve(world(uv), uv, policy(0.), || false),
        Err(Failure::Monotone)
    );
    let uv = controls(1.25);
    let mut polls = 0;
    assert_eq!(
        chart.certify_cubic_curve(world(uv), uv, policy(0.), || {
            polls += 1;
            polls == 9
        }),
        Err(Failure::Attachment(PlanarAttachmentError::Cancelled))
    );
}
