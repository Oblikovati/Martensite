//! Owning numeric storage never acquires a closing endpoint by proximity.
use super::super::closed_extrusion::{
    ClosedExtrusionInputError,
    input_view::{ExtrusionEndpointDomain, QuadraticSplineProofView},
    validation,
};

/// Immutable clamped quadratic source; `OpenQuadraticExtrusionInput::capture(p,w,k,m,d,v)?`.
#[derive(Clone, Debug)]
pub struct OpenQuadraticExtrusionInput {
    poles: Vec<[f64; 3]>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    multiplicities: Vec<u32>,
    direction: [f64; 3],
    interval: [f64; 2],
}
impl OpenQuadraticExtrusionInput {
    /// Own the complete original coefficients; `OpenQuadraticExtrusionInput::capture(p,w,k,m,d,v)?`.
    pub fn capture(
        poles: Vec<[f64; 3]>,
        weights: Vec<f64>,
        knots: Vec<f64>,
        multiplicities: Vec<u32>,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<Self, OpenExtrusionInputError> {
        Self {
            poles,
            weights,
            knots,
            multiplicities,
            direction,
            interval,
        }
        .validated()
    }
    fn validated(self) -> Result<Self, OpenExtrusionInputError> {
        validation::source(&self.proof_view()).map_err(OpenExtrusionInputError::from)?;
        Ok(self)
    }
    /// Read original coefficients; `input.poles()` cannot mutate the snapshot.
    pub fn poles(&self) -> &[[f64; 3]] {
        &self.poles
    }
    /// Read the unchanged positive weights; `input.weights()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Read original distinct knots; `input.knots()`.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }
    /// Read original multiplicities; `input.multiplicities()`.
    pub fn multiplicities(&self) -> &[u32] {
        &self.multiplicities
    }
    /// Read the unnormalized vector; `input.direction()` preserves its bits.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }
    /// Read the unchanged finite extrusion interval; `input.extrusion_interval()`.
    pub fn extrusion_interval(&self) -> [f64; 2] {
        self.interval
    }
    /// Read the complete curve interval including both endpoints; `input.curve_interval()`.
    pub fn curve_interval(&self) -> [f64; 2] {
        self.proof_view().curve_interval()
    }
    pub(super) fn proof_view(&self) -> QuadraticSplineProofView<'_> {
        QuadraticSplineProofView {
            poles: &self.poles,
            weights: &self.weights,
            knots: &self.knots,
            multiplicities: &self.multiplicities,
            direction: self.direction,
            interval: self.interval,
            domain: ExtrusionEndpointDomain::UnidentifiedEndpoints,
        }
    }
}

/// Invalid numeric storage, distinct from a valid input with an unfinished proof.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenExtrusionInputError {
    cause: ClosedExtrusionInputError,
}
impl OpenExtrusionInputError {
    /// Read the original typed offending field; `error.cause()`.
    pub fn cause(&self) -> &ClosedExtrusionInputError {
        &self.cause
    }
}
impl From<ClosedExtrusionInputError> for OpenExtrusionInputError {
    fn from(cause: ClosedExtrusionInputError) -> Self {
        Self { cause }
    }
}
impl std::fmt::Display for OpenExtrusionInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "open quadratic extrusion input={:?}; expected finite clamped degree-two storage, positive weights and nonzero direction",
            self.cause
        )
    }
}
impl std::error::Error for OpenExtrusionInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}
