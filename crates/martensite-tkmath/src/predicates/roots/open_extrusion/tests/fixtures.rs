//! These curves have closed-form monotonicity or explicit crossing witnesses.
use super::*;
pub(super) fn capture(
    poles: Vec<[f64; 3]>,
    knots: Vec<f64>,
    mults: Vec<u32>,
) -> OpenQuadraticExtrusionInput {
    let weights = vec![1.; poles.len()];
    OpenQuadraticExtrusionInput::capture(poles, weights, knots, mults, [0., 0., 1.], [-1., 1.])
        .unwrap()
}
pub(super) fn parabola() -> OpenQuadraticExtrusionInput {
    capture(
        vec![[0., 0., 0.], [1., 1., 0.], [2., 0., 0.]],
        vec![0., 1.],
        vec![3, 3],
    )
}
pub(super) fn triangle(last_x: f64) -> OpenQuadraticExtrusionInput {
    let poles = vec![
        [1., 0., 0.],
        [2., 0., 0.],
        [3., 0., 0.],
        [2., 1., 0.],
        [1., 2., 0.],
        [1., 1., 0.],
        [last_x, 0., 0.],
    ];
    capture(poles, vec![0., 1., 2., 3.], vec![3, 2, 2, 3])
}
pub(super) fn as_closed(
    input: &OpenQuadraticExtrusionInput,
) -> super::super::super::closed_extrusion::ClosedQuadraticExtrusionInput {
    super::super::super::closed_extrusion::ClosedQuadraticExtrusionInput::capture(
        input.poles().to_vec(),
        input.weights().to_vec(),
        input.knots().to_vec(),
        input.multiplicities().to_vec(),
        input.direction(),
        input.extrusion_interval(),
    )
    .unwrap()
}
pub(super) fn crossing() -> OpenQuadraticExtrusionInput {
    capture(
        vec![
            [-1., -1., 0.],
            [0., 0., 0.],
            [1., 1., 0.],
            [0., 1., 0.],
            [-1., 1., 0.],
            [0., 0., 0.],
            [1., -1., 0.],
        ],
        vec![0., 1., 2., 3.],
        vec![3, 2, 2, 3],
    )
}
pub(super) fn weighted(
    weights: Vec<f64>,
) -> Result<OpenQuadraticExtrusionInput, OpenExtrusionInputError> {
    OpenQuadraticExtrusionInput::capture(
        vec![[0., 0., 0.], [1., 1., 0.], [2., 0., 0.]],
        weights,
        vec![0., 1.],
        vec![3, 3],
        [0., 0., 1.],
        [-1., 1.],
    )
}

pub(super) fn directed(poles: Vec<[f64; 3]>, direction: [f64; 3]) -> OpenQuadraticExtrusionInput {
    let weights = vec![1.; poles.len()];
    OpenQuadraticExtrusionInput::capture(
        poles,
        weights,
        vec![0., 1.],
        vec![3, 3],
        direction,
        [-1., 1.],
    )
    .unwrap()
}
