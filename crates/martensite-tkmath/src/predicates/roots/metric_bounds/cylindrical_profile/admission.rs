// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn admit(query: &CylindricalProfileQuery<'_>) -> Checked<()> {
    let policy = query.options;
    if !(8..=4096).contains(&policy.fractional_bits) || policy.max_operations > 1_000_000 {
        return Err(CylindricalProfileFailure::Policy(policy));
    }
    count("authored poles", query.authored.len(), 4)?;
    count("stored poles", query.stored.poles.len(), 10)?;
    value("support radius", 0, query.support_radius, true)?;
    value("stored radius", 0, query.stored_radius, true)?;
    value("angular origin", 0, query.angular_origin, false)?;
    if !query.spatial_limit.is_finite() || query.spatial_limit < 0. {
        return Err(CylindricalProfileFailure::Value {
            role: "spatial limit",
            index: 0,
            value: query.spatial_limit,
        });
    }
    coordinates(*query)?;
    weights(query.stored)
}

pub(super) fn count(role: &'static str, actual: usize, maximum: usize) -> Checked<()> {
    if !(2..=maximum).contains(&actual) {
        return Err(CylindricalProfileFailure::Count { role, actual });
    }
    Ok(())
}

pub(super) fn value(role: &'static str, index: usize, value: f64, positive: bool) -> Checked<()> {
    if !value.is_finite() || (positive && value <= 0.) {
        return Err(CylindricalProfileFailure::Value { role, index, value });
    }
    Ok(())
}

fn coordinates(query: CylindricalProfileQuery<'_>) -> Checked<()> {
    for (index, coordinate) in query.authored.iter().flatten().enumerate() {
        value("authored coordinate", index, *coordinate, false)?;
    }
    for (index, coordinate) in query.stored.poles.iter().flatten().enumerate() {
        value("stored coordinate", index, *coordinate, false)?;
    }
    Ok(())
}

fn weights(curve: super::super::MetricBezier<'_, 3>) -> Checked<()> {
    let Some(weights) = curve.weights else {
        return Ok(());
    };
    if weights.len() != curve.poles.len() {
        return Err(CylindricalProfileFailure::Weights {
            poles: curve.poles.len(),
            weights: weights.len(),
        });
    }
    for (index, weight) in weights.iter().enumerate() {
        value("weight", index, *weight, true)?;
    }
    Ok(())
}
