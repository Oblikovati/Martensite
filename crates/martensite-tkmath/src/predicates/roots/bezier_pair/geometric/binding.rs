// SPDX-License-Identifier: GPL-2.0-only
//! Complete bit snapshots distinguish source provenance from geometric zero equality.
use super::*;
pub(super) fn policy_bits(policy: BezierPairPolicy) -> [u64; 9] {
    let [kind, limit] = match policy.signs.degree_limit {
        BezierPairSignAdmission::DerivedProjection => [0, 0],
        BezierPairSignAdmission::Explicit(limit) => [1, u64::from(limit)],
    };
    [
        u64::from(policy.version),
        u64::from(policy.roots.version),
        u64::from(policy.roots.max_degree),
        u64::from(policy.roots.max_bisections),
        policy.roots.max_width.to_bits(),
        u64::from(policy.signs.version),
        policy.requested_world_error.to_bits(),
        kind,
        limit,
    ]
}

pub(super) fn same_curve(left: BezierPairCurve<'_>, right: BezierPairCurve<'_>) -> bool {
    left.interval.map(f64::to_bits) == right.interval.map(f64::to_bits)
        && left.controls.len() == right.controls.len()
        && left
            .controls
            .iter()
            .zip(right.controls)
            .all(|(a, b)| a.map(f64::to_bits) == b.map(f64::to_bits))
        && same_weights(left.weights, right.weights)
}

fn same_weights(left: Option<&[f64]>, right: Option<&[f64]>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => left
            .iter()
            .map(|w| w.to_bits())
            .eq(right.iter().map(|w| w.to_bits())),
        _ => false,
    }
}
