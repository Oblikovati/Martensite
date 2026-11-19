// SPDX-License-Identifier: GPL-2.0-only
//! Full exact coefficient equality is established after actual original restriction.
use super::*;
#[cfg(test)]
mod tests;
pub(super) fn displacement(
    source: &[Vec<Interval>; 2],
    child: &[Vec<Interval>; 2],
    original: ConicalOriginalSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(DisplacementEvidence, Option<[Vec<Interval>; 2]>), ConicalOriginalFailure> {
    let (x, first) = axis(&source[0], &child[0], original, arithmetic)?;
    let (y, second) = axis(&source[1], &child[1], original, arithmetic)?;
    let evidence = displacement_from_axes(x, y, arithmetic)?;
    Ok((evidence, Some([first, second])))
}
fn axis(
    source: &[Interval],
    child: &[Interval],
    original: ConicalOriginalSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(AxisEvidence, Vec<Interval>), ConicalOriginalFailure> {
    let restricted = restrict(source, original.index, original.level, arithmetic)?;
    let evidence = match exact_coefficients(&restricted, child) {
        true => zero_axis(&restricted, child, arithmetic),
        false => axis_from_restriction(&restricted, child, arithmetic)?,
    };
    Ok((evidence, restricted))
}
fn exact_coefficients(source: &[Interval], child: &[Interval]) -> bool {
    source.len() == child.len()
        && source
            .iter()
            .zip(child)
            .all(|(a, b)| exact_same_point(a, b))
}
fn zero_axis(source: &[Interval], child: &[Interval], arithmetic: &Arithmetic<'_>) -> AxisEvidence {
    let zero = 0.into();
    AxisEvidence {
        error: arithmetic.zero(),
        derivative_error: arithmetic.zero(),
        exact: true,
        positive: source[0].lower > zero && child[0].lower > zero,
        nonnegative: source[0].lower >= zero && child[0].lower >= zero,
        negative: source[0].upper < zero && child[0].upper < zero,
        source_nonnegative: source[0].lower >= zero,
        source_negative: source[0].upper < zero,
        child_nonnegative: child[0].lower >= zero,
        child_negative: child[0].upper < zero,
    }
}
pub(super) fn zero(
    displacement: DisplacementEvidence,
    restricted: Option<[Vec<Interval>; 2]>,
    radius: Interval,
    arithmetic: &Arithmetic<'_>,
) -> OriginalEvidence {
    OriginalEvidence {
        bound: arithmetic.zero(),
        angular_derivative: arithmetic.zero(),
        anchor: displacement.anchor,
        comparison: Some(original_comparison(
            displacement,
            radius,
            arithmetic.zero(),
            restricted,
        )),
    }
}
