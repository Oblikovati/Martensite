// SPDX-License-Identifier: GPL-2.0-only
//! Complete bit snapshots distinguish source provenance from geometric zero equality.
use super::*;
pub(super) fn key(query: &CapturedBezierPair, policy: BezierPairPolicy) -> Vec<u64> {
    let mut bits = vec![
        u64::from(query.is_self()),
        u64::from(policy.version),
        u64::from(policy.roots.version),
        u64::from(policy.roots.max_degree),
        u64::from(policy.roots.max_bisections),
        policy.roots.max_width.to_bits(),
        u64::from(policy.signs.version),
        policy.requested_world_error.to_bits(),
    ];
    match policy.signs.degree_limit {
        BezierPairSignAdmission::DerivedProjection => bits.extend([0, 0]),
        BezierPairSignAdmission::Explicit(limit) => bits.extend([1, u64::from(limit)]),
    }
    for curve in query.captured_curves() {
        curve_bits(curve, &mut bits);
    }
    bits
}
fn curve_bits(curve: BezierPairCurve<'_>, bits: &mut Vec<u64>) {
    bits.extend(curve.interval.map(f64::to_bits));
    bits.push(curve.controls.len() as u64);
    bits.extend(curve.controls.iter().flat_map(|p| p.map(f64::to_bits)));
    match curve.weights {
        None => bits.push(0),
        Some(weights) => {
            bits.extend([1, weights.len() as u64]);
            bits.extend(weights.iter().map(|v| v.to_bits()));
        }
    }
}
