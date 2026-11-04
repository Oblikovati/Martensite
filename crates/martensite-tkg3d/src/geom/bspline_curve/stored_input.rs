//! Owned stored spline fields, without evaluator admission or support construction.
use super::BSplineState;
mod admission;
mod open_admission;
pub use open_admission::{StoredOpenExtrusionFailure, StoredOpenExtrusionInput};

/// Immutable stored spline input; obtain it with `curve.try_input_lease()?.stored_input()`.
#[derive(Clone, Debug, PartialEq)]
pub struct StoredSplineInput {
    poles: Vec<[f64; 3]>,
    weights: Vec<f64>,
    knots: Vec<f64>,
    multiplicities: Vec<i32>,
    flat_knots: Vec<f64>,
    domain: [f64; 2],
    degree: i32,
    periodic: bool,
    rational: bool,
    evaluation_override: bool,
    lower_bounds: [i32; 5],
}

impl StoredSplineInput {
    /// Read the captured poles; `input.poles()`.
    pub fn poles(&self) -> &[[f64; 3]] {
        &self.poles
    }
    /// Read the captured weights; `input.weights()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Read the captured knots; `input.knots()`.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }
    /// Read the captured multiplicities; `input.multiplicities()`.
    pub fn multiplicities(&self) -> &[i32] {
        &self.multiplicities
    }
    /// Read the captured flat knots; `input.flat_knots()`.
    pub fn flat_knots(&self) -> &[f64] {
        &self.flat_knots
    }
    /// Read the captured domain; `input.domain()`.
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }
    /// Read the captured degree; `input.degree()`.
    pub fn degree(&self) -> i32 {
        self.degree
    }
    /// Read the captured periodic; `input.periodic()`.
    pub fn periodic(&self) -> bool {
        self.periodic
    }
    /// Read the captured rational; `input.rational()`.
    pub fn rational(&self) -> bool {
        self.rational
    }
    /// Read the captured evaluation override; `input.evaluation_override()`.
    pub fn evaluation_override(&self) -> bool {
        self.evaluation_override
    }
    /// Read the captured lower bounds; `input.lower_bounds()`.
    pub fn lower_bounds(&self) -> [i32; 5] {
        self.lower_bounds
    }
}

pub(super) fn stored_sizes(state: &BSplineState) -> [usize; 5] {
    [
        state.poles.len(),
        state.weights.len(),
        state.knots.len(),
        state.mults.len(),
        state.flat_knots.len(),
    ]
}

pub(super) fn copy_stored_input(state: &BSplineState) -> StoredSplineInput {
    StoredSplineInput {
        poles: copied_poles(state),
        weights: state.weights.as_slice().to_vec(),
        knots: state.knots.as_slice().to_vec(),
        multiplicities: state.mults.as_slice().to_vec(),
        flat_knots: state.flat_knots.as_slice().to_vec(),
        domain: [state.first_parameter(), state.last_parameter()],
        degree: state.deg,
        periodic: state.periodic,
        rational: state.rational,
        evaluation_override: !state.eval_rep.is_null(),
        lower_bounds: [
            state.poles.lower(),
            state.weights.lower(),
            state.knots.lower(),
            state.mults.lower(),
            state.flat_knots.lower(),
        ],
    }
}

pub(super) fn copied_poles(state: &BSplineState) -> Vec<[f64; 3]> {
    state.poles.iter().map(|p| [p.x(), p.y(), p.z()]).collect()
}
