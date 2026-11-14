// SPDX-License-Identifier: GPL-2.0-only
//! The same spatial allocation arithmetic serves the distinct rotated reference contract.
use super::super::spatial::{SpatialAllocation, allocate, world_error};
use super::*;
/// Owned point-domain allocation bound to the requested rotated reference frame.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedRotatedFrameSpatialBudget {
    frame: CertifiedRotatedFrameTransport,
    input: FrameSpatialInput,
    allocation: SpatialAllocation,
}
impl CertifiedRotatedFrameTransport {
    /// Reserve rotation, coefficient, translation and affine-evaluation error before allocation.
    /// Example: `let budget = proof.spatial_budget(input)?;`.
    pub fn spatial_budget(
        &self,
        input: FrameSpatialInput,
    ) -> Checked<CertifiedRotatedFrameSpatialBudget> {
        let allocation = allocate(
            self.input().stored,
            self.stretch_bounds()[1],
            self.matrix_error_bound(),
            input,
        )?;
        Ok(CertifiedRotatedFrameSpatialBudget {
            frame: self.clone(),
            input,
            allocation,
        })
    }
}
impl CertifiedRotatedFrameSpatialBudget {
    /// Complete rotation-aware coefficient evidence; `budget.frame().rotation().angle`.
    pub fn frame(&self) -> &CertifiedRotatedFrameTransport {
        &self.frame
    }
    /// Original point-domain, translation and spatial limit; `budget.input().spatial_limit`.
    pub fn input(&self) -> FrameSpatialInput {
        self.input
    }
    /// Certified translation-only Euclidean error; `budget.translation_error_bound()`.
    pub fn translation_error_bound(&self) -> f64 {
        self.allocation.translation_error
    }
    /// Rounded affine-evaluation error; `budget.evaluation_error_bound()`.
    pub fn evaluation_error_bound(&self) -> f64 {
        self.allocation.evaluation_error
    }
    /// Complete placement-only error relative to the rotated ideal reference.
    /// Example: `budget.placement_error_bound()`.
    pub fn placement_error_bound(&self) -> f64 {
        self.allocation.placement_error
    }
    /// Positive downward canonical allocation; `budget.canonical_allowance()`.
    pub fn canonical_allowance(&self) -> f64 {
        self.allocation.canonical_allowance
    }
    /// Outward world error using the complete canonical allowance; `budget.world_error_bound()?`.
    pub fn world_error_bound(&self) -> Checked<f64> {
        world_error(
            self.frame.stretch_bounds()[1],
            self.allocation.canonical_allowance,
            self.allocation.placement_error,
        )
    }
}
