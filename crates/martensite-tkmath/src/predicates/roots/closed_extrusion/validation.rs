//! Storage admission uses exact input ordering and no modelling tolerance.
use super::{
    input::ClosedQuadraticExtrusionInput as Input, input_view::QuadraticSplineProofView as View,
    types::ClosedExtrusionInputError as Error,
};

pub(super) fn input(input: &Input) -> Result<(), Error> {
    source(&input.proof_view())?;
    let first = input.poles[0];
    let last = input.poles[input.poles.len() - 1];
    if first != last {
        return Err(Error::OpenEndpoints { first, last });
    }
    Ok(())
}

pub(in crate::predicates::roots) fn source(input: &View<'_>) -> Result<(), Error> {
    storage(input)?;
    numeric(input)?;
    for (index, pair) in input.knots.windows(2).enumerate() {
        interval("knots", index, [pair[0], pair[1]])?;
    }
    interval("extrusion", 0, input.interval)?;
    if input.direction.iter().all(|value| *value == 0.) {
        return Err(Error::ZeroDirection(input.direction));
    }
    Ok(())
}

fn storage(input: &View<'_>) -> Result<(), Error> {
    let m = &input.multiplicities;
    let shape = input.poles.len() >= 3
        && input.weights.len() == input.poles.len()
        && input.knots.len() >= 2
        && m.len() == input.knots.len();
    let counts = m
        .iter()
        .try_fold(0usize, |sum, n| sum.checked_add(*n as usize));
    if shape && clamped(m) && counts == input.poles.len().checked_add(3) {
        return Ok(());
    }
    Err(Error::Storage {
        poles: input.poles.len(),
        weights: input.weights.len(),
        knots: input.knots.len(),
        multiplicities: m.len(),
    })
}

fn clamped(m: &[u32]) -> bool {
    m.first() == Some(&3)
        && m.last() == Some(&3)
        && m.iter()
            .skip(1)
            .take(m.len().saturating_sub(2))
            .all(|n| (1..=2).contains(n))
}

fn numeric(input: &View<'_>) -> Result<(), Error> {
    for (index, value) in input.poles.iter().flatten().enumerate() {
        finite("poles", index, *value)?;
    }
    for (index, value) in input.knots.iter().enumerate() {
        finite("knots", index, *value)?;
    }
    for (index, value) in input.direction.iter().enumerate() {
        finite("direction", index, *value)?;
    }
    weights(input.weights)
}

fn weights(weights: &[f64]) -> Result<(), Error> {
    for (index, value) in weights.iter().enumerate() {
        finite("weights", index, *value)?;
        if *value <= 0. {
            return Err(Error::Numeric {
                field: "weights",
                index,
                value: *value,
            });
        }
    }
    Ok(())
}

fn finite(field: &'static str, index: usize, value: f64) -> Result<(), Error> {
    if !value.is_finite() {
        return Err(Error::Numeric {
            field,
            index,
            value,
        });
    }
    Ok(())
}

fn interval(field: &'static str, index: usize, bounds: [f64; 2]) -> Result<(), Error> {
    let width = bounds[1] - bounds[0];
    if bounds.iter().all(|v| v.is_finite())
        && width > 0.
        && width.is_finite()
        && (1. / width).is_finite()
    {
        return Ok(());
    }
    Err(Error::Interval {
        field,
        index,
        bounds,
    })
}
