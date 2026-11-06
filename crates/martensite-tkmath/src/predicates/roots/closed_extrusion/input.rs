//! Owned numeric inputs define one represented polynomial spline and one seam quotient.
use super::types::ClosedExtrusionInputError;
use super::validation;

#[derive(Clone, Debug)]
/// Immutable degree-two input. Later changes to a geometry handle are not covered.
pub struct ClosedQuadraticExtrusionInput {
    pub(super) poles: Vec<[f64; 3]>,
    pub(super) weights: Vec<f64>,
    pub(super) knots: Vec<f64>,
    pub(super) multiplicities: Vec<u32>,
    pub(super) direction: [f64; 3],
    pub(super) interval: [f64; 2],
}
impl ClosedQuadraticExtrusionInput {
    /// Own one closed clamped support; `ClosedQuadraticExtrusionInput::capture(p,w,k,m,d,v)?`.
    pub fn capture(
        poles: Vec<[f64; 3]>,
        weights: Vec<f64>,
        knots: Vec<f64>,
        multiplicities: Vec<u32>,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<Self, ClosedExtrusionInputError> {
        let input = Self {
            poles,
            weights,
            knots,
            multiplicities,
            direction,
            interval,
        };
        validation::input(&input)?;
        Ok(input)
    }
    /// Read owned support coordinates; `input.poles()[0]` cannot mutate the snapshot.
    pub fn poles(&self) -> &[[f64; 3]] {
        &self.poles
    }
    /// Read the exact supplied weights; `input.weights().len() == input.poles().len()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Read distinct supplied knots; `input.knots().windows(2)` preserves original values.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }
    /// Read the supplied multiplicities; `input.multiplicities()[0] == 3`.
    pub fn multiplicities(&self) -> &[u32] {
        &self.multiplicities
    }
    /// Read the unnormalized vector; `input.direction()` preserves its binary64 bits.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }
    /// Read the finite extrusion interval; `input.extrusion_interval()`.
    pub fn extrusion_interval(&self) -> [f64; 2] {
        self.interval
    }
    /// Read the full represented curve domain; `input.curve_interval()[0]`.
    pub fn curve_interval(&self) -> [f64; 2] {
        [self.knots[0], self.knots[self.knots.len() - 1]]
    }
    pub(super) fn proof_view(&self) -> super::input_view::QuadraticSplineProofView<'_> {
        super::input_view::QuadraticSplineProofView {
            poles: &self.poles,
            weights: &self.weights,
            knots: &self.knots,
            multiplicities: &self.multiplicities,
            direction: self.direction,
            interval: self.interval,
            domain: super::input_view::ExtrusionEndpointDomain::ClosedQuotient,
        }
    }
}
