// SPDX-License-Identifier: GPL-2.0-only
//! Read immutable source-bound boundary measurements.
use super::*;
impl ConicalBoundaryEvidence {
    /// Certified total local-axis upper bound; `proof.total()`.
    pub fn total(&self) -> &DyadicBound {
        &self.total
    }
    /// Ideal-map upper bound before storage; `proof.ideal()`.
    pub fn ideal(&self) -> &DyadicBound {
        &self.ideal
    }
    /// Original-to-child cone error upper bound; `proof.original()`.
    pub fn original(&self) -> &DyadicBound {
        &self.original
    }
    /// Local-parameter angular derivative error for the supplied child and fit.
    /// This excludes original-to-child error; `proof.angular_derivative()`.
    pub fn angular_derivative(&self) -> &DyadicBound {
        &self.angular_derivative
    }
    /// Local-parameter angular derivative error between the original interval and rounded child.
    /// `proof.original_angular_derivative()`; zero if the child is the authored span.
    pub fn original_angular_derivative(&self) -> &DyadicBound {
        &self.original_angular_derivative
    }
    /// Combined angular derivative error in the child's local parameter.
    /// `proof.total_angular_derivative()` includes both separately reported components.
    pub fn total_angular_derivative(&self) -> &DyadicBound {
        &self.total_angular_derivative
    }
    /// Combined angular derivative error in the original authored parameter.
    /// `proof.authored_angular_derivative()` exactly applies the dyadic interval scale.
    pub fn authored_angular_derivative(&self) -> DyadicBound {
        self.total_angular_derivative
            .scale_pow2(self.authored_level)
    }
    /// Whether bounded arithmetic established equal original and child start points.
    /// This does not prove compatible angle lifts; `proof.exact_source_anchor()`.
    pub fn exact_source_anchor(&self) -> bool {
        self.anchor.exact
    }
    /// Whether the anchor joining segment stays within a certified principal-angle branch.
    /// This does not establish equal turn labels; `proof.source_anchor_same_branch()`.
    pub fn source_anchor_same_branch(&self) -> bool {
        self.anchor.turn_adjustment == Some(0)
    }
    /// Certified child-minus-original developed turns along the anchor segment.
    /// `None` means unresolved; `proof.source_anchor_turn_adjustment()`.
    pub fn source_anchor_turn_adjustment(&self) -> Option<i8> {
        self.anchor.turn_adjustment
    }
    /// First and last stored-edge error upper bounds; `proof.storage()`.
    pub fn storage(&self) -> &[DyadicBound; 2] {
        &self.storage
    }
    /// Charged work; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Original arithmetic policy; `proof.options()`.
    pub fn options(&self) -> ConeClearanceOptions {
        self.options
    }
    /// Compare every input bit with the retained query; `proof.matches(&query)`.
    pub fn matches(&self, query: &ConicalBoundaryQuery<'_>) -> bool {
        self.input.matches(query)
    }
}
