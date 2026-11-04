//! Owned numerical input excludes mutable evaluation and resolution caches.
use super::{SurfaceState, state::Direction};
use martensite_tkernel::standard::Failure;

/// Immutable tensor-spline coefficients and layout; `surface.try_input_lease()?.stored_input()?`.
#[derive(Clone, Debug, PartialEq)]
pub struct StoredSplineSurfaceInput {
    poles: Vec<[f64; 3]>,
    weights: Vec<f64>,
    shape: [usize; 2],
    weights_shape: [usize; 2],
    lower_bounds: [[i32; 2]; 2],
    axes: [StoredSplineSurfaceAxis; 2],
    rational: [bool; 2],
    evaluation_override: bool,
}
/// One immutable stored parameter direction; `input.axes()[0].knots()`.
#[derive(Clone, Debug, PartialEq)]
pub struct StoredSplineSurfaceAxis {
    knots: Vec<f64>,
    multiplicities: Vec<i32>,
    flat_knots: Vec<f64>,
    lower_bounds: [i32; 3],
    domain: [f64; 2],
    degree: i32,
    periodic: bool,
}
impl StoredSplineSurfaceInput {
    /// Read row-major poles without geometry handles; `input.poles()`.
    pub fn poles(&self) -> &[[f64; 3]] {
        &self.poles
    }
    /// Read actual stored weights, including nonrational storage; `input.weights()`.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
    /// Read the pole row/column counts; `input.shape()`.
    pub fn shape(&self) -> [usize; 2] {
        self.shape
    }
    /// Read the independently stored weight dimensions; `input.weights_shape()`.
    pub fn weights_shape(&self) -> [usize; 2] {
        self.weights_shape
    }
    /// Read pole and weight row/column origins; `input.lower_bounds()`.
    pub fn lower_bounds(&self) -> [[i32; 2]; 2] {
        self.lower_bounds
    }
    /// Read both stored parameter axes; `input.axes()[1].degree()`.
    pub fn axes(&self) -> &[StoredSplineSurfaceAxis; 2] {
        &self.axes
    }
    /// Read the evaluator's U/V rational flags; `input.rational()`.
    pub fn rational(&self) -> [bool; 2] {
        self.rational
    }
    /// Observe a delegated evaluator without replacing it; `input.evaluation_override()`.
    pub fn evaluation_override(&self) -> bool {
        self.evaluation_override
    }
}
impl StoredSplineSurfaceAxis {
    /// Read distinct stored knots; `axis.knots()`.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }
    /// Read actual stored multiplicities; `axis.multiplicities()`.
    pub fn multiplicities(&self) -> &[i32] {
        &self.multiplicities
    }
    /// Read the actual expanded knot vector; `axis.flat_knots()`.
    pub fn flat_knots(&self) -> &[f64] {
        &self.flat_knots
    }
    /// Read distinct/multiplicity/expanded array origins; `axis.lower_bounds()`.
    pub fn lower_bounds(&self) -> [i32; 3] {
        self.lower_bounds
    }
    /// Read the same bounds as the stored evaluator; `axis.domain()`.
    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }
    /// Read the stored polynomial degree; `axis.degree()`.
    pub fn degree(&self) -> i32 {
        self.degree
    }
    /// Read the actual periodic flag; `axis.periodic()`.
    pub fn periodic(&self) -> bool {
        self.periodic
    }
}
pub(super) fn sizes(state: &SurfaceState) -> [usize; 8] {
    [
        state.poles.len(),
        state.weights.len(),
        state.u.knots.len(),
        state.u.mults.len(),
        state.u.flat_knots.len(),
        state.v.knots.len(),
        state.v.mults.len(),
        state.v.flat_knots.len(),
    ]
}
pub(super) fn copy(state: &SurfaceState) -> Result<StoredSplineSurfaceInput, Failure> {
    Ok(StoredSplineSurfaceInput {
        poles: copied_poles(state),
        weights: state.weights.as_slice().to_vec(),
        shape: [state.poles.nb_rows(), state.poles.nb_columns()],
        weights_shape: [state.weights.nb_rows(), state.weights.nb_columns()],
        lower_bounds: [
            [state.poles.lower_row(), state.poles.lower_col()],
            [state.weights.lower_row(), state.weights.lower_col()],
        ],
        axes: [copy_axis(&state.u)?, copy_axis(&state.v)?],
        rational: [state.u_rational, state.v_rational],
        evaluation_override: !state.eval_rep.is_null(),
    })
}
fn copy_axis(direction: &Direction) -> Result<StoredSplineSurfaceAxis, Failure> {
    let (first, last) = direction.bounds()?;
    Ok(StoredSplineSurfaceAxis {
        knots: direction.knots.as_slice().to_vec(),
        multiplicities: direction.mults.as_slice().to_vec(),
        flat_knots: direction.flat_knots.as_slice().to_vec(),
        lower_bounds: [
            direction.knots.lower(),
            direction.mults.lower(),
            direction.flat_knots.lower(),
        ],
        domain: [first, last],
        degree: direction.deg,
        periodic: direction.periodic,
    })
}

fn copied_poles(state: &SurfaceState) -> Vec<[f64; 3]> {
    state
        .poles
        .as_slice()
        .iter()
        .map(|p| [p.x(), p.y(), p.z()])
        .collect()
}
