//! A separate behavior policy keeps the closed endpoint quotient selectable.
use super::super::closed_extrusion::ClosedExtrusionPolicy;

/// Finite open-domain settings; `OpenExtrusionPolicy::bounded()` selects behavior one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenExtrusionPolicy {
    /// Explicit open-domain behavior, currently one.
    pub version: u32,
    /// Maximum original nonempty knot spans.
    pub max_spans: usize,
    /// Maximum scheduled self and distinct-pair obligations.
    pub max_obligations: usize,
    /// Global exact preparation and separation checkpoints.
    pub max_proof_steps: u64,
    /// Maximum retained exact numerator or denominator bit length.
    pub max_coefficient_bits: u64,
}
impl OpenExtrusionPolicy {
    /// Select the shared finite allowances without widening them; `OpenExtrusionPolicy::bounded()`.
    pub fn bounded() -> Self {
        let limits = ClosedExtrusionPolicy::bounded();
        Self {
            version: 1,
            max_spans: limits.max_spans,
            max_obligations: limits.max_obligations,
            max_proof_steps: limits.max_proof_steps,
            max_coefficient_bits: limits.max_coefficient_bits,
        }
    }
    pub(super) fn proof_policy(self) -> ClosedExtrusionPolicy {
        ClosedExtrusionPolicy {
            version: self.version,
            max_spans: self.max_spans,
            max_obligations: self.max_obligations,
            max_proof_steps: self.max_proof_steps,
            max_coefficient_bits: self.max_coefficient_bits,
        }
    }
}
