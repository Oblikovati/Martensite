// SPDX-License-Identifier: GPL-2.0-only
use super::{super::DyadicBound, rational::ExactRatio};
use std::cmp::Ordering;

const MAX_FINITE_BITS: u64 = 0x7fef_ffff_ffff_ffff;

#[path = "rounding_quotient.rs"]
mod quotient;

pub(in crate::predicates::roots) fn outward_binary64(value: &ExactRatio) -> Option<[f64; 2]> {
    quotient::outward_binary64(value)
}

pub(in crate::predicates::roots) fn nearest_binary64(value: &ExactRatio) -> Option<f64> {
    let Some([lower, upper]) = outward_binary64(value) else {
        return nearest_at_overflow(value);
    };
    if lower == upper {
        return Some(lower);
    }
    let midpoint = DyadicBound::from_f64(lower)
        .ok()?
        .midpoint(&DyadicBound::from_f64(upper).ok()?);
    Some(match value.cmp(&ExactRatio::from_dyadic(&midpoint)) {
        Ordering::Less => lower,
        Ordering::Greater => upper,
        Ordering::Equal if lower.to_bits() & 1 == 0 => lower,
        Ordering::Equal => upper,
    })
}

fn nearest_at_overflow(value: &ExactRatio) -> Option<f64> {
    // The conceptual successor of MAX is 2^1024. Its even significand
    // wins a tie at MAX + 2^970, so only the strict interior rounds to MAX.
    let half_ulp = f64::from_bits((1023_u64 + 970) << 52);
    let midpoint = ExactRatio::from_finite(f64::MAX).add(&ExactRatio::from_finite(half_ulp));
    if value.sign() == Ordering::Less {
        return (value > &midpoint.negated()).then_some(-f64::MAX);
    }
    (value < &midpoint).then_some(f64::MAX)
}

pub(in crate::predicates::roots) fn outward_sqrt(value: &ExactRatio) -> Option<f64> {
    let bounds = outward_sqrt_bounds(value)?;
    Some(bounds[1])
}
pub(in crate::predicates::roots) fn outward_sqrt_bounds(value: &ExactRatio) -> Option<[f64; 2]> {
    if value.sign() == Ordering::Less {
        return None;
    }
    let maximum = ExactRatio::from_finite(f64::MAX);
    if value > &maximum.multiply(&maximum) {
        return None;
    }
    let floor = greatest_bits_where(|candidate| {
        let exact = ExactRatio::from_finite(candidate);
        exact.multiply(&exact) <= *value
    });
    let lower = ExactRatio::from_finite(f64::from_bits(floor));
    let exact = lower.multiply(&lower) == *value;
    Some([
        f64::from_bits(floor),
        f64::from_bits(floor + u64::from(!exact)),
    ])
}
fn greatest_bits_where(mut accepted: impl FnMut(f64) -> bool) -> u64 {
    let mut lower = 0;
    let mut upper = MAX_FINITE_BITS;
    while lower < upper {
        let middle = lower + (upper - lower).div_ceil(2);
        if accepted(f64::from_bits(middle)) {
            lower = middle;
        } else {
            upper = middle - 1;
        }
    }
    lower
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_checks_exact_rational_side_of_the_adjacent_midpoint() {
        let third = ExactRatio::new(1.into(), 3.into());
        assert_eq!(nearest_binary64(&third), Some(1. / 3.));
        assert_eq!(nearest_binary64(&third.negated()), Some(-1. / 3.));
        let odd = ExactRatio::from_finite(1_f64.next_up());
        let even = ExactRatio::from_finite(1_f64.next_up().next_up());
        let midpoint = odd.add(&even).divide(&ExactRatio::from_finite(2.));
        assert_eq!(nearest_binary64(&midpoint), Some(1_f64.next_up().next_up()));
    }

    #[test]
    fn nearest_near_overflow_rounds_to_signed_max_strictly_inside_the_midpoint() {
        let maximum = ExactRatio::from_finite(f64::MAX);
        let unit = f64::from_bits((1023_u64 + 968) << 52);
        for (multiple, finite) in [(1., true), (4., false), (7., false)] {
            let magnitude = maximum.add(&ExactRatio::from_finite(multiple * unit));
            assert_eq!(outward_binary64(&magnitude), None);
            assert_eq!(outward_binary64(&magnitude.negated()), None);
            assert_eq!(nearest_binary64(&magnitude), finite.then_some(f64::MAX));
            assert_eq!(
                nearest_binary64(&magnitude.negated()),
                finite.then_some(-f64::MAX)
            );
        }
    }
    #[test]
    fn owned_square_root_upper_baseline_covers_binary64_extremes() {
        for root in [0., f64::from_bits(1), f64::MIN_POSITIVE, 0.5, 1., f64::MAX] {
            let exact = ExactRatio::from_finite(root);
            assert_eq!(outward_sqrt(&exact.multiply(&exact)), Some(root));
        }
        let tiny = ExactRatio::from_finite(f64::from_bits(1));
        let half = tiny.divide(&ExactRatio::from_finite(2.));
        assert_eq!(outward_sqrt(&half.multiply(&half)), Some(f64::from_bits(1)));
        let maximum = ExactRatio::from_finite(f64::MAX);
        let excessive = maximum.multiply(&maximum).add(&ExactRatio::from_finite(1.));
        assert_eq!(outward_sqrt(&excessive), None);
        let upper = outward_sqrt(&ExactRatio::from_finite(2.)).unwrap();
        assert_eq!(upper.to_bits(), 0x3ff6_a09e_667f_3bcd);
    }
    #[test]
    fn square_root_pair_retains_exact_roots_and_outward_subnormal_bounds() {
        for root in [0., f64::from_bits(1), f64::MIN_POSITIVE, 0.5, f64::MAX] {
            let exact = ExactRatio::from_finite(root);
            assert_eq!(
                outward_sqrt_bounds(&exact.multiply(&exact)),
                Some([root, root])
            );
        }
        let half = ExactRatio::from_finite(f64::from_bits(1)).divide(&ExactRatio::from_finite(2.));
        assert_eq!(
            outward_sqrt_bounds(&half.multiply(&half)),
            Some([0., f64::from_bits(1)])
        );
        let bounds = outward_sqrt_bounds(&ExactRatio::from_finite(2.)).unwrap();
        let expected = [0x3ff6_a09e_667f_3bcc, 0x3ff6_a09e_667f_3bcd];
        assert_eq!(bounds.map(f64::to_bits), expected);
    }
    #[test]
    fn directed_rounding_contains_nonbinary_fraction_without_widening_exact_values() {
        let third = ExactRatio::new(1.into(), 3.into());
        let bounds = outward_binary64(&third).unwrap();
        assert!(ExactRatio::from_finite(bounds[0]) < third);
        assert!(ExactRatio::from_finite(bounds[1]) > third);
        assert_eq!(bounds[1].to_bits(), bounds[0].to_bits() + 1);
        assert_eq!(
            outward_binary64(&third.negated()),
            Some([-bounds[1], -bounds[0]])
        );
        assert_eq!(
            outward_binary64(&ExactRatio::from_finite(0.5)),
            Some([0.5, 0.5])
        );
    }
    #[test]
    fn directed_rounding_distinguishes_subnormal_enclosure_from_overflow() {
        let tiny = ExactRatio::from_finite(f64::from_bits(1));
        let half = tiny.divide(&ExactRatio::from_finite(2.));
        assert_eq!(outward_binary64(&half), Some([0., f64::from_bits(1)]));
        let too_large = ExactRatio::from_finite(f64::MAX).multiply(&ExactRatio::from_finite(2.));
        assert_eq!(outward_binary64(&too_large), None);
    }
    #[test]
    fn square_root_bound_is_the_smallest_binary64_proven_not_below_root() {
        let two = ExactRatio::from_finite(2.);
        let upper = outward_sqrt(&two).unwrap();
        let exact = ExactRatio::from_finite(upper);
        let predecessor = ExactRatio::from_finite(f64::from_bits(upper.to_bits() - 1));
        assert!(exact.multiply(&exact) >= two);
        assert!(predecessor.multiply(&predecessor) < two);
        assert_eq!(outward_sqrt(&ExactRatio::from_finite(4.)), Some(2.));
        assert!(outward_sqrt(&two.negated()).is_none());
    }
}

#[cfg(test)]
#[path = "rounding_quotient_tests.rs"]
mod quotient_tests;
