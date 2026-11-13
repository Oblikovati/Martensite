// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn exact(value: &BigInt, bits: u16) -> ExactRatio {
    ExactRatio::from_dyadic(&DyadicBound::new(value.clone(), -i32::from(bits)))
}

fn contains(interval: &Interval, value: &ExactRatio, bits: u16) {
    assert!(exact(&interval.lower, bits) <= *value);
    assert!(exact(&interval.upper, bits) >= *value);
}

#[test]
fn directed_signed_arithmetic_encloses_independent_exact_ratios() {
    let options = ArithmeticPolicy {
        fractional_bits: 8,
        ..Default::default()
    };
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(options, &mut cancelled);
    for a in [-2.5, -0.1, 0., 0.1, 3.5] {
        for b in [-3.1, -0.125, 0., 0.125, 5.1] {
            check_pair(a, b, &mut arithmetic);
        }
    }
}

fn check_pair(a: f64, b: f64, arithmetic: &mut Arithmetic<'_>) {
    let (x, y) = (arithmetic.scalar(a).unwrap(), arithmetic.scalar(b).unwrap());
    let (a, b) = (ExactRatio::from_finite(a), ExactRatio::from_finite(b));
    contains(&arithmetic.add(&x, &y).unwrap(), &a.add(&b), 8);
    contains(&arithmetic.subtract(&x, &y).unwrap(), &a.subtract(&b), 8);
    contains(&arithmetic.multiply(&x, &y).unwrap(), &a.multiply(&b), 8);
    contains(
        &arithmetic.fraction(&x, 1, 3).unwrap(),
        &a.divide(&ExactRatio::from_finite(3.)),
        8,
    );
}

#[test]
fn subnormal_capture_stays_outward_and_integer_growth_is_bounded() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let tiny = arithmetic.scalar(f64::from_bits(1)).unwrap();
    contains(&tiny, &ExactRatio::from_finite(f64::from_bits(1)), 128);
    assert_eq!((tiny.lower, tiny.upper), (0.into(), 1.into()));
    let enormous = BigInt::from(1) << 16_384;
    assert!(matches!(
        checked(enormous, 0.into()),
        Err(Failure::ArithmeticBits { .. })
    ));
}

#[test]
fn operation_local_arithmetic_can_move_between_threads() {
    fn require_send<T: Send>() {}
    require_send::<Arithmetic<'static>>();
}

#[test]
fn unrepresentable_bound_retains_the_actual_endpoint_without_clamping() {
    let mut cancelled = || false;
    let arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let cause = arithmetic.upper_f64(BigInt::from(1) << 1152).unwrap_err();
    let Failure::UnrepresentableBound { bound } = cause else {
        panic!("unexpected cause: {cause:?}");
    };
    assert_eq!(
        bound.compare_f64(f64::MAX).unwrap(),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn signed_interval_division_encloses_all_independent_endpoint_quotients() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let numerator = Interval {
        lower: BigInt::from(-3),
        upper: 5.into(),
    };
    let denominator = Interval {
        lower: 2.into(),
        upper: 7.into(),
    };
    let quotient = arithmetic.divide(&numerator, &denominator).unwrap();
    for n in [-3., 5.] {
        for d in [2., 7.] {
            let exact = ExactRatio::from_finite(n).divide(&ExactRatio::from_finite(d));
            contains(&quotient, &exact, 128);
        }
    }
}
