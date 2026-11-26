// SPDX-License-Identifier: GPL-2.0-only
use super::{NonFiniteCoordinate, exact_coordinate};
use num_bigint::BigInt;
use std::cmp::Ordering;

/// Exact stationarity and second-variation evidence for the supplied binary64 jet.
/// `matches!(certificate, DistanceJetCertificate::Resolved(Ordering::Greater))`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DistanceJetCertificate {
    /// Nonzero second variation outside the query uncertainty band.
    Resolved(Ordering),
    /// The supplied tangent is exactly zero.
    SingularTangent,
    /// The normal-foot residual exceeds the query displacement band.
    Nonstationary,
    /// The second variation is zero or lies within the query displacement band.
    WithinResolution,
}

/// Certify a supplied stationary jet for query displacement norm at most `|resolution|`.
/// The comparisons are exact; derivative construction remains the caller's responsibility.
/// ```
/// use martensite_tkmath::predicates::{certify_distance_stationary_jet, DistanceJetCertificate};
/// let certificate = certify_distance_stationary_jet([2.,0.,0.], [1.,0.,0.], [0.,1.,0.], [-1.,0.,0.], 1e-7)?;
/// assert_eq!(certificate, DistanceJetCertificate::Resolved(std::cmp::Ordering::Greater));
/// # Ok::<(), martensite_tkmath::predicates::NonFiniteCoordinate>(())
/// ```
pub fn certify_distance_stationary_jet(
    query: [f64; 3],
    position: [f64; 3],
    tangent: [f64; 3],
    second: [f64; 3],
    resolution: f64,
) -> Result<DistanceJetCertificate, NonFiniteCoordinate> {
    let mut jet = ExactDistanceJet::default();
    for axis in 0..3 {
        jet.accumulate_axis(
            [query[axis], position[axis], tangent[axis], second[axis]],
            axis,
        )?;
    }
    let resolution = exact_coordinate(resolution, 4, 0)?;
    Ok(jet.classify(&resolution))
}

#[derive(Default)]
struct ExactDistanceJet {
    normal_residual: BigInt,
    tangent_squared: BigInt,
    second_variation: BigInt,
    second_squared: BigInt,
}

impl ExactDistanceJet {
    fn accumulate_axis(
        &mut self,
        coordinates: [f64; 4],
        axis: usize,
    ) -> Result<(), NonFiniteCoordinate> {
        let query = exact_coordinate(coordinates[0], 0, axis)?;
        let position = exact_coordinate(coordinates[1], 1, axis)?;
        let tangent = exact_coordinate(coordinates[2], 2, axis)?;
        let second = exact_coordinate(coordinates[3], 3, axis)?;
        let separation = position - query;
        let tangent_squared = &tangent * &tangent;
        self.normal_residual += &separation * &tangent;
        self.tangent_squared += &tangent_squared;
        self.second_variation += tangent_squared + separation * &second;
        self.second_squared += &second * &second;
        Ok(())
    }

    fn classify(&self, resolution: &BigInt) -> DistanceJetCertificate {
        if self.tangent_squared == BigInt::from(0) {
            return DistanceJetCertificate::SingularTangent;
        }
        // Common binary64 scales cancel after squaring; no rounded norm or residual gates a root.
        let squared_resolution = resolution * resolution;
        if exceeds_squared_band(
            &self.normal_residual,
            &squared_resolution,
            &self.tangent_squared,
        ) {
            return DistanceJetCertificate::Nonstationary;
        }
        if !exceeds_squared_band(
            &self.second_variation,
            &squared_resolution,
            &self.second_squared,
        ) {
            return DistanceJetCertificate::WithinResolution;
        }
        DistanceJetCertificate::Resolved(self.second_variation.cmp(&BigInt::from(0)))
    }
}

fn exceeds_squared_band(component: &BigInt, squared_scale: &BigInt, squared_norm: &BigInt) -> bool {
    component * component > squared_scale * squared_norm
}
