// SPDX-License-Identifier: GPL-2.0-only
//! Allocate canonical error only after bounding stored placement and rounded evaluation.
use super::*;
mod evaluation;

/// Physical placement inputs; the caller must establish the declared point-domain bound.
/// Example: `FrameSpatialInput { origin: [0.;3], translation: [0.;3], point_norm: 12., spatial_limit: 1e-5 }`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameSpatialInput {
    /// Requested ideal world origin.
    pub origin: [f64; 3],
    /// Actual retained translation, not an assumed copy of the requested origin.
    pub translation: [f64; 3],
    /// Nonnegative upper bound on the Euclidean norm of both ideal and retained local points.
    pub point_norm: f64,
    /// Strictly positive original requested world-space error limit.
    pub spatial_limit: f64,
}
/// Owned conditional spatial certificate, including binary64 affine evaluation error.
/// Native admission, domain validity and decoding correspondence remain caller obligations.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedFrameSpatialBudget {
    frame: CertifiedFrameTransport,
    input: FrameSpatialInput,
    translation_error: f64,
    evaluation_error: f64,
    placement_error: f64,
    canonical_allowance: f64,
}
impl CertifiedFrameTransport {
    /// Reserve placement error and round the remaining canonical allocation downward.
    /// Evaluation is three products, two left-associated sums, then translation, with unit scale.
    /// Example: `let budget = proof.spatial_budget(input)?; let epsilon = budget.canonical_allowance();`.
    pub fn spatial_budget(&self, input: FrameSpatialInput) -> Checked<CertifiedFrameSpatialBudget> {
        let allocation = allocate(
            self.input.stored,
            self.stretch_bounds()[1],
            self.matrix_error_bound(),
            input,
        )?;
        Ok(CertifiedFrameSpatialBudget {
            frame: self.clone(),
            input,
            translation_error: allocation.translation_error,
            evaluation_error: allocation.evaluation_error,
            placement_error: allocation.placement_error,
            canonical_allowance: allocation.canonical_allowance,
        })
    }
}
impl CertifiedFrameSpatialBudget {
    /// Exact frame evidence used by this allocation; `budget.frame().input()`.
    pub fn frame(&self) -> &CertifiedFrameTransport {
        &self.frame
    }
    /// Exact owned domain, origin, translation and requested limit; `budget.input()`.
    pub fn input(&self) -> FrameSpatialInput {
        self.input
    }
    /// Translation-only Euclidean error bound; `budget.translation_error_bound()`.
    pub fn translation_error_bound(&self) -> f64 {
        self.translation_error
    }
    /// Bound on rounded affine evaluation, separate from coefficient error; `budget.evaluation_error_bound()`.
    pub fn evaluation_error_bound(&self) -> f64 {
        self.evaluation_error
    }
    /// Placement-only error, including translation and evaluation; `budget.placement_error_bound()`.
    pub fn placement_error_bound(&self) -> f64 {
        self.placement_error
    }
    /// Positive downward-rounded canonical allocation; `budget.canonical_allowance()`.
    pub fn canonical_allowance(&self) -> f64 {
        self.canonical_allowance
    }
    /// Outward world error using the complete canonical allowance; `budget.world_error_bound()?`.
    pub fn world_error_bound(&self) -> Checked<f64> {
        world_error(
            self.frame.stretch_bounds()[1],
            self.canonical_allowance,
            self.placement_error,
        )
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SpatialAllocation {
    pub translation_error: f64,
    pub evaluation_error: f64,
    pub placement_error: f64,
    pub canonical_allowance: f64,
}
pub(super) fn allocate(
    stored: [[f64; 3]; 3],
    stretch: f64,
    matrix_error: f64,
    input: FrameSpatialInput,
) -> Checked<SpatialAllocation> {
    admit_spatial(input)?;
    let translation_error = translation_error(input)?;
    let evaluation_error = evaluation::bound(stored, input)?;
    let placement_error =
        placement_error(matrix_error, input, translation_error, evaluation_error)?;
    let canonical_allowance = allocation(stretch, input, placement_error)?;
    Ok(SpatialAllocation {
        translation_error,
        evaluation_error,
        placement_error,
        canonical_allowance,
    })
}
pub(super) fn world_error(stretch: f64, canonical: f64, placement: f64) -> Checked<f64> {
    let canonical = ExactRatio::from_finite(canonical);
    let stretch = ExactRatio::from_finite(stretch);
    upper(
        &canonical
            .multiply(&stretch)
            .add(&ExactRatio::from_finite(placement)),
    )
}
fn admit_spatial(input: FrameSpatialInput) -> Checked<()> {
    admit_vector(input.origin, FrameTransportField::Origin, 0)?;
    admit_vector(input.translation, FrameTransportField::Translation, 0)?;
    if !input.point_norm.is_finite()
        || input.point_norm < 0.
        || !input.spatial_limit.is_finite()
        || input.spatial_limit <= 0.
    {
        return Err(FrameTransportError::SpatialInput {
            point_norm: input.point_norm,
            spatial_limit: input.spatial_limit,
        });
    }
    Ok(())
}
fn translation_error(input: FrameSpatialInput) -> Checked<f64> {
    let differences = std::array::from_fn(|i| {
        ExactRatio::from_finite(input.translation[i])
            .subtract(&ExactRatio::from_finite(input.origin[i]))
    });
    Ok(arithmetic::square_root(
        &arithmetic::dot(&differences, &differences),
        FrameTransportQuantity::SpatialBudget,
    )?[1])
}
fn placement_error(
    matrix_error: f64,
    input: FrameSpatialInput,
    translation: f64,
    evaluation: f64,
) -> Checked<f64> {
    let matrix = ExactRatio::from_finite(matrix_error);
    let domain = ExactRatio::from_finite(input.point_norm);
    upper(
        &matrix
            .multiply(&domain)
            .add(&ExactRatio::from_finite(translation))
            .add(&ExactRatio::from_finite(evaluation)),
    )
}
fn allocation(stretch: f64, input: FrameSpatialInput, placement: f64) -> Checked<f64> {
    let failure = || FrameTransportError::SpatialLimit {
        requested: input.spatial_limit,
        placement_upper: placement,
    };
    if placement >= input.spatial_limit {
        return Err(failure());
    }
    let remaining =
        ExactRatio::from_finite(input.spatial_limit).subtract(&ExactRatio::from_finite(placement));
    let canonical = remaining.divide(&ExactRatio::from_finite(stretch));
    let lower = arithmetic::bounds(&canonical, FrameTransportQuantity::SpatialBudget)?[0];
    if lower <= 0. {
        return Err(failure());
    }
    Ok(lower)
}
fn upper(value: &ExactRatio) -> Checked<f64> {
    Ok(arithmetic::bounds(value, FrameTransportQuantity::SpatialBudget)?[1])
}
#[cfg(test)]
mod tests;
