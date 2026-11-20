// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn width(
    range: &Interval,
    period: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    let width = arithmetic.subtract(&numeric::upper_point(range), &numeric::lower_point(range))?;
    if width.upper >= period.lower {
        return Err(error::unresolved(
            "actual cap angular width",
            &width,
            "strictly below certified full period",
            arithmetic.fractional_bits(),
        ));
    }
    Ok(())
}
pub(super) fn certify(
    caps: &[CapCapture],
    period: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    let family = numeric::hull(&caps.iter().map(|v| v.u.clone()).collect::<Vec<_>>());
    width(&family, period, arithmetic)?;
    for first in 0..caps.len() {
        arithmetic.check_cancelled()?;
        for last in first + 1..caps.len() {
            pair(&caps[first], &caps[last], arithmetic).map_err(|cause| {
                ConicalActualPatchFailure::CapPair {
                    caps: [
                        caps[first].source.carrier().id,
                        caps[last].source.carrier().id,
                    ],
                    cause: Box::new(cause),
                }
            })?;
        }
    }
    Ok(())
}
pub(super) fn pair(a: &CapCapture, b: &CapCapture, arithmetic: &mut Arithmetic<'_>) -> Outcome<()> {
    let same = a.source.carrier().member == b.source.carrier().member;
    if !same && let (CapExtent::Slant(first), CapExtent::Slant(last)) = (&a.extent, &b.extent) {
        arithmetic.certificate_step()?;
        if separated(first, last) || separated(&a.level, &b.level) {
            return Ok(());
        }
    }
    let mut gap = initial_gap(a, b, same, arithmetic)?;
    if gap.upper < 0.into() {
        gap = reverse_gap(a, b, same, &gap, arithmetic)?;
    }
    let quantity = if same {
        "opposite actual cap levels"
    } else {
        "cross-member filled-cap phase gap"
    };
    numeric::positive(quantity, &gap, arithmetic)
}

fn separated(a: &Interval, b: &Interval) -> bool {
    a.upper < b.lower || b.upper < a.lower
}

fn initial_gap(
    a: &CapCapture,
    b: &CapCapture,
    same: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    if same {
        return Ok(arithmetic.subtract(&b.level, &a.level)?);
    }
    Ok(arithmetic.subtract(&numeric::lower_point(&b.u), &numeric::upper_point(&a.u))?)
}
fn reverse_gap(
    a: &CapCapture,
    b: &CapCapture,
    same: bool,
    gap: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    if same {
        return Ok(numeric::neg(gap));
    }
    Ok(numeric::neg(&arithmetic.subtract(
        &numeric::upper_point(&b.u),
        &numeric::lower_point(&a.u),
    )?))
}
