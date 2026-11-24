// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn admit(query: &ConicalMaterialQuery<'_>) -> Result<(), CylindricalProfileFailure> {
    if !(8..=4096).contains(&query.options.fractional_bits)
        || query.options.max_operations > 1_000_000
    {
        return Err(value(
            "material arithmetic policy",
            f64::from(query.options.max_operations),
        ));
    }
    if query.spans.is_empty() {
        return Err(value("material nonempty span count", 0.));
    }
    admit_scalars(query)?;
    admit_carriers(query)?;
    for span in query.spans {
        admit_span(span)?;
    }
    Ok(())
}
fn admit_scalars(query: &ConicalMaterialQuery<'_>) -> Result<(), CylindricalProfileFailure> {
    for (role, scalar) in scalars(query) {
        if !scalar.is_finite() {
            return Err(value(role, scalar));
        }
    }
    if query.reference_slant <= 0. || query.spatial_limit < 0. || query.depths[0] >= query.depths[1]
    {
        return Err(value(
            "positive reference, nonnegative limit and increasing depths",
            query.reference_slant,
        ));
    }
    Ok(())
}

fn admit_carriers(query: &ConicalMaterialQuery<'_>) -> Result<(), CylindricalProfileFailure> {
    let a = query.carriers[0].semi_angle;
    if a <= 0.
        || a >= std::f64::consts::FRAC_PI_2
        || a.to_bits() != query.carriers[1].semi_angle.to_bits()
    {
        return Err(value("common cone angle in (0,pi/2)", a));
    }
    Ok(())
}
fn admit_span(span: &ConicalMaterialSpan<'_>) -> Result<(), CylindricalProfileFailure> {
    if !(2..=4).contains(&span.original.len()) || !(2..=4).contains(&span.uv.len()) {
        return Err(value(
            "material source/UV pole counts in 2..=4",
            span.uv.len() as f64,
        ));
    }
    for scalar in span.original.iter().chain(span.uv).flatten() {
        if !scalar.is_finite() {
            return Err(value("finite material coefficient", *scalar));
        }
    }
    Ok(())
}
fn scalars(query: &ConicalMaterialQuery<'_>) -> Vec<(&'static str, f64)> {
    let mut values = vec![
        ("reference slant", query.reference_slant),
        ("chart origin", query.chart_origin),
        ("first depth", query.depths[0]),
        ("last depth", query.depths[1]),
        ("spatial limit", query.spatial_limit),
    ];
    for carrier in query.carriers {
        values.extend([
            ("carrier radius", carrier.radius),
            ("carrier height", carrier.axial_origin),
            ("carrier angle", carrier.semi_angle),
        ]);
    }
    values
}
pub(super) fn value(role: &'static str, value: f64) -> CylindricalProfileFailure {
    CylindricalProfileFailure::Value {
        role,
        index: 0,
        value,
    }
}
pub(super) fn bits(query: &ConicalMaterialQuery<'_>) -> Vec<u64> {
    let mut bits: Vec<_> = scalars(query)
        .into_iter()
        .map(|(_, value)| value.to_bits())
        .collect();
    bits.extend([
        u64::from(query.options.fractional_bits),
        u64::from(query.options.max_operations),
        query.spans.len() as u64,
    ]);
    for span in query.spans {
        span_bits(span, &mut bits);
    }
    bits
}
fn span_bits(span: &ConicalMaterialSpan<'_>, bits: &mut Vec<u64>) {
    bits.extend([
        span.original.len() as u64,
        span.uv.len() as u64,
        span.interval.index(),
        u64::from(span.interval.level()),
    ]);
    bits.extend(
        span.original
            .iter()
            .chain(span.uv)
            .flatten()
            .map(|value| value.to_bits()),
    );
}
