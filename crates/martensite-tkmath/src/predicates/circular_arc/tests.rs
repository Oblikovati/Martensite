// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn metrics(x: f64, y: f64) -> ArcTripleMetrics {
    ArcTripleMetrics::new([[-4., 0., 0.], [4., 0., 0.], [x, y, 0.]]).unwrap()
}

#[test]
fn two_directed_branches_share_one_exact_circle_radius() {
    for (height, expected) in [(2., Ordering::Less), (8., Ordering::Greater)] {
        let arc = metrics(0., height);
        assert_eq!(arc.branch(), expected);
        assert_eq!(arc.midpoint_offset(0.), Ok(Ordering::Equal));
        assert_eq!(arc.radius_in_band(5., 0.), Ok(true));
        assert_eq!(arc.radius_in_band(4., 1.), Ok(true));
        assert_eq!(arc.radius_in_band(4., 1.0_f64.next_down()), Ok(false));
    }
    assert_eq!(metrics(0., 4.).branch(), Ordering::Equal);
}

#[test]
fn actual_triangle_radius_remains_independent_of_the_midpoint_chart() {
    let arc = metrics(1., 2.);
    assert_eq!(arc.midpoint_offset(1.), Ok(Ordering::Equal));
    assert_eq!(arc.midpoint_offset(1.0_f64.next_up()), Ok(Ordering::Less));
    assert_eq!(
        arc.midpoint_offset(1.0_f64.next_down()),
        Ok(Ordering::Greater)
    );
    let expected_radius = 23.5625_f64.sqrt();
    assert_eq!(arc.radius_in_band(expected_radius, 1e-14), Ok(true));
    assert_eq!(arc.radius_in_band(5., 0.1), Ok(false));
}

#[test]
fn bad_source_geometry_and_scalar_keep_their_input_identity() {
    assert_eq!(
        ArcTripleMetrics::new([[0.; 3], [1., 0., 0.], [2., 0., 0.]]),
        Err(ArcTripleFailure::Collinear)
    );
    assert!(matches!(
        metrics(0., 2.).radius_in_band(5., f64::INFINITY),
        Err(ArcTripleFailure::NonFinite(NonFiniteCoordinate {
            point: 4,
            axis: 0,
            ..
        }))
    ));
}

#[test]
fn exact_radius_difference_uses_its_own_closed_band_for_opposite_branches() {
    let small = metrics(0., 8.);
    let large = ArcTripleMetrics::new([[-8., 0., 0.], [8., 0., 0.], [0., 4., 0.]]).unwrap();
    for (first, second) in [(&small, &large), (&large, &small)] {
        assert_eq!(
            first.compare_radius_difference(second, 5.0_f64.next_down()),
            Ok(Ordering::Greater)
        );
        assert_eq!(
            first.compare_radius_difference(second, 5.),
            Ok(Ordering::Equal)
        );
        assert_eq!(
            first.compare_radius_difference(second, 5.0_f64.next_up()),
            Ok(Ordering::Less)
        );
    }
    assert_eq!(
        small.compare_radius_difference(&metrics(0., 2.), 0.),
        Ok(Ordering::Equal)
    );
}
