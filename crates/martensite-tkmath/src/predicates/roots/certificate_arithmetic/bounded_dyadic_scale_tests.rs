// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn original_scalar(arithmetic: &Arithmetic<'_>, value: f64) -> Checked<Interval> {
    let integer = crate::predicates::exact_coordinate(value, 0, 0)
        .map_err(|_| Failure::NonFiniteScalar { value })?;
    let numerator = integer << usize::from(arithmetic.fractional_bits());
    arithmetic.rounded(&numerator, &numerator, &(BigInt::from(1) << 1074))
}

fn original_product(arithmetic: &Arithmetic<'_>, a: &Interval, b: &Interval) -> Checked<Interval> {
    let products = [
        &a.lower * &b.lower,
        &a.lower * &b.upper,
        &a.upper * &b.lower,
        &a.upper * &b.upper,
    ];
    let lower = products
        .iter()
        .fold(products[0].clone(), |a, b| a.min(b.clone()));
    let upper = products
        .iter()
        .fold(products[0].clone(), |a, b| a.max(b.clone()));
    arithmetic.rounded(&lower, &upper, &arithmetic.unit)
}

#[test]
fn compiled_binomials_match_each_integer_coefficient_without_work() {
    let mut never_poll = || panic!("reading the constructor and Pascal table owes no poll");
    let arithmetic = Arithmetic::new(Default::default(), &mut never_poll);
    for n in 0..=50 {
        let mut choose = 1_u64;
        for k in 0..=50 {
            assert_eq!(arithmetic.binomial(n, k), choose, "C({n},{k})");
            choose = if k < n {
                choose * (n - k) as u64 / (k + 1) as u64
            } else {
                0
            };
        }
    }
    assert_eq!(arithmetic.operations(), 0);
}

#[test]
fn shift_scalar_matches_original_signed_extremes_and_grid_limits() {
    let values = [
        0.,
        -0.,
        f64::from_bits(1),
        -f64::from_bits(3),
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        -1.25,
        -1.1,
        0.9,
        1.,
        f64::MAX,
        -f64::MAX,
        f64::INFINITY,
    ];
    for fractional_bits in [0, 1, 8, 128, 1022, 1074, 1075, 14_286, 14_287, u16::MAX] {
        let options = ArithmeticPolicy {
            fractional_bits,
            ..Default::default()
        };
        let mut no_cancel = || false;
        let mut arithmetic = Arithmetic::new(options, &mut no_cancel);
        for value in values {
            let expected = original_scalar(&arithmetic, value);
            assert_eq!(
                arithmetic.scalar(value),
                expected,
                "grid {fractional_bits}, value {value}"
            );
        }
        assert_eq!(arithmetic.operations(), values.len() as u32);
    }
}

#[test]
fn shift_product_matches_original_outward_endpoints_and_pre_rounding_refusals() {
    let small =
        [(-20, -3), (-18, 0), (-3, 2), (0, 0), (0, 5), (1, 9)].map(|(low, high)| Interval {
            lower: low.into(),
            upper: high.into(),
        });
    let factor = BigInt::from(1) << 8191_usize;
    let large = [
        Interval {
            lower: -&factor,
            upper: factor.clone(),
        },
        Interval {
            lower: -(&factor << 1_usize),
            upper: BigInt::from(0),
        },
    ];
    for fractional_bits in [0, 1, 3, 128, 1024, u16::MAX] {
        let options = ArithmeticPolicy {
            fractional_bits,
            ..Default::default()
        };
        let mut no_cancel = || false;
        let mut arithmetic = Arithmetic::new(options, &mut no_cancel);
        for a in small.iter().chain(&large) {
            for b in small.iter().chain(&large) {
                let expected = original_product(&arithmetic, a, b);
                assert_eq!(
                    arithmetic.multiply(a, b),
                    expected,
                    "grid {fractional_bits}"
                );
            }
        }
        assert_eq!(arithmetic.operations(), 64);
    }
}

#[test]
fn shift_lanes_keep_one_poll_and_charge_and_cancel_before_refusals() {
    let mut polls = 0;
    let mut cancel_third = || {
        polls += 1;
        polls == 3
    };
    let options = ArithmeticPolicy {
        fractional_bits: u16::MAX,
        max_operations: 2,
    };
    {
        let mut arithmetic = Arithmetic::new(options, &mut cancel_third);
        let nan = f64::from_bits(0xfff8_0000_1234_5678);
        let error = arithmetic.scalar(nan).unwrap_err();
        let Failure::NonFiniteScalar { value } = error else {
            panic!("{error:?}")
        };
        assert_eq!(value.to_bits(), nan.to_bits());
        let zero = arithmetic.zero();
        assert_eq!(arithmetic.multiply(&zero, &zero), Ok(zero));
        assert_eq!(arithmetic.scalar(f64::MAX), Err(Failure::Cancelled));
        assert_eq!(
            arithmetic.scalar(f64::MAX),
            Err(Failure::WorkLimit {
                completed: 2,
                limit: 2
            })
        );
        assert_eq!(arithmetic.operations(), 2);
    }
    assert_eq!(polls, 4);
}
