//! Explicit aggregate policies leave each exact primitive's finite ceiling unchanged.
use super::*;

/// Select the aggregate work admission before executing a captured cone family.
/// Example: `let policy = WrappedConeArithmeticGrantPolicy::InputBoundV1;`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WrappedConeArithmeticGrantPolicy {
    /// Preserve the original one-million aggregate allowance ceiling.
    #[default]
    OriginalV1,
    /// Admit the requested aggregate against captured original spans and fit level.
    /// The ceiling is `min(16_000_000, max(1_000_000, spans * (level + 1) * 4096))`.
    InputBoundV1,
    /// Scale by captured span count, fit level and an outward physical-angle extent.
    /// The cap is `clamp((2048 * spans + ceil(10000 * radians)) * (level + 1), 1M, 16M)`.
    PhysicalAngleBoundV2,
}

/// Immutable admission facts; these do not promise completion or a CPU-time bound.
/// Example: `assert!(result.arithmetic_grant_receipt().unwrap().requested_operations > 0);`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrappedConeArithmeticGrantReceipt {
    /// Executed, explicitly selected aggregate policy.
    pub policy: WrappedConeArithmeticGrantPolicy,
    /// Original selected span occurrences, before fitting or refinement.
    pub source_spans: u32,
    /// Original admitted maximum fit level.
    pub maximum_fit_level: u8,
    /// Base allowance per source span and fit level for the selected policy.
    pub operations_per_span_level: u32,
    /// Caller-requested aggregate; admission never replaces or clamps it.
    pub requested_operations: u32,
    /// Input-derived ceiling admitted before charged work begins.
    pub input_ceiling: u32,
    /// Absolute ceiling for this aggregate policy version.
    pub hard_ceiling: u32,
    /// Maximum allowance any individual exact primitive can receive.
    pub primitive_ceiling: u32,
}

impl WrappedConeNormalResult {
    /// Source-bound aggregate admission retained with the immutable completed result.
    /// Original selections return `None`; example: `result.arithmetic_grant_receipt()`.
    pub fn arithmetic_grant_receipt(&self) -> Option<&WrappedConeArithmeticGrantReceipt> {
        self.arithmetic_grant.as_ref().map(|grant| &grant.receipt)
    }
}
