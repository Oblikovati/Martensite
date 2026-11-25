// SPDX-License-Identifier: GPL-2.0-only
//! Differentiate exact endpoint stencils and moving line-frame projections.
use super::{
    DualNumber, FrameReference, ResidualError, Vector, checked_length, components, directed, unit,
};
use crate::constraints::ParallelDirection;
use martensite_tkmath::predicates::roots::{CubicEndJet, CubicEndStencil};

pub(super) fn line_cubic(
    inputs: &[DualNumber],
    line: &Vector,
    reference: FrameReference,
    curvature_scale: Option<f64>,
) -> Result<Vec<DualNumber>, ResidualError> {
    let (velocity, acceleration) = derivatives(inputs, curvature_scale.is_some())?;
    let line = unit(line)?;
    let speed = checked_length(&velocity)?;
    let tangent = divide(&velocity, &speed)?;
    directed(&line, &tangent, ParallelDirection::Same)?;
    let basis = reference.basis(&line, "line/cubic continuity")?;
    let mut rows = components(&basis, &tangent)?.to_vec();
    if let (Some(scale), Some(acceleration)) = (curvature_scale, acceleration) {
        let curvature = normal_curvature(&tangent, &speed, &acceleration)?;
        let [a, b] = components(&basis, &curvature)?;
        rows.extend([a.scaled(scale)?, b.scaled(scale)?]);
    }
    Ok(rows)
}

fn derivatives(inputs: &[DualNumber], g2: bool) -> Result<(Vector, Option<Vector>), ResidualError> {
    // Binding keeps each of the line endpoint and local cubic controls in one slot.
    let point = |offset: usize| std::array::from_fn(|axis| inputs[offset + axis].value());
    let (join, adjacent) = (point(3), point(6));
    let stencil = if g2 {
        CubicEndStencil::Curvature([join, adjacent, point(9)])
    } else {
        CubicEndStencil::Tangent([join, adjacent])
    };
    let jet = CubicEndJet::new(stencil)
        .and_then(|jet| jet.rounded())
        .map_err(|cause| ResidualError::Domain(cause.to_string()))?;
    let velocity = inject(jet.first, inputs, [-3., 3.])?;
    let acceleration = jet
        .second
        .map(|value| inject(value, inputs, [6., -12., 6.]))
        .transpose()?;
    Ok((velocity, acceleration))
}

fn inject<const N: usize>(
    values: [f64; 3],
    inputs: &[DualNumber],
    stencil: [f64; N],
) -> Result<Vector, ResidualError> {
    let dimensions = inputs
        .iter()
        .map(|input| input.gradient().len())
        .max()
        .unwrap_or(0);
    let [x, y, z] = [0, 1, 2].map(|axis| {
        let gradient = (0..dimensions)
            .map(|column| partial(inputs, stencil, axis, column))
            .collect();
        DualNumber::with_gradient(values[axis], gradient)
    });
    Ok(Vector::new([x?, y?, z?])?)
}

fn partial<const N: usize>(
    inputs: &[DualNumber],
    stencil: [f64; N],
    axis: usize,
    column: usize,
) -> f64 {
    stencil
        .into_iter()
        .enumerate()
        .map(|(control, coefficient)| {
            let gradient = inputs[3 * (control + 1) + axis].gradient();
            gradient.get(column).map_or(0., |entry| coefficient * entry)
        })
        .sum()
}

fn normal_curvature(
    tangent: &Vector,
    speed: &DualNumber,
    acceleration: &Vector,
) -> Result<Vector, ResidualError> {
    let tangential = tangent.product(&tangent.dot(acceleration)?)?;
    // Two divisions avoid forming speed squared, which can overflow for finite jets.
    divide(&divide(&acceleration.minus(&tangential)?, speed)?, speed)
}

fn divide(vector: &Vector, divisor: &DualNumber) -> Result<Vector, ResidualError> {
    let [x, y, z] = vector
        .components()
        .each_ref()
        .map(|value| value.quotient(divisor));
    Ok(Vector::new([x?, y?, z?])?)
}
