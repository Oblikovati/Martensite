//! Storage validation binds direct Bézier extraction to the actual evaluator layout.
use super::types::{FaceQueryIncomplete as Cause, require};
use martensite_tkernel::ncollection::Array1;
use martensite_tkg2d::geom2d::spline_data::knot_data;
use martensite_tkg3d::geom::{StoredSplineSurfaceAxis as Axis, StoredSplineSurfaceInput as Input};
pub(super) fn validate(input: &Input, owner: usize) -> Result<(), Cause> {
    evaluator_layout(input, owner)?;
    validate_axes(input, owner)?;
    let shape = input.shape();
    let complete =
        input.poles().len() == shape[0] * shape[1] && input.weights().len() == input.poles().len();
    require(
        complete,
        owner,
        "complete row-major tensor storage",
        (shape, input.poles().len(), input.weights().len()),
    )
}
fn evaluator_layout(input: &Input, owner: usize) -> Result<(), Cause> {
    let admitted = !input.evaluation_override()
        && input.lower_bounds() == [[1; 2]; 2]
        && input.shape() == input.weights_shape();
    let actual = (
        input.evaluation_override(),
        input.lower_bounds(),
        input.shape(),
        input.weights_shape(),
    );
    require(
        admitted,
        owner,
        "stored tensor evaluator and matching one-based grids",
        actual,
    )
}
fn validate_axes(input: &Input, owner: usize) -> Result<(), Cause> {
    for (position, axis) in input.axes().iter().enumerate() {
        let expected = axis_size(axis, position, owner)?;
        require(
            input.shape()[position] == expected,
            owner,
            "degree/span pole layout",
            (position, input.shape()[position], expected),
        )?;
        validate_flat(axis, owner)?;
    }
    Ok(())
}
fn axis_size(axis: &Axis, position: usize, owner: usize) -> Result<usize, Cause> {
    let degree = axis.degree();
    require(
        (1..=2).contains(&degree) && axis.lower_bounds() == [1; 3],
        owner,
        "degree one or two and one-based knot layout",
        (degree, axis.lower_bounds()),
    )?;
    if axis.periodic() {
        return periodic_axis(axis, position, owner);
    }
    require(
        axis.knots().len() == 2 && axis.multiplicities() == [degree + 1; 2],
        owner,
        "one complete clamped nonperiodic span",
        (axis.knots(), axis.multiplicities()),
    )?;
    Ok(degree as usize + 1)
}
fn periodic_axis(axis: &Axis, position: usize, owner: usize) -> Result<usize, Cause> {
    require(
        position == 0
            && axis.degree() == 2
            && axis.knots().len() == 4
            && axis.multiplicities() == [2; 4],
        owner,
        "three-span quadratic U-periodic cycle",
        (position, axis.degree(), axis.knots(), axis.multiplicities()),
    )?;
    Ok(6)
}
fn validate_flat(axis: &Axis, owner: usize) -> Result<(), Cause> {
    validate_partition(axis, owner)?;
    let knots = Array1::from_shared(axis.knots(), 1);
    let mults = Array1::from_shared(axis.multiplicities(), 1);
    let prepared = knot_data(axis.degree(), axis.periodic(), &knots, &mults).map_err(|cause| {
        Cause::Geometry {
            owner,
            cause: Box::new(cause),
        }
    })?;
    require(
        same_bits(prepared.flat_knots.as_slice(), axis.flat_knots()),
        owner,
        "actual expanded knot vector and degree offsets",
        axis.flat_knots(),
    )
}
fn validate_partition(axis: &Axis, owner: usize) -> Result<(), Cause> {
    let ordered = axis
        .knots()
        .windows(2)
        .all(|p| p[0].is_finite() && p[1].is_finite() && p[0] < p[1]);
    require(
        ordered,
        owner,
        "finite nonempty ordered span partition",
        axis.knots(),
    )?;
    let domain = [axis.knots()[0], *axis.knots().last().unwrap()];
    require(
        same_bits(&domain, &axis.domain()),
        owner,
        "canonical bounds equal knot endpoints",
        (domain, axis.domain()),
    )
}
fn same_bits(left: &[f64], right: &[f64]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(a, b)| a.to_bits() == b.to_bits())
}
