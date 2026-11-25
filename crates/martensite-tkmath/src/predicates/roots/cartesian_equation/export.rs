// SPDX-License-Identifier: GPL-2.0-only
//! Once-rounded native controls, with full-interval error and independent regularity proof.
use super::{
    super::certificate_arithmetic::rounding::outward_sqrt,
    CartesianEquationCurve, CartesianEquationError as Error, EquationDomainCertificate, R,
    nearest_binary64,
    net::{self, Net},
    poll,
    program::zero,
};

/// Certified unweighted Bézier materialization of a Cartesian equation.
#[derive(Clone, Debug, PartialEq)]
pub struct EquationBezierCertificate {
    poles: Vec<[f64; 3]>,
    parameter: [f64; 2],
    distance_bound: f64,
    normalized: EquationDomainCertificate,
}

impl EquationBezierCertificate {
    /// Native binary64 poles in increasing normalized parameter order. Example: `proof.poles()`.
    pub fn poles(&self) -> &[[f64; 3]] {
        &self.poles
    }

    /// Authored interval corresponding to native [0, 1]. Example: `proof.authored_domain()`.
    pub fn authored_domain(&self) -> [f64; 2] {
        self.parameter
    }

    /// Inclusive model-space discrepancy between exact polynomial values at the same normalized parameter.
    /// Example: `proof.distance_bound()`.
    pub fn distance_bound(&self) -> f64 {
        self.distance_bound
    }

    /// Finite exact polynomial jets and speed bound of the retained binary64 control net.
    /// Example: `proof.normalized_domain_certificate().speed_lower`.
    pub fn normalized_domain_certificate(&self) -> EquationDomainCertificate {
        self.normalized
    }
}

impl CartesianEquationCurve {
    /// Prove the rounded native poles stay within the inclusive requested error.
    /// Example: `curve.certify_bezier(1e-9, || false)?.distance_bound()`.
    pub fn certify_bezier(
        &self,
        limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<EquationBezierCertificate, Error> {
        poll(&mut cancelled)?;
        if !limit.is_finite() || limit < 0. {
            return Err(Error::DistanceLimit(limit));
        }
        let (poles, squared) = round_controls(&self.rows[0], &mut cancelled)?;
        let distance_bound = qualify_distance(&squared, limit)?;
        let rounded = Net(std::array::from_fn(|axis| {
            poles
                .iter()
                .map(|pole| R::from_finite(pole[axis]))
                .collect()
        }));
        let normalized = net::certify(&net::jet(rounded, &R::from_finite(1.)), &mut cancelled)?;
        poll(&mut cancelled)?;
        Ok(EquationBezierCertificate {
            poles,
            parameter: self.domain,
            distance_bound,
            normalized,
        })
    }
}

fn round_controls(
    net: &Net,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Vec<[f64; 3]>, R), Error> {
    let mut poles = Vec::with_capacity(net.0[0].len());
    let mut squared = zero();
    for index in 0..net.0[0].len() {
        poll(cancelled)?;
        let exact = net.0.each_ref().map(|coordinate| &coordinate[index]);
        let [x, y, z] = [0, 1, 2].map(|axis| {
            nearest_binary64(exact[axis]).ok_or(Error::Unrepresentable { order: 0, axis })
        });
        let rounded = [x?, y?, z?];
        squared = squared.max(discrepancy(exact, rounded));
        poles.push(rounded);
    }
    Ok((poles, squared))
}

fn discrepancy(exact: [&R; 3], rounded: [f64; 3]) -> R {
    // The error curve shares the same Bernstein basis, so its norm is bounded
    // everywhere by the largest norm of its difference controls.
    exact
        .into_iter()
        .zip(rounded)
        .fold(zero(), |sum, (original, pole)| {
            let delta = original.subtract(&R::from_finite(pole));
            sum.add(&delta.multiply(&delta))
        })
}

fn qualify_distance(squared: &R, limit: f64) -> Result<f64, Error> {
    let bound = if squared.sign().is_eq() {
        Some(0.)
    } else {
        outward_sqrt(squared)
    };
    let cap = R::from_finite(limit);
    if *squared > cap.multiply(&cap) {
        return Err(Error::ExportDistance { bound, limit });
    }
    bound.ok_or(Error::ExportDistance { bound, limit })
}
