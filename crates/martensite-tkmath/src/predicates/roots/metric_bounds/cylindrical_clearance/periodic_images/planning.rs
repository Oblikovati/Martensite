// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use num_bigint::BigInt;

pub(super) struct PeriodicPlan {
    pub images: Vec<CylindricalPeriodicTranslation>,
    pub period: Interval,
    pub width: Interval,
    pub tail: Interval,
    pub operations: u32,
    pub maximum: u32,
}

pub(super) fn plan(
    source: &CertifiedCylindricalClearance,
    maximum: u32,
    options: CylindricalClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> WorkChecked<PeriodicPlan> {
    admit_maximum(maximum).map_err(|cause| (cause, 0))?;
    let policy = ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    build(source, maximum, &mut arithmetic).map_err(|cause| (cause, arithmetic.operations()))
}

fn build(
    source: &CertifiedCylindricalClearance,
    maximum: u32,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<PeriodicPlan> {
    let width = periodic_slab::source_width(&source.sources, arithmetic)?;
    let period = periodic_slab::circumference(source.radius, arithmetic)?;
    let count = image_count(&width, &period, source.clearance, maximum, arithmetic)?;
    let images = image_shifts(count, &period, source.clearance, arithmetic)?;
    assemble_plan(width, period, images, maximum, arithmetic)
}

fn image_shifts(
    count: u32,
    period: &Interval,
    clearance: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<CylindricalPeriodicTranslation>> {
    (1..=count)
        .map(|turn| translation(turn, period, clearance, arithmetic))
        .collect()
}

fn assemble_plan(
    width: Interval,
    period: Interval,
    images: Vec<CylindricalPeriodicTranslation>,
    maximum: u32,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<PeriodicPlan> {
    let next = arithmetic.fraction(&period, images.len() as u64 + 1, 1)?;
    let tail = arithmetic.subtract(&next, &width)?;
    Ok(PeriodicPlan {
        images,
        period,
        width,
        tail,
        operations: arithmetic.operations(),
        maximum,
    })
}

fn admit_maximum(maximum: u32) -> Checked<()> {
    if maximum > 1024 {
        return Err(CylindricalClearanceFailure::PeriodicImagePolicy {
            maximum,
            admitted: 1024,
        });
    }
    Ok(())
}

fn image_count(
    width: &Interval,
    period: &Interval,
    clearance: f64,
    maximum: u32,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<u32> {
    let requested = arithmetic.scalar(clearance)?;
    let span = arithmetic.add(width, &requested)?;
    let count = arithmetic.integer_quotient(&span.upper, &period.lower)?;
    bounded_count(count, maximum)
}

fn bounded_count(count: BigInt, maximum: u32) -> Checked<u32> {
    if count > BigInt::from(maximum) {
        return Err(CylindricalClearanceFailure::PeriodicImageLimit {
            required: count,
            maximum,
        });
    }
    u32::try_from(count.clone()).map_err(|_| CylindricalClearanceFailure::PeriodicImageLimit {
        required: count,
        maximum,
    })
}

fn translation(
    turn: u32,
    period: &Interval,
    clearance: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CylindricalPeriodicTranslation> {
    let actual = arithmetic.fraction(period, u64::from(turn), 1)?;
    let offset = arithmetic.upper_f64((&actual.lower + &actual.upper) / 2)?;
    let error = translation_error(&actual, offset, arithmetic)?;
    let requested = arithmetic.scalar(clearance)?;
    let limit = arithmetic.add(&requested, &error)?;
    Ok(CylindricalPeriodicTranslation {
        turn,
        offset,
        error: bound(error.upper, arithmetic.fractional_bits()),
        comparison: arithmetic.upper_f64(limit.upper)?,
    })
}

fn translation_error(
    actual: &Interval,
    offset: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let literal = arithmetic.scalar(offset)?;
    let difference = arithmetic.subtract(actual, &literal)?.magnitude();
    Ok(periodic_slab::point(difference))
}
