// SPDX-License-Identifier: GPL-2.0-only
//! Independent rational values test containment, rounding and actual failed work.
use super::*;
fn options(max_operations: u32) -> CylindricalProfileOptions {
    CylindricalProfileOptions {
        fractional_bits: 128,
        max_operations,
        ..Default::default()
    }
}
fn point(n: i64, d: i64) -> ContributionBounds {
    let value = ExactRatio::new(n.into(), d.into());
    ContributionBounds {
        lower: value.clone(),
        upper: value,
    }
}
fn bounds(sum: ConicalSum) -> ContributionBounds {
    sum.finish().into_parts().0
}
#[test]
fn signed_nonbinary_and_distinct_denominators_contain_independent_exact_sum() {
    let mut sum = ConicalSum::new(true, options(1000));
    let mut operations = 0;
    let mut exact = ExactRatio::from_finite(0.);
    for (n, d) in [(1, 3), (-1, 7), (2, 11), (-3, 13), (5, 17)] {
        let contribution = point(n, d);
        exact = exact.add(&contribution.lower);
        sum.accumulate(contribution, 0, &mut operations, &mut || false)
            .unwrap();
    }
    let result = bounds(sum);
    assert!(result.lower <= exact && result.upper >= exact);
    assert_eq!(operations, 35);
}
#[test]
fn large_exact_cancellation_does_not_relax_the_final_cap() {
    let huge = ExactRatio::new(num_bigint::BigInt::from(1) << 1000, 1.into());
    let mut sum = ConicalSum::new(true, options(100));
    let mut operations = 0;
    for value in [huge.clone(), huge.negated(), ExactRatio::from_finite(0.5)] {
        sum.accumulate(
            ContributionBounds {
                lower: value.clone(),
                upper: value,
            },
            0,
            &mut operations,
            &mut || false,
        )
        .unwrap();
    }
    let result = bounds(sum);
    assert_eq!(result.lower, ExactRatio::from_finite(0.5));
    assert_eq!(result.upper, result.lower);
}
#[test]
fn exhausted_add_reports_actual_import_work_and_examined_cells() {
    let mut sum = ConicalSum::new(true, options(6));
    let mut operations = 0;
    let failure = sum
        .accumulate(point(1, 3), 2, &mut operations, &mut || false)
        .unwrap_err();
    assert!(matches!(
        failure,
        BoundedArithmeticFailure::WorkLimit {
            completed: 6,
            limit: 6
        }
    ));
    assert_eq!(operations, 6);
    assert_eq!(sum.finish().into_parts().1, 2);
}
#[test]
fn bit_refusal_keeps_the_actual_offending_integer_size() {
    let mut sum = ConicalSum::new(true, options(100));
    let mut operations = 0;
    let value = ExactRatio::new(num_bigint::BigInt::from(1) << 16385, 1.into());
    let failure = sum
        .accumulate(
            ContributionBounds {
                lower: value.clone(),
                upper: value,
            },
            0,
            &mut operations,
            &mut || false,
        )
        .unwrap_err();
    assert!(
        matches!(failure,BoundedArithmeticFailure::ArithmeticBits{actual,limit:16384} if actual>16384)
    );
    assert_eq!(operations, 1);
}
#[test]
fn v1_remains_exact_unpolled_and_does_not_spend_bounded_work() {
    let mut sum = ConicalSum::new(false, options(0));
    let mut operations = 0;
    sum.accumulate(point(1, 3), 4, &mut operations, &mut || {
        panic!("V1 accumulation has no callback")
    })
    .unwrap();
    sum.accumulate(point(-1, 3), 2, &mut operations, &mut || {
        panic!("V1 accumulation has no callback")
    })
    .unwrap();
    assert_eq!(operations, 0);
    let (bounds, cells) = sum.finish().into_parts();
    assert_eq!(cells, 6);
    assert_eq!(bounds.lower, ExactRatio::from_finite(0.));
}
#[test]
fn rounded_cancellation_can_refuse_a_zero_cap_without_a_retry() {
    let mut sum = ConicalSum::new(true, options(100));
    let mut operations = 0;
    sum.accumulate(point(1, 3), 0, &mut operations, &mut || false)
        .unwrap();
    sum.accumulate(point(-1, 3), 0, &mut operations, &mut || false)
        .unwrap();
    let options = RationalIntegralOptions {
        absolute_error_limit: 0.,
        ..Default::default()
    };
    assert!(
        matches!(publish::rounded_scalar(sum.finish(),options),Err(RationalIntegralFailure::ErrorLimit{requested:0.,achieved}) if achieved>0.)
    );
}
#[test]
fn every_import_checkpoint_cancels_before_its_actual_charge() {
    let mut count = 0;
    let mut operations = 0;
    let mut sum = ConicalSum::new(true, options(100));
    sum.accumulate(point(1, 3), 0, &mut operations, &mut || {
        count += 1;
        false
    })
    .unwrap();
    for stop in 1..=count {
        cancelled_import(stop);
    }
}
#[test]
fn retained_term_width_and_outward_rounding_have_distinct_budgets() {
    let mut sum = ConicalSum::new(true, options(100));
    let mut operations = 0;
    let lower = ExactRatio::new(1.into(), 3.into());
    let upper = ExactRatio::new(2.into(), 3.into());
    let width = upper.subtract(&lower);
    sum.accumulate(
        ContributionBounds {
            lower: lower.clone(),
            upper: upper.clone(),
        },
        0,
        &mut operations,
        &mut || false,
    )
    .unwrap();
    let result = bounds(sum);
    assert!(result.lower <= lower && result.upper >= upper);
    let two_quanta = ExactRatio::new(2.into(), num_bigint::BigInt::from(1) << 128);
    assert!(result.upper.subtract(&result.lower) < width.add(&two_quanta));
}
#[test]
fn parent_two_explicitly_selects_child_one_without_changing_shared_engine() {
    let parent = CylindricalFluxOptions {
        behavior_version: 2,
        ..Default::default()
    };
    assert_eq!(term_engine(parent, 3).behavior_version, 1);
    assert_eq!(parent.engine(3).behavior_version, 2);
    assert_eq!(term_engine(parent, 3).max_subintervals, 3);
}

fn cancelled_import(stop: usize) {
    let mut sum = ConicalSum::new(true, options(100));
    let mut operations = 0;
    let mut polls = 0;
    let failure = sum
        .accumulate(point(1, 3), 0, &mut operations, &mut || {
            polls += 1;
            polls == stop
        })
        .unwrap_err();
    assert_eq!(failure, BoundedArithmeticFailure::Cancelled);
    assert_eq!(polls, stop);
    assert_eq!(operations, stop as u32 - 1);
}
