// SPDX-License-Identifier: GPL-2.0-only
//! Direct actual-UV development is compared with the exact original dyadic restriction.
use super::*;
pub(super) struct SpanBounds {
    pub uv: [Interval; 2],
    pub error: Interval,
    pub halfplane: Interval,
}
pub(super) fn certify(
    query: &ConicalMaterialQuery<'_>,
    span: &ConicalMaterialSpan<'_>,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<SpanBounds, CylindricalProfileFailure> {
    let uv = capture_planar(span.uv, arithmetic)?;
    let phase = phase(&uv[0], query.chart_origin, sine, arithmetic)?;
    principal_halfplane(&phase, arithmetic)?;
    let slant = offset(&uv[1], query.reference_slant, arithmetic)?;
    let trig = trigonometric::enclose(&phase, arithmetic)?;
    let actual = [
        polynomial::product(&slant, &trig[0], arithmetic)?,
        polynomial::product(&slant, &trig[1], arithmetic)?,
    ];
    let original = original(span, arithmetic)?;
    let halfplane = algebra::hull(&[algebra::hull(&actual[0]), algebra::hull(&original[0])]);
    let error = residual(&actual, &original, arithmetic)?;
    Ok(SpanBounds {
        uv: uv.each_ref().map(|axis| algebra::hull(axis)),
        error,
        halfplane,
    })
}
fn original(
    span: &ConicalMaterialSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 2], CylindricalProfileFailure> {
    let original = capture_planar(span.original, arithmetic)?;
    Ok([
        restrict(
            &original[0],
            span.interval.index(),
            span.interval.level(),
            arithmetic,
        )?,
        restrict(
            &original[1],
            span.interval.index(),
            span.interval.level(),
            arithmetic,
        )?,
    ])
}
fn phase(
    u: &[Interval],
    origin: f64,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, CylindricalProfileFailure> {
    let origin = arithmetic.scalar(origin)?;
    u.iter()
        .map(|value| {
            let delta = arithmetic.subtract(value, &origin)?;
            Ok(arithmetic.multiply(&delta, sine)?)
        })
        .collect()
}
fn principal_halfplane(
    phase: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    let half = arithmetic.fraction(&pi, 1, 2)?;
    let hull = algebra::hull(phase);
    if hull.lower <= -half.lower.clone() || hull.upper >= half.lower {
        return Err(input::value(
            "canonical developed phase strictly in (-pi/2,pi/2)",
            arithmetic.upper_f64(hull.upper)?,
        ));
    }
    Ok(())
}
fn offset(
    values: &[Interval],
    scalar: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, CylindricalProfileFailure> {
    let scalar = arithmetic.scalar(scalar)?;
    values
        .iter()
        .map(|value| Ok(arithmetic.add(value, &scalar)?))
        .collect()
}
fn residual(
    actual: &[Vec<Interval>; 2],
    original: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let x = algebra::difference(&actual[0], &original[0], arithmetic)?;
    let y = algebra::difference(&actual[1], &original[1], arithmetic)?;
    Ok(arithmetic.add(&algebra::magnitude(&x), &algebra::magnitude(&y))?)
}
