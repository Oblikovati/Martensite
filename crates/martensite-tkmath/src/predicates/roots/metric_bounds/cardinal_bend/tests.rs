// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn captured_quarter_circle_has_exact_points_and_bidirectional_cut_enclosures() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(CardinalBendPlacement, CardinalBendFailure)>();
    let placed =
        cardinal_bend_placement([60., 0., 0.], [[30., 0., 0.], [60., 30., 0.]], 5.).unwrap();
    let front = 1. / 6_f64;
    let back = 5. / 6_f64;
    for cut in placed.legs {
        assert_eq!(cut.from_corner, [front, front.next_up()]);
        assert_eq!(cut.to_corner, [back.next_down(), back]);
    }
    assert_eq!(
        placed.legs.map(|leg| leg.tangent),
        [[55., 0., 0.], [60., 5., 0.]]
    );
    assert_eq!(placed.center, [55., 5., 0.]);
    assert_eq!(placed.radial_axes, [[0., -1., 0.], [1., 0., 0.]]);
    assert_eq!(placed.normal, [0., 0., 1.]);
    assert_eq!(placed.radius, 5.);
    assert_eq!(placed.position_error_upper, 0.);
    assert_eq!(placed.radius_error_upper, 0.);
    assert_eq!(placed.terminal_error_upper, 5. * f64::EPSILON);
}

#[test]
fn rounding_at_a_wide_corner_keeps_equal_radii_only_and_bounds_center_error() {
    let h = 9_007_199_254_740_992_f64;
    let corner = [h, h, 7.];
    let rays = [[h + 8., h, 7.], [h, h + 8., 7.]];
    let rounded = cardinal_bend_placement(corner, rays, 3.).unwrap();
    assert_eq!(rounded.center, [h + 4., h + 4., 7.]);
    assert_eq!(rounded.radius, 4.);
    assert_eq!(rounded.radius_error_upper, 1.);
    assert_eq!(rounded.position_error_upper, std::f64::consts::SQRT_2);
    assert_eq!(rounded.legs[0].from_corner, [0.5; 2]);
    assert_eq!(
        cardinal_bend_placement(corner, rays, 1.),
        Err(CardinalBendFailure::RoundedLegEnd(0))
    );
    let mixed = [[h + 8., 0., 7.], [h, 8., 7.]];
    assert_eq!(
        cardinal_bend_placement([h, 0., 7.], mixed, 3.),
        Err(CardinalBendFailure::UnequalStoredRadii)
    );
    let short = [[h + 4., h, 7.], rays[1]];
    assert_eq!(
        cardinal_bend_placement(corner, short, 3.),
        Err(CardinalBendFailure::RoundedLegEnd(0))
    );
}

#[test]
fn huge_segment_displacements_are_subtracted_exactly() {
    let m = f64::MAX;
    let placed = cardinal_bend_placement([-m, -m, 0.], [[m, -m, 0.], [-m, m, 0.]], m).unwrap();
    assert_eq!(placed.center, [0.; 3]);
    assert_eq!(placed.legs[0].from_corner, [0.5; 2]);
    assert_eq!(placed.legs[1].to_corner, [0.5; 2]);
    assert_eq!(placed.radius, m);
    assert_eq!(placed.position_error_upper, 0.);
}

#[test]
fn input_scope_and_radius_limit_refuse_independently() {
    let corner = [0.; 3];
    let good = [[4., 0., 0.], [0., 5., 0.]];
    assert!(
        matches!(cardinal_bend_placement(corner, good, f64::NAN), Err(CardinalBendFailure::InvalidRadius(value)) if value.is_nan())
    );
    assert_eq!(
        cardinal_bend_placement(corner, good, 4.),
        Err(CardinalBendFailure::RadiusReachesEnd(0, 4.))
    );
    assert_eq!(
        cardinal_bend_placement(corner, [good[0], [-3., 0., 0.]], 1.),
        Err(CardinalBendFailure::ParallelLegs)
    );
    assert_eq!(
        cardinal_bend_placement(corner, [good[0], [0., 5., f64::from_bits(1)]], 1.),
        Err(CardinalBendFailure::NonCardinalLeg(1))
    );
    assert!(matches!(
        cardinal_bend_placement(corner, [[4., f64::INFINITY, 0.], good[1]], 1.),
        Err(CardinalBendFailure::NonFinite(NonFiniteCoordinate {
            point: 1,
            axis: 1,
            ..
        }))
    ));
}
