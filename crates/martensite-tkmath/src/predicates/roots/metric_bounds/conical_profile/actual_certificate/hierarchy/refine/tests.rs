// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn rational_subpatches_separate_parallel_sides_with_overlapping_whole_boxes() {
    let mut cancel = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut cancel,
    );
    let patches = [0., 0.375].map(|offset| parallel_side(offset, &mut arithmetic));
    assert!(!visit::separated(&patches[0].hull, &patches[1].hull));
    let mut visits = 0;
    assert!(complete_product(patches, &mut visits, &mut arithmetic).unwrap());
    assert!(visits > 1);
}

#[test]
fn overlapping_sides_cannot_be_certified_by_spending_more_split_levels() {
    let mut cancel = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut cancel,
    );
    let patches = [0., 0.].map(|offset| parallel_side(offset, &mut arithmetic));
    let mut visits = 0;
    assert!(!complete_product(patches, &mut visits, &mut arithmetic).unwrap());
    assert_eq!(visits, u64::from(MAX_PRODUCT_DEPTH) + 1);
}

fn parallel_side(offset: f64, arithmetic: &mut Arithmetic<'_>) -> Patch {
    // Each rational row lies on y = x + offset for all admitted positive weights.
    let rows = [0., 1.].map(|z| {
        [[0., offset, z], [1., 1. + offset, z]]
            .map(|p| p.map(|value| arithmetic.scalar(value).unwrap()))
            .to_vec()
    });
    Patch::capture(&rows, &[1., 0.5], arithmetic).unwrap()
}

#[test]
fn a_refining_pair_retains_its_owners_when_work_or_cancellation_refuses() {
    check_refusal(2, false);
    check_refusal(1000, true);
}

fn check_refusal(limit: u32, cancel_in_split: bool) {
    let mut never = || false;
    let mut capture = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut never,
    );
    let patches = [0., 0.375].map(|offset| parallel_side(offset, &mut capture));
    let mut polls = 0;
    let mut cancel = || {
        polls += 1;
        cancel_in_split && polls > 2
    };
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: limit,
        },
        &mut cancel,
    );
    let mut visits = 0;
    let source = complete_product(patches, &mut visits, &mut arithmetic).unwrap_err();
    let sides = [ConicalActualSideId(17), ConicalActualSideId(29)];
    let original = format!("{source:?}");
    let wrapped = visit::pair_failure(sides, source);
    assert_eq!(arithmetic.operations(), 2);
    assert_eq!(visits, 1);
    assert_side_refusal(wrapped, sides, &original, cancel_in_split);
}

fn assert_side_refusal(
    failure: ConicalActualPatchFailure,
    sides: [ConicalActualSideId; 2],
    original: &str,
    cancelled: bool,
) {
    let ConicalActualPatchFailure::SidePair {
        sides: actual,
        cause,
    } = failure
    else {
        panic!("expected the original side pair");
    };
    assert_eq!(actual, sides);
    assert_eq!(format!("{cause:?}"), original);
    assert!(matches!(*cause, ConicalActualPatchFailure::Arithmetic(_)));
    assert_eq!(original.contains("Cancelled"), cancelled);
}
