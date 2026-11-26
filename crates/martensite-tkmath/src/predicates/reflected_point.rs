// SPDX-License-Identifier: GPL-2.0-only
//! Compare a stored point against an exact plane reflection.
use super::{
    NonFiniteCoordinate, exact_coordinate, exact_vector3, roots::round_positive_ratio_binary64,
    triangle::dot3,
};
use num_bigint::{BigInt, Sign};
use std::{cmp::Ordering, error::Error, fmt};

/// An operand or rounded result is outside the finite reflection domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReflectionComparisonError {
    /// An operand contains a nonfinite coordinate.
    NonFinite(NonFiniteCoordinate),
    /// The supplied plane normal has zero squared length.
    ZeroNormal,
    /// An exact reflected coordinate rounds beyond finite binary64.
    Unrepresentable {
        /// Zero-based coordinate index.
        axis: usize,
    },
}

impl fmt::Display for ReflectionComparisonError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(error) => error.fmt(output),
            Self::ZeroNormal => write!(
                output,
                "reflection normal [0, 0, 0] must have positive squared length"
            ),
            Self::Unrepresentable { axis } => write!(
                output,
                "reflected coordinate {axis} must round to a finite binary64 value"
            ),
        }
    }
}

impl Error for ReflectionComparisonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NonFinite(source) => Some(source),
            Self::ZeroNormal | Self::Unrepresentable { .. } => None,
        }
    }
}

impl From<NonFiniteCoordinate> for ReflectionComparisonError {
    fn from(error: NonFiniteCoordinate) -> Self {
        Self::NonFinite(error)
    }
}

/// Compare the Euclidean distance from `target` to the exact reflection of
/// `source` through the plane `[origin, normal]` with `|limit|`.
///
/// The supplied finite coordinates are exact binary64 values. Either normal sense
/// and any nonzero normal length define the same plane. Operand indices are source
/// 0, target 1, origin 2, normal 3, and limit 4.
/// Example: `compare_reflected_point_error_3d([1.,0.,0.],[-1.,0.,0.],[[0.;3],[1.,0.,0.]],0.)`.
pub fn compare_reflected_point_error_3d(
    source: [f64; 3],
    target: [f64; 3],
    plane: [[f64; 3]; 2],
    limit: f64,
) -> Result<Ordering, ReflectionComparisonError> {
    let source = exact_vector3(source, 0)?;
    let target = exact_vector3(target, 1)?;
    let reflection = ExactReflection::new(source, plane)?;
    let limit = exact_coordinate(limit, 4, 0)?;
    Ok(reflection.compare_target(&target, limit))
}

/// Round the exact reflection to binary64, then compare its Euclidean error with `|limit|`.
///
/// Changed coordinates round once, to nearest with ties to even. Unchanged coordinates
/// retain their supplied bits. Operand indices match [`compare_reflected_point_error_3d`].
/// Example: `round_reflected_point_3d([1.,0.,0.],[[0.;3],[1.,0.,0.]],0.)`.
pub fn round_reflected_point_3d(
    source: [f64; 3],
    plane: [[f64; 3]; 2],
    limit: f64,
) -> Result<([f64; 3], Ordering), ReflectionComparisonError> {
    let reflection = ExactReflection::new(exact_vector3(source, 0)?, plane)?;
    let limit = exact_coordinate(limit, 4, 0)?;
    let target = reflection.rounded(source)?;
    let error = reflection.compare_target(&exact_vector3(target, 1)?, limit);
    Ok((target, error))
}

struct ExactReflection {
    source: [BigInt; 3],
    denominator: BigInt,
    correction: [BigInt; 3],
}

impl ExactReflection {
    fn new(
        source: [BigInt; 3],
        [origin, normal]: [[f64; 3]; 2],
    ) -> Result<Self, ReflectionComparisonError> {
        let origin = exact_vector3(origin, 2)?;
        let normal = exact_vector3(normal, 3)?;
        let denominator = dot3(&normal, &normal);
        if denominator.sign() == Sign::NoSign {
            return Err(ReflectionComparisonError::ZeroNormal);
        }
        let local = std::array::from_fn(|axis| &source[axis] - &origin[axis]);
        let twice_projection = dot3(&normal, &local) << 1usize;
        let correction = normal.map(|value| value * &twice_projection);
        Ok(Self {
            source,
            denominator,
            correction,
        })
    }

    fn rounded(&self, mut target: [f64; 3]) -> Result<[f64; 3], ReflectionComparisonError> {
        for (axis, coordinate) in target.iter_mut().enumerate() {
            if self.correction[axis].sign() == Sign::NoSign {
                continue;
            }
            let numerator = &self.source[axis] * &self.denominator - &self.correction[axis];
            // The decoded point integers use 2^-1074 units. Restore that scale once.
            let denominator = &self.denominator << 1074usize;
            *coordinate = round_positive_ratio_binary64(numerator, denominator)
                .ok_or(ReflectionComparisonError::Unrepresentable { axis })?;
        }
        Ok(target)
    }

    fn compare_target(&self, target: &[BigInt; 3], limit: BigInt) -> Ordering {
        let limit = limit * &self.denominator;
        let error: [BigInt; 3] = std::array::from_fn(|axis| {
            (&target[axis] - &self.source[axis]) * &self.denominator + &self.correction[axis]
        });
        // All terms share the same dyadic scale after clearing the positive denominator.
        dot3(&error, &error).cmp(&(&limit * &limit))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ReflectionComparisonError, compare_reflected_point_error_3d as compare,
        round_reflected_point_3d as round,
    };
    use std::cmp::Ordering::{Equal, Greater, Less};

    #[test]
    fn oblique_plane_reflection_is_independent_of_normal_magnitude_and_sense() {
        // y + z = 1 reflects (x, y, z) to (x, 1 - z, 1 - y).
        let source = [3., 4., 5.];
        let target = [3., -4., -3.];
        for normal in [[0., 1., 1.], [0., -7., -7.], [0., 1e300, 1e300]] {
            let plane = [[8., 1., 0.], normal];
            assert_eq!(compare(source, target, plane, 0.), Ok(Equal));
            assert_eq!(compare(target, source, plane, 0.), Ok(Equal));
        }
    }

    #[test]
    fn nonunit_and_subnormal_normals_still_compare_exact_distances() {
        let normal = [1., 2., 2.];
        let source = [3., 0., 0.];
        let plane = [[0.; 3], normal];
        // Twice the source's plane distance is 2 * (3 / 3) = 2.
        assert_eq!(compare(source, source, plane, 2.), Ok(Equal));
        assert_eq!(
            compare(source, source, plane, 2_f64.next_down()),
            Ok(Greater)
        );
        let tiny = f64::from_bits(1);
        assert_eq!(
            compare([1., 0., 0.], [-1., 0., 0.], [[0.; 3], [tiny, 0., 0.]], 0.),
            Ok(Equal)
        );
    }

    #[test]
    fn lost_coordinate_bit_cannot_be_hidden_by_a_rounded_error_limit() {
        let large = 9_007_199_254_740_992.;
        let plane = [[-0.5, 0., 0.], [1., 0., 0.]];
        // Expected x is -(2^53 + 1), the midpoint between two representable points.
        let source = [large, 1., 2.];
        let target = [-large, 1., 2.];
        for (limit, expected) in [
            (1_f64.next_down(), Greater),
            (1., Equal),
            (1_f64.next_up(), Less),
        ] {
            assert_eq!(compare(source, target, plane, limit), Ok(expected));
        }
    }

    #[test]
    fn euclidean_limit_combines_error_in_all_axes() {
        let plane = [[0.; 3], [1., 0., 0.]];
        let source = [0.; 3];
        let target = [3., 4., 12.];
        assert_eq!(compare(source, target, plane, 13.), Ok(Equal));
        assert_eq!(
            compare(source, target, plane, 13_f64.next_down()),
            Ok(Greater)
        );
    }

    #[test]
    fn rational_coordinates_round_once_with_exact_error_status() {
        let plane = [[0.; 3], [1., 2., 2.]];
        let source = [3., 0., 0.];
        let expected = [7. / 3., -4. / 3., -4. / 3.];
        assert_eq!(round(source, plane, 0.), Ok((expected, Greater)));
        assert_eq!(round(source, plane, 1e-15), Ok((expected, Less)));

        let tiny = f64::from_bits(1);
        let (rounded, _) = round([tiny, 0., 0.], plane, tiny).unwrap();
        assert_eq!(
            rounded.map(f64::to_bits),
            [tiny, -0., -0.].map(f64::to_bits)
        );
    }

    #[test]
    fn nearest_reflection_distinguishes_signed_finite_and_overflow_midpoints() {
        let quarter_ulp = f64::from_bits((1023_u64 + 969) << 52);
        for sign in [1., -1.] {
            let source = [-sign * f64::MAX, 0., 0.];
            let normal = [sign, 0., 0.];
            let finite = [[sign * quarter_ulp / 2., 0., 0.], normal];
            let expected = [sign * f64::MAX, 0., 0.];
            assert_eq!(round(source, finite, quarter_ulp), Ok((expected, Equal)));
            assert_eq!(
                round(source, finite, quarter_ulp.next_down()),
                Ok((expected, Greater))
            );
            let overflow = [[sign * quarter_ulp, 0., 0.], normal];
            assert_eq!(
                round(source, overflow, f64::MAX),
                Err(ReflectionComparisonError::Unrepresentable { axis: 0 })
            );
        }
    }

    #[test]
    fn invalid_normal_and_nonfinite_inputs_are_typed() {
        assert_eq!(
            compare([0.; 3], [0.; 3], [[0.; 3]; 2], 0.),
            Err(ReflectionComparisonError::ZeroNormal)
        );
        let cause = compare(
            [0.; 3],
            [0., 0., f64::INFINITY],
            [[0.; 3], [1., 0., 0.]],
            0.,
        )
        .unwrap_err();
        let ReflectionComparisonError::NonFinite(error) = cause else {
            panic!("unexpected refusal {cause:?}");
        };
        assert_eq!(
            (error.point, error.axis, error.value),
            (1, 2, f64::INFINITY)
        );
    }
}
