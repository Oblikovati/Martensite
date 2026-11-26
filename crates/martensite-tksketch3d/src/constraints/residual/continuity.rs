// SPDX-License-Identifier: GPL-2.0-only
//! Differentiate exact endpoint stencils and a moving reference frame.
use super::{
    DualNumber, FrameReference, ResidualError, Vector, checked_length, components, directed, unit,
    vector,
};
use crate::constraints::ParallelDirection;
use martensite_tkmath::predicates::roots::{
    CubicEndJet, CubicEndStencil, CubicFitEnd, CubicFitTangent,
};

pub(super) fn evaluate(
    inputs: &[DualNumber],
    line: &Vector,
    reference: FrameReference,
    cubic_reference: bool,
    curvature_scale: Option<f64>,
) -> Result<Vec<DualNumber>, ResidualError> {
    let g2 = curvature_scale.is_some();
    let adjusted = endpoint(inputs, [3, 6, 9], g2, 1.)?;
    let incoming = if cubic_reference {
        endpoint(inputs, [3, 0, 12], g2, -1.)?
    } else {
        Endpoint {
            tangent: unit(line)?,
            curvature: None,
        }
    };
    let basis = tangent_basis(reference, &incoming.tangent, &adjusted.tangent)?;
    let mut rows = components(&basis, &adjusted.tangent)?.to_vec();
    if let (Some(scale), Some(mut curvature)) = (curvature_scale, adjusted.curvature) {
        if let Some(reference_curvature) = incoming.curvature {
            curvature = curvature.minus(&reference_curvature)?;
        }
        let [a, b] = components(&basis, &curvature)?;
        rows.extend([a.scaled(scale)?, b.scaled(scale)?]);
    }
    Ok(rows)
}

pub(super) fn evaluate_fit(
    inputs: &[DualNumber],
    line: &Vector,
    reference: FrameReference,
    tangent: CubicFitTangent,
    weights: [f64; 4],
) -> Result<Vec<DualNumber>, ResidualError> {
    let offsets = match tangent.end() {
        CubicFitEnd::Start => [3, 6, 9, 12],
        CubicFitEnd::End => [12, 9, 6, 3],
    };
    let points = offsets.map(|start| std::array::from_fn(|axis| inputs[start + axis].value()));
    let value = tangent
        .velocity(points)
        .map_err(|cause| ResidualError::Domain(cause.to_string()))?;
    let stencil = std::array::from_fn(|index| (offsets[index], weights[index]));
    let adjusted = unit(&inject::<4>(value, inputs, stencil)?)?;
    let basis = tangent_basis(reference, &unit(line)?, &adjusted)?;
    Ok(components(&basis, &adjusted)?.into())
}

pub(super) fn evaluate_arc(
    inputs: &[DualNumber],
    line: &Vector,
    reference: FrameReference,
) -> Result<Vec<DualNumber>, ResidualError> {
    let selected = vector(inputs, 3)?;
    let chord = vector(inputs, 6)?.minus(&selected)?;
    let through = vector(inputs, 9)?.minus(&selected)?;
    let adjusted = arc_direction(chord, through)?;
    let basis = tangent_basis(reference, &unit(line)?, &adjusted)?;
    Ok(components(&basis, &adjusted)?.into())
}

fn arc_direction(chord: Vector, through: Vector) -> Result<Vector, ResidualError> {
    let scale = chord
        .components()
        .iter()
        .chain(through.components())
        .map(|value| value.value().abs())
        .fold(0., f64::max);
    let inverse = scale.recip();
    if !scale.is_finite() || scale <= 0. || !inverse.is_finite() {
        return Err(ResidualError::Domain(format!(
            "arc tangent scale {scale}: expected a positive finite reciprocal"
        )));
    }
    // A common positive scale cancels when the homogeneous tangent is normalized.
    // Holding this scale constant in the differential gives the same direction derivative.
    let chord = chord.scaled(inverse)?;
    let through = through.scaled(inverse)?;
    let chord_term = chord.product(&through.dot(&through)?)?;
    let through_term = through.product(&chord.dot(&chord)?)?;
    unit(&through_term.minus(&chord_term)?)
}

fn tangent_basis(
    reference: FrameReference,
    incoming: &Vector,
    outgoing: &Vector,
) -> Result<[Vector; 2], ResidualError> {
    directed(incoming, outgoing, ParallelDirection::Same)?;
    reference.basis(incoming, "curve continuity")
}

struct Endpoint {
    tangent: Vector,
    curvature: Option<Vector>,
}

fn endpoint(
    inputs: &[DualNumber],
    offsets: [usize; 3],
    g2: bool,
    sign: f64,
) -> Result<Endpoint, ResidualError> {
    let (velocity, acceleration) = derivatives(inputs, offsets, g2)?;
    let speed = checked_length(&velocity)?;
    let tangent = divide(&velocity.scaled(sign)?, &speed)?;
    let curvature = acceleration
        .map(|a| normal_curvature(&tangent, &speed, &a))
        .transpose()?;
    Ok(Endpoint { tangent, curvature })
}

fn derivatives(
    inputs: &[DualNumber],
    [join, adjacent, second]: [usize; 3],
    g2: bool,
) -> Result<(Vector, Option<Vector>), ResidualError> {
    // Both endpoint stencils read the same position and derivative columns at the join.
    let point = |offset: usize| std::array::from_fn(|axis| inputs[offset + axis].value());
    let stencil = if g2 {
        CubicEndStencil::Curvature([point(join), point(adjacent), point(second)])
    } else {
        CubicEndStencil::Tangent([point(join), point(adjacent)])
    };
    let jet = CubicEndJet::new(stencil)
        .and_then(|jet| jet.rounded())
        .map_err(|cause| ResidualError::Domain(cause.to_string()))?;
    let velocity = inject(jet.first, inputs, [(join, -3.), (adjacent, 3.)])?;
    let acceleration = jet
        .second
        .map(|value| inject(value, inputs, [(join, 6.), (adjacent, -12.), (second, 6.)]))
        .transpose()?;
    Ok((velocity, acceleration))
}

fn inject<const N: usize>(
    values: [f64; 3],
    inputs: &[DualNumber],
    stencil: [(usize, f64); N],
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
    stencil: [(usize, f64); N],
    axis: usize,
    column: usize,
) -> f64 {
    stencil
        .into_iter()
        .map(|(offset, coefficient)| {
            let gradient = inputs[offset + axis].gradient();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraints::residual::displacement;

    #[test]
    fn arc_tangent_out_of_plane_partial_has_the_expected_physical_scale() {
        let points = [-7., -4., 0., -4., 0., 0., 4., 0., 0., 0., 2., 0.];
        let seeded = DualNumber::seed(&points).unwrap();
        let line = displacement(&seeded, 0).unwrap();
        let frame = FrameReference::from_direction([3., 4., 0.]);
        let rows = evaluate_arc(&seeded, &line, frame).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.value().abs() < 1e-14));
        // For the first row the line frame is +Z, so varying W.z has
        // derivative |E-S|² / |T| = 64/160 at the exact arc.
        assert!((rows[0].gradient()[11] - 0.4).abs() < 1e-14);
    }
}
