// SPDX-License-Identifier: GPL-2.0-only
use super::*;
/// Immutable geometric attempt with original complete source and policy provenance.
#[derive(Debug)]
pub struct BezierPairGeometricReport {
    pub(super) source: CapturedBezierPair,
    pub(super) policy: BezierPairPolicy,
    pub(super) outcome: Checked<Option<GeometricProof>>,
}
impl BezierPairGeometricReport {
    /// Borrow the independently owned input capture; `report.source().captured_curves()`.
    pub fn source(&self) -> &CapturedBezierPair {
        &self.source
    }
    /// Read the unchanged requested solver/fallback settings; `report.policy().version`.
    pub fn policy(&self) -> BezierPairPolicy {
        self.policy
    }
    /// Geometric predicates perform no root subdivision; `assert_eq!(report.work().completed,0)`.
    pub fn work(&self) -> BezierPairWork {
        BezierPairWork {
            requested: self.policy.roots.max_bisections,
            completed: 0,
        }
    }
    /// Distinguish complete coverage from unsupported, cancelled or invalid input; `report.completion()`.
    pub fn completion(&self) -> BezierPairGeometricCompletion {
        match &self.outcome {
            Ok(Some(_)) => BezierPairGeometricCompletion::Complete,
            Ok(None) => BezierPairGeometricCompletion::Inconclusive,
            Err(BezierPairGeometricFailure::Root(RootIsolationError::Cancelled)) => {
                BezierPairGeometricCompletion::Cancelled
            }
            Err(_) => BezierPairGeometricCompletion::Invalid,
        }
    }
    /// Borrow genuine complete evidence only; `report.complete().map(|proof|proof.kind())`.
    pub fn complete(&self) -> Option<BezierPairGeometricCertificate<'_>> {
        Some(BezierPairGeometricCertificate {
            source: &self.source,
            proof: self.outcome.as_ref().ok()?.as_ref()?,
        })
    }
    /// Read the original typed admission or cancellation cause; `report.failure()`.
    pub fn failure(&self) -> Option<&BezierPairGeometricFailure> {
        self.outcome.as_ref().err()
    }
    /// Rebind every source, mode and policy bit; `assert!(report.matches_source(&query,policy))`.
    pub fn matches_source(&self, query: &CapturedBezierPair, policy: BezierPairPolicy) -> bool {
        self.matches_curves(query.captured_curves(), query.is_self(), policy)
    }

    /// Compare ordered source curves and policy bits directly. Pass both copies of
    /// the source for a self domain. Example: `report.matches_curves([a, b], false, policy)`.
    pub fn matches_curves(
        &self,
        curves: [BezierPairCurve<'_>; 2],
        self_off_diagonal: bool,
        policy: BezierPairPolicy,
    ) -> bool {
        self.source.is_self() == self_off_diagonal
            && binding::policy_bits(self.policy) == binding::policy_bits(policy)
            && self
                .source
                .captured_curves()
                .into_iter()
                .zip(curves)
                .all(|(a, b)| binding::same_curve(a, b))
    }
}
