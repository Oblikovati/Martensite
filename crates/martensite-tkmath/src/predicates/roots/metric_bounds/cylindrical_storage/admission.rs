// SPDX-License-Identifier: GPL-2.0-only
use super::{error::Checked, *};
use CylindricalStorageFailure as Failure;

pub(super) fn admit(query: &CylindricalStorageQuery<'_>) -> Checked<()> {
    admit_policy(query.options)?;
    for (depth, curve) in query.curves.iter().enumerate() {
        value(query.radii[depth], "radius", 0, true, Some(depth))?;
        admit_curve(*curve, depth, query.options.max_degree)?;
    }
    admit_limits(query.limits)
}

fn admit_policy(policy: CylindricalStorageOptions) -> Checked<()> {
    if !(8..=4096).contains(&policy.fractional_bits)
        || !(1..=25).contains(&policy.max_degree)
        || policy.max_operations > 1_000_000
    {
        return Err(Failure::Policy(policy));
    }
    Ok(())
}

fn admit_limits(limits: CylindricalStorageLimits) -> Checked<()> {
    let limits = [
        limits.radial_distance,
        limits.scaled_ruling,
        limits.height_drift,
        limits.normal_sweep,
    ];
    for (i, limit) in limits.into_iter().enumerate() {
        value(limit, "storage limit", i, false, None)?;
    }
    Ok(())
}

fn admit_curve(curve: MetricBezier<'_, 3>, depth: usize, limit: u8) -> Checked<()> {
    admit_counts(curve, depth, limit)?;
    for (i, pole) in curve.poles.iter().enumerate() {
        admit_pole(*pole, i, depth)?;
        value(
            curve.weights.map_or(1., |w| w[i]),
            "weight",
            i,
            true,
            Some(depth),
        )?;
    }
    Ok(())
}

fn admit_counts(curve: MetricBezier<'_, 3>, depth: usize, limit: u8) -> Checked<()> {
    if curve.poles.len() < 2 || curve.weights.is_some_and(|w| w.len() != curve.poles.len()) {
        return Err(Failure::PoleCount {
            depth,
            poles: curve.poles.len(),
            weights: curve.weights.map(<[f64]>::len),
        });
    }
    if curve.poles.len() - 1 > usize::from(limit) {
        return Err(Failure::Degree {
            depth,
            actual: curve.poles.len() - 1,
            limit,
        });
    }
    Ok(())
}

fn admit_pole(pole: [f64; 3], index: usize, depth: usize) -> Checked<()> {
    for (axis, value) in pole.into_iter().enumerate() {
        if !value.is_finite() {
            return Err(Failure::Value {
                depth: Some(depth),
                field: ["pole x", "pole y", "pole z"][axis],
                index,
                value,
            });
        }
    }
    Ok(())
}

fn value(
    value: f64,
    field: &'static str,
    index: usize,
    positive: bool,
    depth: Option<usize>,
) -> Checked<()> {
    if !value.is_finite() || value < 0. || (positive && value == 0.) {
        return Err(Failure::Value {
            depth,
            field,
            index,
            value,
        });
    }
    Ok(())
}

const STORAGE_RESIDUAL_ORDER: [CylindricalStorageResidual; 5] = [
    CylindricalStorageResidual::FirstRadius,
    CylindricalStorageResidual::LastRadius,
    CylindricalStorageResidual::ScaledRuling,
    CylindricalStorageResidual::HeightDrift,
    CylindricalStorageResidual::NormalSweep,
];

pub(super) fn accept(bounds: [f64; 5], limits: CylindricalStorageLimits) -> Checked<()> {
    let residuals = STORAGE_RESIDUAL_ORDER;
    let limits = [
        limits.radial_distance,
        limits.radial_distance,
        limits.scaled_ruling,
        limits.height_drift,
        limits.normal_sweep,
    ];
    for i in 0..5 {
        if bounds[i] > limits[i] {
            return Err(Failure::Uncertified {
                residual: residuals[i],
                bound: bounds[i],
                requested: limits[i],
            });
        }
    }
    Ok(())
}
