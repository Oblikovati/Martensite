// SPDX-License-Identifier: GPL-2.0-only
//! #570: only parent-version2 conical aggregates round already weighted volume contributions.
use super::{publish, subinterval::ContributionBounds, *};
use crate::predicates::roots::metric_bounds::CylindricalProfileOptions;
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{
        bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
        bounded_failure::BoundedArithmeticFailure,
    },
};
pub(super) enum ConicalSum {
    Exact(publish::BoundsSum),
    Outward {
        value: Interval,
        cells: u64,
        options: CylindricalProfileOptions,
    },
}
impl ConicalSum {
    pub(super) fn new(outward: bool, options: CylindricalProfileOptions) -> Self {
        if outward {
            return Self::Outward {
                value: Interval {
                    lower: 0.into(),
                    upper: 0.into(),
                },
                cells: 0,
                options,
            };
        }
        Self::Exact(publish::BoundsSum::zero())
    }
    pub(super) fn accumulate(
        &mut self,
        part: ContributionBounds,
        cells: u64,
        operations: &mut u32,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Result<(), BoundedArithmeticFailure> {
        match self {
            Self::Exact(sum) => {
                sum.accumulate(part, cells);
                Ok(())
            }
            Self::Outward {
                value,
                cells: examined,
                options,
            } => {
                *examined += cells;
                accumulate_outward(value, part, *options, operations, cancelled)
            }
        }
    }
    pub(super) fn finish(self) -> publish::BoundsSum {
        match self {
            Self::Exact(sum) => sum,
            Self::Outward {
                value,
                cells,
                options,
            } => dyadic_sum(value, cells, options),
        }
    }
}
fn accumulate_outward(
    value: &mut Interval,
    part: ContributionBounds,
    options: CylindricalProfileOptions,
    operations: &mut u32,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<(), BoundedArithmeticFailure> {
    if *operations > options.max_operations {
        return Err(BoundedArithmeticFailure::WorkLimit {
            completed: *operations,
            limit: options.max_operations,
        });
    }
    let policy = ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations - *operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let result = import(&part, &mut arithmetic).and_then(|part| arithmetic.add(value, &part));
    *operations += arithmetic.operations();
    *value = result?;
    Ok(())
}
fn import(
    part: &ContributionBounds,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundedArithmeticFailure> {
    let lower = endpoint(&part.lower, arithmetic)?;
    let upper = endpoint(&part.upper, arithmetic)?;
    Ok(Interval {
        lower: lower.lower,
        upper: upper.upper,
    })
}
fn endpoint(
    value: &ExactRatio,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundedArithmeticFailure> {
    let quantum = arithmetic.quantum();
    let numerator = arithmetic.integer_scale(&quantum, value.numerator())?;
    let denominator = arithmetic.integer_scale(&quantum, value.denominator())?;
    arithmetic.divide(&numerator, &denominator)
}
pub(super) fn term_engine(
    options: CylindricalFluxOptions,
    remaining: u32,
) -> RationalIntegralOptions {
    let mut child = options.engine(remaining);
    // The parent selects accumulation2; the unchanged scalar quotient algorithm is explicitly1.
    child.behavior_version = 1;
    child
}
#[cfg(test)]
mod tests;

fn dyadic_sum(
    value: Interval,
    cells: u64,
    options: CylindricalProfileOptions,
) -> publish::BoundsSum {
    let exponent = -i32::from(options.fractional_bits);
    let bounds = ContributionBounds {
        lower: ExactRatio::from_dyadic(&DyadicBound::new(value.lower, exponent)),
        upper: ExactRatio::from_dyadic(&DyadicBound::new(value.upper, exponent)),
    };
    let mut sum = publish::BoundsSum::zero();
    sum.accumulate(bounds, cells);
    sum
}
