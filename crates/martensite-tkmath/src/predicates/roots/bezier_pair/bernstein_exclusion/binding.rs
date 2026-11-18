// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn same_policy(a: BezierPairExclusionPolicy, b: BezierPairExclusionPolicy) -> bool {
    let x = a.pair_policy;
    let y = b.pair_policy;
    a.version == b.version
        && x.version == y.version
        && x.signs == y.signs
        && x.roots.version == y.roots.version
        && x.roots.max_degree == y.roots.max_degree
        && x.roots.max_bisections == y.roots.max_bisections
        && x.roots.max_width.to_bits() == y.roots.max_width.to_bits()
        && x.requested_world_error.to_bits() == y.requested_world_error.to_bits()
}
pub(super) fn same_source(a: &CapturedBezierPair, b: &CapturedBezierPair) -> bool {
    a.is_self() == b.is_self()
        && a.captured_curves()
            .into_iter()
            .zip(b.captured_curves())
            .all(|(x, y)| same_curve(x, y))
}
fn same_curve(a: BezierPairCurve<'_>, b: BezierPairCurve<'_>) -> bool {
    a.interval.map(f64::to_bits) == b.interval.map(f64::to_bits)
        && a.controls.len() == b.controls.len()
        && a.controls
            .iter()
            .map(|p| p.map(f64::to_bits))
            .eq(b.controls.iter().map(|p| p.map(f64::to_bits)))
        && same_weights(a.weights, b.weights)
}
fn same_weights(a: Option<&[f64]>, b: Option<&[f64]>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            a.len() == b.len()
                && a.iter()
                    .map(|v| v.to_bits())
                    .eq(b.iter().map(|v| v.to_bits()))
        }
        _ => false,
    }
}
