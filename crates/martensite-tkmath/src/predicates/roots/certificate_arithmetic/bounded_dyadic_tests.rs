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

#[test]
fn copied_policy_preserves_settings_without_charging_or_polling() {
    let options = ArithmeticPolicy {
        fractional_bits: 8,
        max_operations: 3,
    };
    let mut cancelled = || true;
    let arithmetic = Arithmetic::new(options, &mut cancelled);
    let copied = arithmetic.policy();
    assert_eq!(copied.fractional_bits, options.fractional_bits);
    assert_eq!(copied.max_operations, options.max_operations);
    assert_eq!(arithmetic.operations(), 0);
}

fn square_root_bounds(lower: i64, upper: i64, bits: u16) -> Interval {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: bits,
            ..Default::default()
        },
        &mut cancelled,
    );
    let input = Interval {
        lower: lower.into(),
        upper: upper.into(),
    };
    let root = arithmetic.positive_sqrt(&input).unwrap();
    assert_eq!(arithmetic.operations(), 1);
    assert_tight_square_bounds(&input, &root, bits);
    root
}

fn assert_tight_square_bounds(input: &Interval, root: &Interval, bits: u16) {
    let lower = exact(&root.lower, bits);
    let upper = exact(&root.upper, bits);
    assert!(lower.multiply(&lower) <= exact(&input.lower, bits));
    assert!(upper.multiply(&upper) >= exact(&input.upper, bits));
    let next = exact(&(&root.lower + 1), bits);
    let previous = exact(&(&root.upper - 1), bits);
    assert!(next.multiply(&next) > exact(&input.lower, bits));
    assert!(previous.multiply(&previous) < exact(&input.upper, bits));
}

#[test]
fn exact_squares_retain_exact_positive_endpoints() {
    for (input, root) in [(1, 16), (256, 256), (1024, 512)] {
        let result = square_root_bounds(input, input, 8);
        assert_eq!(
            result,
            Interval {
                lower: root.into(),
                upper: root.into()
            }
        );
    }
}

#[test]
fn nonsquare_positive_intervals_have_tight_outward_grid_endpoints() {
    let bounds = square_root_bounds(2, 3, 8);
    assert_eq!(
        bounds,
        Interval {
            lower: 22.into(),
            upper: 28.into()
        }
    );
    for bits in [0, 1, 8, 128] {
        for (lower, upper) in [(1, 1), (2, 3), (8, 19), (257, 513)] {
            square_root_bounds(lower, upper, bits);
        }
    }
}

#[test]
fn nonpositive_lower_endpoints_refuse_with_the_original_bound() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    for lower in [-4, -1, 0] {
        let input = Interval {
            lower: lower.into(),
            upper: 5.into(),
        };
        let cause = arithmetic.positive_sqrt(&input).unwrap_err();
        let PositiveSqrtFailure::Arithmetic(Failure::DenominatorUnresolved { lower: actual }) =
            cause
        else {
            panic!("expected positive-domain refusal, got {cause:?}");
        };
        assert_eq!(actual, DyadicBound::new(lower.into(), -128));
    }
    assert_eq!(arithmetic.operations(), 3);
}

#[test]
fn square_root_growth_refuses_before_integer_root_evaluation() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let input = Interval {
        lower: 1.into(),
        upper: BigInt::from(1) << MAX_INTEGER_BITS,
    };
    assert!(matches!(
        arithmetic.positive_sqrt(&input),
        Err(PositiveSqrtFailure::Arithmetic(Failure::ArithmeticBits { actual, limit }))
            if actual > limit && limit == MAX_INTEGER_BITS
    ));
    assert_eq!(arithmetic.operations(), 1);
}

#[test]
fn square_root_work_refusal_precedes_domain_admission() {
    let mut cancelled = || false;
    let options = ArithmeticPolicy {
        max_operations: 0,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(options, &mut cancelled);
    assert!(matches!(
        arithmetic.positive_sqrt(&arithmetic.zero()),
        Err(PositiveSqrtFailure::Arithmetic(Failure::WorkLimit {
            completed: 0,
            limit: 0
        }))
    ));
    assert_eq!(arithmetic.operations(), 0);
}

#[test]
fn square_root_cancellation_precedes_work_and_domain_admission() {
    let mut cancelled = || true;
    let options = ArithmeticPolicy {
        max_operations: 0,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(options, &mut cancelled);
    assert!(matches!(
        arithmetic.positive_sqrt(&arithmetic.zero()),
        Err(PositiveSqrtFailure::Arithmetic(Failure::Cancelled))
    ));
    assert_eq!(arithmetic.operations(), 0);
}

#[test]
fn reversed_square_root_intervals_refuse_with_original_endpoints() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    for upper in [-1, 0, 1] {
        let input = Interval {
            lower: 2.into(),
            upper: upper.into(),
        };
        let cause = arithmetic.positive_sqrt(&input).unwrap_err();
        assert_eq!(
            cause,
            PositiveSqrtFailure::InvalidInterval {
                lower: DyadicBound::new(2.into(), -128),
                upper: DyadicBound::new(upper.into(), -128),
            }
        );
    }
    assert_eq!(arithmetic.operations(), 3);
}

#[test]
fn square_root_work_refusal_precedes_interval_order_admission() {
    let input = Interval {
        lower: 2.into(),
        upper: (-1).into(),
    };
    let mut cancelled = || false;
    let options = ArithmeticPolicy {
        max_operations: 0,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(options, &mut cancelled);
    assert!(matches!(
        arithmetic.positive_sqrt(&input),
        Err(PositiveSqrtFailure::Arithmetic(Failure::WorkLimit { .. }))
    ));
}

#[test]
fn square_root_cancellation_precedes_interval_order_admission() {
    let input = Interval {
        lower: 2.into(),
        upper: (-1).into(),
    };
    let options = ArithmeticPolicy {
        max_operations: 0,
        ..Default::default()
    };
    let mut cancelled = || true;
    let mut arithmetic = Arithmetic::new(options, &mut cancelled);
    assert!(matches!(
        arithmetic.positive_sqrt(&input),
        Err(PositiveSqrtFailure::Arithmetic(Failure::Cancelled))
    ));
}
