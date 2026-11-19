// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn admit(query: &CapturedBezierPair, policy: BezierPairPolicy) -> Checked<bool> {
    policy_admission(policy)?;
    let mut supported = true;
    for (index, curve) in query.captured_curves().into_iter().enumerate() {
        super::super::super::checked_policy_interval(curve.interval, policy.roots)?;
        finite(curve, index)?;
        supported &= supported_curve(curve, policy);
    }
    Ok(supported)
}
fn finite(curve: BezierPairCurve<'_>, source_index: usize) -> Checked<()> {
    for (index, point) in curve.controls.iter().enumerate() {
        finite_point(*point, index, source_index)?;
    }
    for (index, value) in curve.weights.into_iter().flatten().enumerate() {
        if !value.is_finite() {
            return Err(source_error(
                source_index,
                BezierPairCaptureFailure::Weight {
                    index,
                    value: *value,
                },
            ));
        }
    }
    Ok(())
}
fn source_error(
    source_index: usize,
    cause: BezierPairCaptureFailure,
) -> BezierPairGeometricFailure {
    BezierPairGeometricFailure::Source {
        source_index,
        cause,
    }
}
fn supported_curve(curve: BezierPairCurve<'_>, policy: BezierPairPolicy) -> bool {
    (1..=4).contains(&curve.controls.len())
        && curve.controls.len() - 1 <= usize::from(policy.roots.max_degree)
        && curve.interval == [0., 1.]
        && curve
            .weights
            .is_none_or(|weights| weights.iter().all(|v| *v == 1.))
}

fn policy_admission(policy: BezierPairPolicy) -> Checked<()> {
    if !(1..=2).contains(&policy.version) {
        return Err(BezierPairGeometricFailure::PairPolicyVersion(
            policy.version,
        ));
    }
    if !(1..=2).contains(&policy.signs.version) {
        return Err(BezierPairGeometricFailure::SignPolicy(
            BezierPairSignFailure::Version(policy.signs.version),
        ));
    }
    if !policy.requested_world_error.is_finite() || policy.requested_world_error <= 0. {
        return Err(BezierPairGeometricFailure::WorldRequest(
            policy.requested_world_error,
        ));
    }
    Ok(())
}

fn finite_point(point: Point, index: usize, source_index: usize) -> Checked<()> {
    for (axis, value) in point.into_iter().enumerate() {
        if !value.is_finite() {
            let cause = NonFiniteCoordinate {
                point: index,
                axis,
                value,
            };
            return Err(source_error(
                source_index,
                BezierPairCaptureFailure::Coordinate(cause),
            ));
        }
    }
    Ok(())
}
