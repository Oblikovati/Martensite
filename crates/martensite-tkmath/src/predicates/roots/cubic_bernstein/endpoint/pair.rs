// SPDX-License-Identifier: GPL-2.0-only
use super::{
    CubicEndJet, CubicEndJetFailure, SQUARED_SCALE, TangentDirectionBand, exact_cross, magnitude,
    squared, tangent_band,
};
use num_bigint::BigInt;
use std::cmp::Ordering;

impl CubicEndJet {
    /// Compare the incoming tangent of this endpoint with the outgoing tangent of `next`.
    /// Both stencils are captured away from their common endpoint. Example:
    /// `first.tangent_into(&second, 1e-8)?.dot`.
    pub fn tangent_into(
        &self,
        next: &Self,
        sine: f64,
    ) -> Result<TangentDirectionBand, CubicEndJetFailure> {
        let mut band = tangent_band(&self.first, &next.first, sine)?;
        // The first stencil points away from the joint, opposite to incoming traversal.
        band.dot = band.dot.reverse();
        Ok(band)
    }

    /// Compare the norm of the spatial curvature-vector difference with an inverse-length cap.
    /// Endpoint traversal reversal leaves spatial curvature invariant. Example:
    /// `first.curvature_difference(&second, 1e-6)?`.
    pub fn curvature_difference(
        &self,
        other: &Self,
        cap: f64,
    ) -> Result<Ordering, CubicEndJetFailure> {
        let limit = magnitude(cap)?;
        let (a, a_scale) = curvature_ratio(self)?;
        let (b, b_scale) = curvature_ratio(other)?;
        let residual = std::array::from_fn(|i| &a[i] * &b_scale - &b[i] * &a_scale);
        let scale = a_scale * b_scale;
        // Each vector has an extra 2^1074 from dividing by one power of length.
        let observed = squared(&residual) << (2 * SQUARED_SCALE);
        let allowed = &limit * &limit * &scale * &scale;
        Ok(observed.cmp(&allowed))
    }
}

fn curvature_ratio(jet: &CubicEndJet) -> Result<([BigInt; 3], BigInt), CubicEndJetFailure> {
    let second = jet
        .second
        .as_ref()
        .ok_or(CubicEndJetFailure::MissingSecond)?;
    let speed_squared = squared(&jet.first);
    let numerator = exact_cross(&exact_cross(&jet.first, second), &jet.first);
    Ok((numerator, &speed_squared * &speed_squared))
}

#[cfg(test)]
mod tests {
    use super::super::CubicEndStencil;
    use super::*;
    use std::cmp::Ordering::{Equal, Greater, Less};

    fn curve(a: [f64; 3], b: [f64; 3]) -> CubicEndJet {
        CubicEndJet::new(CubicEndStencil::Curvature([[0.; 3], a, b])).unwrap()
    }

    #[test]
    fn opposite_outgoing_derivatives_and_distinct_parameter_speeds_can_share_curvature() {
        let left = curve([-1., 0., 0.], [-2., 1., 0.]);
        let right = curve([2., 0., 0.], [4., 4., 0.]);
        assert_eq!(
            left.tangent_into(&right, 0.).unwrap(),
            TangentDirectionBand {
                dot: Greater,
                sine: Equal,
            }
        );
        assert_eq!(left.tangent_into(&left, 0.).unwrap().dot, Less);
        assert_eq!(left.curvature_difference(&right, 0.), Ok(Equal));
        let opposite_normal = curve([2., 0., 0.], [4., -4., 0.]);
        let nearest = 4_f64 / 3.;
        assert_eq!(
            left.curvature_difference(&opposite_normal, nearest),
            Ok(Greater)
        );
        assert_eq!(
            left.curvature_difference(&opposite_normal, nearest.next_up()),
            Ok(Less)
        );
    }

    #[test]
    fn rotated_same_magnitude_curvatures_compare_as_spatial_vectors() {
        let left = curve([-2., 0., 0.], [-4., 3., 0.]);
        let right = curve([2., 0., 0.], [4., 0., 3.]);
        let flat = curve([2., 0., 0.], [4., 0., 0.]);
        assert_eq!(left.curvature_difference(&flat, 0.5), Ok(Equal));
        assert_eq!(
            left.curvature_difference(&flat, 0.5_f64.next_down()),
            Ok(Greater)
        );
        let diagonal = 0.5_f64.sqrt();
        assert_eq!(
            left.curvature_difference(&right, diagonal.next_down()),
            Ok(Greater)
        );
        assert_eq!(left.curvature_difference(&right, diagonal), Ok(Less));
        assert_eq!(right.curvature_difference(&left, diagonal), Ok(Less));
    }
}
