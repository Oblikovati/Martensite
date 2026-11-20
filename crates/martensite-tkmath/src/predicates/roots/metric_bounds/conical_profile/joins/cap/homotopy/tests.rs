// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn exclusions_cover_only_adjacent_pairs_in_sorted_contour_order() {
    let expected = [
        [0, 1],
        [0, 3],
        [1, 2],
        [2, 3],
        [4, 5],
        [7, 8],
        [7, 9],
        [8, 9],
    ];
    assert!(adjacencies(&[4, 2, 1, 3]).eq(expected));
}

#[test]
fn motion_equal_to_the_clearance_margin_refuses() {
    let one = DyadicBound::from_f64(1.).unwrap();
    let relation = ConicalCapClearanceRelation::Periodic;
    assert!(matches!(
        separation(relation, &one, &one),
        Err(ConicalCapHomotopyFailure::Gap { .. })
    ));
    let next = DyadicBound::from_f64(1_f64.next_up()).unwrap();
    let gap = separation(relation, &next, &one).unwrap();
    assert_eq!(gap, DyadicBound::from_f64(f64::EPSILON).unwrap());
}
