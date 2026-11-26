// SPDX-License-Identifier: GPL-2.0-only
//! Directed D1 of the immutable affine interpolation model.
use super::{
    CubicFitNumericError, CubicFitOperator, DyadicBound, ExactRatio, affine_displacement,
    complete_weights, dyadic_point, dyadic_stations, nearest_binary64, round_weights,
};

/// End of the authored normalized cubic span. A tangent points into the curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CubicFitEnd {
    /// Parameter zero, using the forward derivative.
    Start,
    /// Parameter one, using the negative of the forward derivative.
    End,
}

/// One endpoint velocity of a four-station affine fit before pole materialization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicFitTangent {
    end: CubicFitEnd,
    interior: [f64; 3],
}

impl CubicFitOperator {
    /// Capture the interior pole row adjacent to the selected endpoint.
    /// Example: `operator.endpoint_tangent(CubicFitEnd::End)`.
    pub fn endpoint_tangent(&self, end: CubicFitEnd) -> CubicFitTangent {
        let index = usize::from(end == CubicFitEnd::End);
        CubicFitTangent {
            end,
            interior: self.interiors[index],
        }
    }
}

impl CubicFitTangent {
    /// Return the selected traversal end. Example: `tangent.end()`.
    pub const fn end(self) -> CubicFitEnd {
        self.end
    }

    /// Evaluate the real affine endpoint velocity, with one rounding per component.
    /// Stations use the original directed fit order. Example: `tangent.velocity(stations)?`.
    pub fn velocity(self, stations: [[f64; 3]; 4]) -> Result<[f64; 3], CubicFitNumericError> {
        let stations = dyadic_stations(stations)?;
        let coefficients = self.coefficients()?;
        let [x, y, z] = [0, 1, 2].map(|axis| {
            let value = affine_displacement(&stations, &coefficients, axis);
            nearest_binary64(&ExactRatio::from_dyadic(&value))
                .ok_or(CubicFitNumericError::Unbounded)
        });
        Ok([x?, y?, z?])
    }

    /// Round the four D1 sensitivity coefficients independently in authored station order.
    /// Example: `tangent.station_weights()?[1]`.
    pub fn station_weights(self) -> Result<[f64; 4], CubicFitNumericError> {
        let weights = self.coefficients()?.each_ref().map(ExactRatio::from_dyadic);
        round_weights(complete_weights(weights, 1))
    }

    fn coefficients(self) -> Result<[DyadicBound; 3], CubicFitNumericError> {
        let three = DyadicBound::new(3.into(), 0);
        let mut coefficients = dyadic_point(self.interior, 0)?.map(|weight| weight.product(&three));
        if self.end == CubicFitEnd::End {
            coefficients[2] = coefficients[2].difference(&three);
        }
        Ok(coefficients)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_end_stencils_match_the_fixed_operator_and_reverse_native_end_direction() {
        let operator = CubicFitOperator::new([[1., 0., 0.], [0., 1., 0.]]).unwrap();
        let stations = [[0., 0., 0.], [1., 0., 0.], [2., 1., 0.], [3., 3., 1.]];
        for (end, parameter, sign, expected) in [
            (CubicFitEnd::Start, 0., 1., [3., 0., 0.]),
            (CubicFitEnd::End, 1., -1., [-3., -6., -3.]),
        ] {
            let tangent = operator.endpoint_tangent(end);
            assert_eq!(tangent.end(), end);
            assert_eq!(tangent.velocity(stations), Ok(expected));
            let expected_weights =
                operator.weight_jet(parameter).unwrap()[1].map(|weight| sign * weight);
            assert_eq!(tangent.station_weights(), Ok(expected_weights));
        }
    }

    #[test]
    fn one_rounding_keeps_local_geometry_after_large_term_cancellation() {
        let far = (1_u64 << 60) as f64;
        let operator = CubicFitOperator::new([[1., 1., 1.], [0., 0., 1.]]).unwrap();
        let positions = [[0.; 3], [far, 0., 0.], [1., 1., 0.], [-far, 0., 0.]];
        let tangent = operator.endpoint_tangent(CubicFitEnd::Start);
        assert_eq!(tangent.velocity(positions), Ok([3., 3., 0.]));
    }
}
