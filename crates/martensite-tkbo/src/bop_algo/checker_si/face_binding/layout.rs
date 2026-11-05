// SPDX-License-Identifier: GPL-2.0-only
use super::{capture_curves::unsupported, types::*};
pub(super) fn check(b: &OwnedSpline) -> Result<(), BindingFailure> {
    counts(b)?;
    coefficients(b)?;
    Ok(())
}
fn coefficients(b: &OwnedSpline) -> Result<(), BindingFailure> {
    let finite = finite_coefficients(b);
    let clamped = clamped(b);
    let knots = b.knots.windows(2).all(|p| p[0] < p[1])
        && b.domain == [b.knots[0], *b.knots.last().unwrap()];
    let weights = b.weights[0] > 0. && b.weights.iter().all(|w| *w == b.weights[0]);
    let closed = b.poles.first() == b.poles.last();
    if !finite
        || !clamped
        || !knots
        || !weights
        || !closed
        || b.multiplicities.iter().map(|m| i64::from(*m)).sum::<i64>() != b.poles.len() as i64 + 3
    {
        return Err(unsupported(
            "closed quadratic coefficients",
            format!(
                "finite={finite}, clamped={clamped}, knots={knots}, constant_positive_weights={weights}, closed={closed}; expected closed clamped complete layout"
            ),
        ));
    }
    Ok(())
}

fn counts(b: &OwnedSpline) -> Result<(), BindingFailure> {
    let valid = b.degree == 2
        && !b.periodic
        && b.poles.len() >= 3
        && b.weights.len() == b.poles.len()
        && b.knots.len() >= 2
        && b.knots.len() == b.multiplicities.len();
    if !valid {
        return Err(unsupported(
            "closed quadratic layout",
            format!(
                "degree={}, poles={}, weights={}, knots={}, multiplicities={}, periodic={}; expected complete clamped quadratic",
                b.degree,
                b.poles.len(),
                b.weights.len(),
                b.knots.len(),
                b.multiplicities.len(),
                b.periodic
            ),
        ));
    }
    Ok(())
}

fn clamped(b: &OwnedSpline) -> bool {
    b.multiplicities[0] == 3
        && *b.multiplicities.last().unwrap() == 3
        && b.multiplicities[1..b.multiplicities.len() - 1]
            .iter()
            .all(|m| *m > 0 && *m <= 2)
}

fn finite_coefficients(b: &OwnedSpline) -> bool {
    b.poles
        .iter()
        .flatten()
        .chain(b.weights.iter())
        .chain(b.knots.iter())
        .all(|v| v.is_finite())
}
