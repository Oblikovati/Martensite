// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn bounds(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    delta: &[Interval],
    native_radius: &[Interval],
    input: &Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 3], CylindricalProfileFailure> {
    let depth = physical_depth(input, depth, arithmetic)?;
    let radial_row = Component::radial(query, input);
    let radial = component(radial_row, &depth, delta, input, arithmetic)?;
    let axial_row = Component::axial(query, input);
    let axial = component(axial_row, &depth, delta, input, arithmetic)?;
    Ok([radial, axial, algebra::magnitude(native_radius)])
}
fn physical_depth(
    input: &Bridge<'_, '_>,
    depth: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let depth = if input.source.outward {
        input.source.depths[depth]
    } else {
        -input.source.depths[depth]
    };
    Ok(arithmetic.scalar(depth)?)
}
pub(super) fn native_radius(
    query: ConicalTrimQuery<'_>,
    v: &[Interval],
    input: &Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, CylindricalProfileFailure> {
    let radius = arithmetic.scalar(query.carrier.radius)?;
    let v = scaled(v, &input.domain.sine, arithmetic)?;
    let radius = v
        .iter()
        .map(|v| arithmetic.add(v, &radius))
        .collect::<Checked<Vec<_>>>()?;
    trace::positive_radius(&radius, query.carrier.radius, arithmetic)?;
    Ok(radius)
}
struct Component<'a> {
    offset: f64,
    offset_trig: &'a Interval,
    residual_trig: &'a Interval,
    add_depth: bool,
}
fn component(
    row: Component<'_>,
    depth: &Interval,
    delta: &[Interval],
    input: &Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let offset = arithmetic.scalar(row.offset)?;
    let shift = arithmetic.multiply(depth, row.offset_trig)?;
    let offset = if row.add_depth {
        arithmetic.add(&offset, &shift)?
    } else {
        arithmetic.subtract(&offset, &shift)?
    };
    let weighted = scaled(&input.capture.weights, &offset, arithmetic)?;
    let delta = scaled(delta, row.residual_trig, arithmetic)?;
    let negative = negate(&delta);
    let residual = algebra::difference(&weighted, &negative, arithmetic)?;
    Ok(arithmetic.divide(
        &algebra::magnitude(&residual),
        &minimum(&input.capture.weights),
    )?)
}

impl<'a> Component<'a> {
    fn radial(query: ConicalTrimQuery<'_>, input: &'a Bridge<'_, '_>) -> Self {
        Self {
            offset: query.carrier.radius,
            offset_trig: &input.domain.cosine,
            residual_trig: &input.domain.sine,
            add_depth: false,
        }
    }
    fn axial(query: ConicalTrimQuery<'_>, input: &'a Bridge<'_, '_>) -> Self {
        Self {
            offset: query.carrier.axial_origin,
            offset_trig: &input.domain.sine,
            residual_trig: &input.domain.cosine,
            add_depth: true,
        }
    }
}
fn negate(values: &[Interval]) -> Vec<Interval> {
    values
        .iter()
        .map(|v| Interval {
            lower: -v.upper.clone(),
            upper: -v.lower.clone(),
        })
        .collect()
}
fn minimum(values: &[Interval]) -> Interval {
    let minimum = polynomial::lower(values);
    Interval {
        lower: minimum.clone(),
        upper: minimum,
    }
}

pub(super) fn meridian_delta(
    v: &[Interval],
    input: &Bridge<'_, '_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, CylindricalProfileFailure> {
    let vw = polynomial::product(v, &input.capture.weights, arithmetic)?;
    algebra::difference(&vw, &input.capture.numerator, arithmetic)
}
