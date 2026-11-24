// SPDX-License-Identifier: GPL-2.0-only
//! Whole-domain cone normal-map bounds, conditional on an independently certified UV material.
use super::cylindrical_profile::{algebra, trigonometric};
use super::*;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as polynomial,
    bernstein_curve::{capture_planar, restrict},
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
mod bounds;
mod input;
mod source;
#[cfg(test)]
mod tests;

/// Exact authored interval corresponding to an actual retained polynomial UV span.
#[derive(Clone, Copy)]
pub struct ConicalMaterialSpan<'a> {
    /// Original authored polynomial before rounded subdivision.
    pub original: &'a [[f64; 2]],
    /// Exact dyadic restriction of the original.
    pub interval: CylindricalProfileInterval,
    /// Actual retained polynomial `(U,V)` controls, common to both caps.
    pub uv: &'a [[f64; 2]],
}
/// Normal-map inputs for one compact, independently certified UV region with holes.
#[derive(Clone, Copy)]
pub struct ConicalMaterialQuery<'a> {
    /// Source-keyed closed boundary correspondence, including every hole.
    pub spans: &'a [ConicalMaterialSpan<'a>],
    /// Actual decoded first and last analytic cap data.
    pub carriers: [ConicalTrimCarrier; 2],
    /// Positive slant reference used by `V=L-reference_slant`.
    pub reference_slant: f64,
    /// Physical angular datum; this leaf admits the canonical physical lift zero.
    pub chart_origin: f64,
    /// Full increasing signed outward normal depth interval.
    pub depths: [f64; 2],
    /// Requested whole-material spatial error.
    pub spatial_limit: f64,
    /// Remaining shared arithmetic allowance.
    pub options: ConeClearanceOptions,
}
/// Complete arithmetic receipt; compactness, closure and nesting belong to the caller.
#[derive(Debug)]
pub struct ConicalMaterialEvidence {
    bits: Vec<u64>,
    spatial: f64,
    operations: u32,
}
/// Invalid inputs or an incomplete whole-domain calculation with charged work.
#[derive(Debug)]
pub struct ConicalMaterialError {
    /// Index of the actual offending span, when known.
    pub span: Option<usize>,
    /// Original numerical cause; no sampled success is substituted.
    pub cause: CylindricalProfileFailure,
    /// Arithmetic work consumed before refusal.
    pub operations: u32,
}
impl ConicalMaterialEvidence {
    /// Outward full-cap material error; `let error = proof.spatial_error();`.
    pub fn spatial_error(&self) -> f64 {
        self.spatial
    }
    /// Charged arithmetic work; `let work = proof.operations();`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bind the entire original and actual input; `assert!(proof.matches(&query));`.
    pub fn matches(&self, query: &ConicalMaterialQuery<'_>) -> bool {
        self.bits == input::bits(query)
    }
}
/// Certify normal-map injectivity and filled-cap error over the compact UV material.
/// The caller must prove the supplied UV boundaries form the specified compact material.
/// Example: `let proof = certify_conical_material(query, || false)?;`.
pub fn certify_conical_material(
    query: ConicalMaterialQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalMaterialEvidence, ConicalMaterialError> {
    input::admit(&query).map_err(|cause| error(None, cause, 0))?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let spatial = bounds::certify(&query, &mut arithmetic)?;
    Ok(ConicalMaterialEvidence {
        bits: input::bits(&query),
        spatial,
        operations: arithmetic.operations(),
    })
}
fn error(
    span: Option<usize>,
    cause: CylindricalProfileFailure,
    operations: u32,
) -> ConicalMaterialError {
    ConicalMaterialError {
        span,
        cause,
        operations,
    }
}
impl std::fmt::Display for ConicalMaterialError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "conical material span={:?}: {} after {} operations",
            self.span, self.cause, self.operations
        )
    }
}
impl std::error::Error for ConicalMaterialError {}
