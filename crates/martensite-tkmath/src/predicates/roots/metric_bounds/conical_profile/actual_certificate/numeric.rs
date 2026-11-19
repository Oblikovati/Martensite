// SPDX-License-Identifier: GPL-2.0-only
//! Shared outward source operations; no manual uncharged numerical algebra.
use super::*;
pub(super) fn import(value: &DyadicBound, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let (n, d) = value.ratio();
    let bits = usize::from(arithmetic.fractional_bits());
    let n = n << bits;
    let d = d << bits;
    arithmetic.divide(
        &Interval {
            lower: n.clone(),
            upper: n,
        },
        &Interval {
            lower: d.clone(),
            upper: d,
        },
    )
}
pub(super) fn capture(values: &[f64], arithmetic: &mut Arithmetic<'_>) -> Checked<Vec<Interval>> {
    values.iter().map(|v| arithmetic.scalar(*v)).collect()
}
pub(super) fn hull(values: &[Interval]) -> Interval {
    Interval {
        lower: values
            .iter()
            .map(|v| &v.lower)
            .min()
            .cloned()
            .unwrap_or_default(),
        upper: values
            .iter()
            .map(|v| &v.upper)
            .max()
            .cloned()
            .unwrap_or_default(),
    }
}
pub(super) fn magnitude(value: &Interval) -> Interval {
    Interval {
        lower: 0.into(),
        upper: value.magnitude(),
    }
}
pub(super) fn max_magnitude(values: &[Interval]) -> Interval {
    Interval {
        lower: 0.into(),
        upper: values
            .iter()
            .map(Interval::magnitude)
            .max()
            .unwrap_or_default(),
    }
}
pub(super) fn lower_point(value: &Interval) -> Interval {
    Interval {
        lower: value.lower.clone(),
        upper: value.lower.clone(),
    }
}
pub(super) fn upper_point(value: &Interval) -> Interval {
    Interval {
        lower: value.upper.clone(),
        upper: value.upper.clone(),
    }
}
pub(super) fn neg(value: &Interval) -> Interval {
    Interval {
        lower: -value.upper.clone(),
        upper: -value.lower.clone(),
    }
}
pub(super) fn square(value: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let mut result = arithmetic.multiply(value, value)?;
    if value.lower <= 0.into() && value.upper >= 0.into() {
        result.lower = 0.into();
    }
    Ok(result)
}
pub(super) fn positive(
    field: &'static str,
    value: &Interval,
    arithmetic: &Arithmetic<'_>,
) -> Outcome<()> {
    if value.lower <= 0.into() {
        return Err(error::unresolved(
            field,
            value,
            "strictly positive",
            arithmetic.fractional_bits(),
        ));
    }
    Ok(())
}
pub(super) fn orientation(value: i8, field: &'static str) -> Outcome<()> {
    if !matches!(value, -1 | 1) {
        return Err(ConicalActualPatchFailure::Value {
            field,
            actual: f64::from(value),
            expected: "orientation -1 or +1",
        });
    }
    Ok(())
}
pub(super) fn policy(options: ConeClearanceOptions) -> Outcome<()> {
    if !(8..=4096).contains(&options.fractional_bits) || options.max_operations > 1_000_000 {
        return Err(ConicalActualPatchFailure::Policy { actual: options });
    }
    Ok(())
}
