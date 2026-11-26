// SPDX-License-Identifier: GPL-2.0-only
//! Compare ordered finite point sets under one positive uniform scale and a translation.
use super::{NonFiniteCoordinate, exact_vector3};
use num_bigint::{BigInt, Sign};

/// First obstacle to an exact positive homothety.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Homothety3dFailure {
    /// Both point sets must supply the same count and at least two points.
    Count {
        /// Size of the first point set.
        first: usize,
        /// Size of the second point set.
        second: usize,
    },
    /// A supplied scalar was not finite.
    NonFinite(NonFiniteCoordinate),
    /// The first two points are coincident on the first set.
    CoincidentAnchors,
    /// The corresponding initial vector gives a zero or negative scale.
    NonpositiveScale,
    /// A corresponding point differs under the required homothety.
    UnmatchedPoint {
        /// Correspondence index of the first unequal point.
        index: usize,
    },
}

/// Prove the exact supplied point correspondence under one positive scale and translation.
/// Comparisons use the original binary64 values without rounded coordinate subtraction.
/// Example: `positive_homothety_3d(&first, &second)?`.
pub fn positive_homothety_3d(
    first: &[[f64; 3]],
    second: &[[f64; 3]],
) -> Result<(), Homothety3dFailure> {
    use Homothety3dFailure as Cause;
    if first.len() < 2 || first.len() != second.len() {
        return Err(Cause::Count {
            first: first.len(),
            second: second.len(),
        });
    }
    let starts = [decode(first[0], 0)?, decode(second[0], first.len())?];
    let base = relative(first[1], 1, &starts[0])?;
    let mapped = relative(second[1], 1 + first.len(), &starts[1])?;
    let (from, to) = scale(&base, &mapped)?;
    for i in 1..first.len() {
        let x = relative(first[i], i, &starts[0])?;
        let y = relative(second[i], i + first.len(), &starts[1])?;
        if x.iter().zip(&y).any(|(x, y)| x * to != y * from) {
            return Err(Cause::UnmatchedPoint { index: i });
        }
    }
    Ok(())
}

fn decode(value: [f64; 3], index: usize) -> Result<[BigInt; 3], Homothety3dFailure> {
    exact_vector3(value, index).map_err(Homothety3dFailure::NonFinite)
}

fn relative(
    value: [f64; 3],
    index: usize,
    origin: &[BigInt; 3],
) -> Result<[BigInt; 3], Homothety3dFailure> {
    let mut point = decode(value, index)?;
    for (value, origin) in point.iter_mut().zip(origin) {
        *value -= origin;
    }
    Ok(point)
}

fn scale<'a>(
    x: &'a [BigInt; 3],
    y: &'a [BigInt; 3],
) -> Result<(&'a BigInt, &'a BigInt), Homothety3dFailure> {
    let Some((x, y)) = x.iter().zip(y).find(|(x, _)| x.sign() != Sign::NoSign) else {
        return Err(Homothety3dFailure::CoincidentAnchors);
    };
    if x.sign() != y.sign() {
        return Err(Homothety3dFailure::NonpositiveScale);
    }
    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_positive_homothety_checks_all_ordered_points_and_sign() {
        let first = [[2., -4., 0.], [2., -1., 0.], [5., -2., 0.]];
        let mut second = [[9., 3., 5.], [9., 9., 5.], [15., 7., 5.]];
        assert_eq!(positive_homothety_3d(&first, &second), Ok(()));
        second[2][1] = 7.0_f64.next_up();
        assert_eq!(
            positive_homothety_3d(&first, &second),
            Err(Homothety3dFailure::UnmatchedPoint { index: 2 })
        );
        second[1][1] = -1.;
        assert_eq!(
            positive_homothety_3d(&first, &second),
            Err(Homothety3dFailure::NonpositiveScale)
        );
    }
}
