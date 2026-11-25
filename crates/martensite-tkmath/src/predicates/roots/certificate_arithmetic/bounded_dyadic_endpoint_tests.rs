// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn original_divide(
    arithmetic: &mut Arithmetic<'_>,
    a: &Interval,
    b: &Interval,
) -> Checked<Interval> {
    arithmetic.charge()?;
    arithmetic.positive_denominator(&b.lower)?;
    let pairs = [
        (&a.lower, &b.lower),
        (&a.lower, &b.upper),
        (&a.upper, &b.lower),
        (&a.upper, &b.upper),
    ];
    let [a, b, c, d] =
        pairs.map(|(a, b)| arithmetic.rounded(&(a * &arithmetic.unit), &(a * &arithmetic.unit), b));
    let bounds = [a?, b?, c?, d?];
    let lower = bounds
        .iter()
        .map(|value| &value.lower)
        .min()
        .unwrap()
        .clone();
    let upper = bounds
        .iter()
        .map(|value| &value.upper)
        .max()
        .unwrap()
        .clone();
    checked(lower, upper)
}

fn interval(lower: impl Into<BigInt>, upper: impl Into<BigInt>) -> Interval {
    Interval {
        lower: lower.into(),
        upper: upper.into(),
    }
}

#[test]
fn endpoint_division_matches_four_corner_results_and_refusal_order() {
    for fractional_bits in [0, 1, 8, 128, 16_383, u16::MAX] {
        let options = ArithmeticPolicy {
            fractional_bits,
            ..Default::default()
        };
        let mut old_cancel = || false;
        let mut new_cancel = || false;
        let mut old = Arithmetic::new(options, &mut old_cancel);
        let mut new = Arithmetic::new(options, &mut new_cancel);
        let smaller = BigInt::from(1) << 16_383usize;
        let larger = BigInt::from(1) << 16_390usize;
        for a in [
            interval(-7, -2),
            interval(-2, 7),
            interval(0, 0),
            interval(0, 1),
            interval(2, 7),
            interval(7, -2),
            interval(-&smaller, larger.clone()),
            interval(-&larger, smaller),
        ] {
            for b in [
                interval(1, 1),
                interval(1, 3),
                interval(2, 7),
                interval(7, 1),
                interval(7, -1),
                interval(-1, 7),
            ] {
                assert_eq!(new.divide(&a, &b), original_divide(&mut old, &a, &b));
            }
        }
        assert_eq!(old.operations(), new.operations());
    }
}

#[test]
fn endpoint_division_keeps_cancellation_and_work_precedence() {
    let mut polls = 0;
    let mut cancel = || {
        polls += 1;
        polls == 2
    };
    let options = ArithmeticPolicy {
        max_operations: 1,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(options, &mut cancel);
    let a = interval(-1, 2);
    assert!(arithmetic.divide(&a, &interval(1, 3)).is_ok());
    assert_eq!(
        arithmetic.divide(&a, &interval(-1, 0)),
        Err(Failure::Cancelled)
    );
    assert_eq!(
        arithmetic.divide(&a, &interval(-1, 0)),
        Err(Failure::WorkLimit {
            completed: 1,
            limit: 1
        })
    );
    assert_eq!(arithmetic.operations(), 1);
    assert_eq!(polls, 3);
}

#[test]
fn endpoint_products_match_all_four_corners_across_signs() {
    let pairs = [(-7, -2), (-2, 7), (-7, 0), (0, 0), (0, 1), (2, 7), (7, -2)];
    let mut cancel = || false;
    let options = ArithmeticPolicy {
        fractional_bits: 3,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(options, &mut cancel);
    for (alo, ahi) in pairs {
        for (blo, bhi) in pairs {
            let products = [alo * blo, alo * bhi, ahi * blo, ahi * bhi];
            let lower = BigInt::from(*products.iter().min().unwrap());
            let upper = BigInt::from(*products.iter().max().unwrap());
            assert_eq!(
                arithmetic.multiply(&interval(alo, ahi), &interval(blo, bhi)),
                rounded_shift(&lower, &upper, usize::from(options.fractional_bits)),
            );
        }
    }
    assert_eq!(arithmetic.operations(), (pairs.len() * pairs.len()) as u32);
}
