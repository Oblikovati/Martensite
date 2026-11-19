// SPDX-License-Identifier: GPL-2.0-only
//! Numerical proof refinement is explicit and independent of geometric identity.
/// Select primitive hull refinement without changing existing entry-point behavior.
/// Example: `let refinement = ConicalPrimitiveRefinement::HalfSplitV1;`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ConicalPrimitiveRefinement {
    /// Original complete-interval coefficient hull and absolute coefficient mean.
    #[default]
    UnsplitV1,
    /// One charged midpoint split of the already computed signed primitive.
    HalfSplitV1,
}

impl ConicalPrimitiveRefinement {
    pub(super) fn tag(self) -> u64 {
        match self {
            Self::UnsplitV1 => 0,
            Self::HalfSplitV1 => 1,
        }
    }
}
