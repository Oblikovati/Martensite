// SPDX-License-Identifier: GPL-2.0-only
//! The additive entry reuses the original source/denominator/diagonal preparation without invoking graph elimination.
use super::*;
type Checked<T> = Result<T, BezierPairExclusionFailure>;
pub(super) fn execute(
    query: &CapturedBezierPair,
    policy: BezierPairExclusionPolicy,
    budget: &PairWorkBudget,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<Box<ExclusionProof>>> {
    admit(query, policy)?;
    poll(cancelled)?;
    let (equations, domains) = super::super::operation::checked_equations(
        query.sources(),
        query.is_self(),
        policy.pair_policy.roots,
        budget,
        cancelled,
    )?;
    witness(query, policy, equations, domains, cancelled)
}

fn admit(query: &CapturedBezierPair, policy: BezierPairExclusionPolicy) -> Checked<()> {
    if policy.version != 1 {
        return Err(BezierPairExclusionFailure::Version(policy.version));
    }
    let pair = policy.pair_policy;
    if !(1..=2).contains(&pair.version) {
        return Err(BezierPairExclusionFailure::PairPolicyVersion(pair.version));
    }
    if !(1..=2).contains(&pair.signs.version) {
        return Err(BezierPairExclusionFailure::SignPolicy(
            BezierPairSignFailure::Version(pair.signs.version),
        ));
    }
    super::super::operation::validate_world_request(pair.requested_world_error)?;
    intervals(query, pair.roots)
}
fn intervals(
    query: &CapturedBezierPair,
    roots: crate::predicates::roots::RootIsolationOptions,
) -> Checked<()> {
    for (source_index, source) in query.sources().iter().enumerate() {
        crate::predicates::roots::validate_policy_interval(source.interval, roots).map_err(
            |cause| {
                super::super::operation_failure::source_domain_failure(
                    source_index,
                    BezierPairOperationFailure::Root(cause),
                )
            },
        )?;
    }
    Ok(())
}
fn degree_limit(policy: BezierPairExclusionPolicy) -> usize {
    let primary = policy.pair_policy.roots.max_degree;
    let limit = match policy.pair_policy.signs.degree_limit {
        BezierPairSignAdmission::DerivedProjection => primary,
        BezierPairSignAdmission::Explicit(limit) => primary.min(limit),
    };
    usize::from(limit)
}

fn witness(
    query: &CapturedBezierPair,
    policy: BezierPairExclusionPolicy,
    equations: PairedEquations,
    domains: [SourceCertificate; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<Box<ExclusionProof>>> {
    let rectangle = query.captured_curves().map(|curve| curve.interval);
    let limit = degree_limit(policy);
    for axis in 0..2 {
        let candidate = basis::certify(&equations.0[axis], rectangle, limit, cancelled)?;
        if let Some(witness) = candidate {
            return Ok(Some(Box::new(ExclusionProof {
                equations,
                domains,
                rectangle,
                axis,
                divided_self_diagonal: query.is_self(),
                witness,
            })));
        }
    }
    Ok(None)
}
