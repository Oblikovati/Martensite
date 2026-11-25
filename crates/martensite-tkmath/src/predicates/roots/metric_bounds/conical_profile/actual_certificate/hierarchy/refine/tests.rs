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
    assert!(test_product(patches, &mut visits, &mut arithmetic).unwrap());
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
    assert!(!test_product(patches, &mut visits, &mut arithmetic).unwrap());
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

fn test_product(
    patches: [Patch; 2],
    visits: &mut u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<bool> {
    let [a, b] = patches.map(Arc::new);
    complete_product(
        [PatchUse::root(0, a), PatchUse::root(1, b)],
        visits,
        arithmetic,
        &mut Memo::new(&[]),
    )
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
    let source = test_product(patches, &mut visits, &mut arithmetic).unwrap_err();
    let sides = [ConicalActualSideId(17), ConicalActualSideId(29)];
    let original = format!("{source:?}");
    let wrapped = visit::pair_failure(sides, source);
    assert_eq!(arithmetic.operations(), 2);
    assert_eq!(visits, 1);
    assert_side_refusal(wrapped, sides, &original, cancel_in_split);
}

#[test]
fn reuse_preserves_the_full_product_and_original_ledger() {
    let mut never = || false;
    let mut arithmetic = test_arithmetic(u32::MAX, &mut never);
    let patches = [0., 0.375].map(|offset| Arc::new(parallel_side(offset, &mut arithmetic)));
    let [a, b] = [0, 1].map(|i| PatchUse::root(i, Arc::clone(&patches[i])));
    let run = |memo: &mut Memo<'_>| {
        let mut never = || false;
        let mut arithmetic = test_arithmetic(u32::MAX, &mut never);
        let mut visits = 0;
        let result =
            complete_product([a.clone(), b.clone()], &mut visits, &mut arithmetic, memo).unwrap();
        (result, visits, arithmetic.operations())
    };
    let baseline = run(&mut Memo::without_storage());
    let mut memo = Memo::new(&[]);
    assert!(baseline.0);
    assert_eq!(run(&mut memo), baseline);
    assert_eq!(run(&mut memo), baseline);
}

#[test]
fn reuse_distinguishes_exact_source_and_subdivision_axis() {
    let mut never = || false;
    let mut arithmetic = test_arithmetic(u32::MAX, &mut never);
    let a = PatchUse::root(0, Arc::new(parallel_side(0., &mut arithmetic)));
    let b = PatchUse::root(1, Arc::new(parallel_side(0.375, &mut arithmetic)));
    let mut memo = Memo::new(&[]);
    let [a_left, _] = memo.split(&a, true, &mut arithmetic).unwrap();
    let [reused, _] = memo.split(&a, true, &mut arithmetic).unwrap();
    let [other_source, _] = memo.split(&b, true, &mut arithmetic).unwrap();
    let [other_axis, _] = memo.split(&a, false, &mut arithmetic).unwrap();
    assert!(Arc::ptr_eq(&a_left.patch, &reused.patch));
    assert!(!Arc::ptr_eq(&a_left.patch, &other_source.patch));
    assert!(!Arc::ptr_eq(&a_left.patch, &other_axis.patch));
    let [a_child, _] = memo.split(&a_left, false, &mut arithmetic).unwrap();
    let [axis_child, _] = memo.split(&other_axis, false, &mut arithmetic).unwrap();
    assert!(!Arc::ptr_eq(&a_child.patch, &axis_child.patch));
}

#[test]
fn cached_subdivision_preserves_the_exact_cutoff_and_cancel_position() {
    let mut never = || false;
    let mut arithmetic = test_arithmetic(u32::MAX, &mut never);
    let patch = PatchUse::root(0, Arc::new(parallel_side(0.375, &mut arithmetic)));
    let mut memo = Memo::new(&[]);
    memo.split(&patch, true, &mut arithmetic).unwrap();
    for (limit, cancel_at) in [(3, u32::MAX), (u32::MAX, 4), (3, 4)] {
        let run = |memo: &mut Memo<'_>| {
            let mut polls = 0;
            let mut cancelled = || {
                polls += 1;
                polls == cancel_at
            };
            let mut arithmetic = test_arithmetic(limit, &mut cancelled);
            let result = memo.split(&patch, true, &mut arithmetic).err().unwrap();
            (format!("{result:?}"), arithmetic.operations(), polls)
        };
        assert_eq!(run(&mut memo), run(&mut Memo::without_storage()));
    }
}

fn test_arithmetic(limit: u32, cancelled: &mut (dyn FnMut() -> bool + Send)) -> Arithmetic<'_> {
    Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: limit,
        },
        cancelled,
    )
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
