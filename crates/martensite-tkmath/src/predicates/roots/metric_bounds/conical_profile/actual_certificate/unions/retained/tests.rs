// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn actual_control_steps_and_ruling_are_bounded_in_the_selected_orientation() {
    let mut stop = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut stop,
    );
    let axis = point([1., 1., 0.], &mut arithmetic);
    let first = [[0., 0., 0.], [2., -1., 0.], [1., 2., 0.]];
    let last = [[0., 0., 1.], [2.25, -1., 1.], [1.5, 2., 1.]];
    let rows = [first, last].map(|row| row.map(|p| point(p, &mut arithmetic)).to_vec());
    let minimum = minimum_step(&rows, &axis, true, &mut arithmetic).unwrap();
    assert_eq!(minimum, arithmetic.scalar(1.).unwrap());
    assert!(minimum_step(&rows, &axis, false, &mut arithmetic).is_err());
    let ruling = ruling_bounds(&rows, &axis, &mut arithmetic).unwrap();
    assert_eq!(ruling.lower, arithmetic.zero().lower);
    assert_eq!(ruling.upper, arithmetic.scalar(0.5).unwrap().upper);
}

#[test]
fn common_direction_uses_both_sides_of_the_branch_cut() {
    let mut angles = [-3., 3., -3.1];
    let midpoint = direction::narrowest_arc_midpoint(&mut angles);
    assert!((midpoint - std::f64::consts::PI).abs() < 1e-14);
}

fn point(coordinates: [f64; 3], arithmetic: &mut Arithmetic<'_>) -> Point {
    coordinates.map(|value| arithmetic.scalar(value).unwrap())
}

#[test]
fn weighted_projection_bound_encloses_the_exact_minimum_for_both_orientations() {
    let mut cancel = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut cancel,
    );
    let rows = [[[0., 0., 0.], [2., 0., 0.]], [[0., 0., 1.], [1., 0., 1.]]]
        .map(|row| row.map(|p| point(p, &mut arithmetic)).to_vec());
    for (orientation, forward) in [(1., true), (-1., false)] {
        let axis = point([orientation, 0., 0.], &mut arithmetic);
        let step = minimum_step(&rows, &axis, forward, &mut arithmetic).unwrap();
        let bound = weighted_step(&step, &[0.5, 2.], &mut arithmetic).unwrap();
        // At the end with weight 2 the shorter row's derivative is exactly 1/4.
        assert!(bound.lower > 0.into());
        assert!(bound.upper <= arithmetic.scalar(0.25).unwrap().lower);
        assert_eq!(bound, arithmetic.scalar(0.0625).unwrap());
    }
}
