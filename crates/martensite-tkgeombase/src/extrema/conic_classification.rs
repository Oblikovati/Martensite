// SPDX-License-Identifier: GPL-2.0-only
//! One local differential certificate for elementary conic stationary roots (#406).
use super::gp_space::{GpPoint, GpVector};
use crate::native_failure::failure;
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkmath::predicates::{DistanceJetCertificate, certify_distance_stationary_jet};
use std::cmp::Ordering;

/// Versioned minimum classification; select legacy only when replaying old results.
/// `let policy = PointCurveClassification::default();`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PointCurveClassification {
    /// Retained flags from the original elementary solver, including known errors.
    LegacyV1,
    /// Certify stationarity and classify the local squared-distance second variation.
    #[default]
    DifferentialV2,
}

impl PointCurveClassification {
    pub(super) fn legacy_seed(self, retained_flag: impl FnOnce() -> bool) -> bool {
        match self {
            Self::LegacyV1 => retained_flag(),
            Self::DifferentialV2 => false,
        }
    }
}

/// Local classification evidence for one retained stationary root (#406).
/// `matches!(extrema.classification(1)?, StationaryNature::Minimum)`.
#[derive(Clone, Debug)]
pub enum StationaryNature {
    /// A minimum reported by the selected solver version.
    Minimum,
    /// A maximum reported by the selected solver version.
    Maximum,
    /// The root and distance remain available, but the local class is not certified.
    Unresolved(Failure),
}

impl StationaryNature {
    pub(super) fn minimum_flag(&self) -> Result<bool, Failure> {
        match self {
            Self::Minimum => Ok(true),
            Self::Maximum => Ok(false),
            Self::Unresolved(error) => Err(error.clone()),
        }
    }
}

impl From<bool> for StationaryNature {
    fn from(minimum: bool) -> Self {
        if minimum {
            Self::Minimum
        } else {
            Self::Maximum
        }
    }
}

pub(super) fn certified_minimum<P: GpPoint>(
    query: P,
    parameter: f64,
    (position, tangent, second): (P, P::Vector, P::Vector),
) -> Result<bool, Failure> {
    let certificate = certify_distance_stationary_jet(
        point_coordinates(query),
        point_coordinates(position),
        vector_coordinates::<P>(tangent),
        vector_coordinates::<P>(second),
        precision::confusion(),
    )
    .map_err(|error| decline(parameter, &error.to_string()))?;
    match certificate {
        DistanceJetCertificate::Resolved(Ordering::Greater) => Ok(true),
        DistanceJetCertificate::Resolved(Ordering::Less) => Ok(false),
        unresolved => Err(unresolved_jet_failure(parameter, unresolved)),
    }
}

fn unresolved_jet_failure(parameter: f64, certificate: DistanceJetCertificate) -> Failure {
    decline(
        parameter,
        &format!(
            "{certificate:?}, query resolution {}",
            precision::confusion()
        ),
    )
}

fn point_coordinates<P: GpPoint>(point: P) -> [f64; 3] {
    std::array::from_fn(|axis| {
        if axis < P::DIMENSION {
            point.coordinate(axis)
        } else {
            0.
        }
    })
}

fn vector_coordinates<P: GpPoint>(vector: P::Vector) -> [f64; 3] {
    std::array::from_fn(|axis| {
        if axis < P::DIMENSION {
            vector.coordinate(axis)
        } else {
            0.
        }
    })
}

fn decline(parameter: f64, reason: &str) -> Failure {
    failure(
        "Extrema_UncertifiedStationaryRoot",
        &format!(
            "conic root {parameter}: {reason}; expected finite regular stationary root with resolved second variation"
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{PointCurveClassification, certified_minimum};
    use martensite_tkmath::gp::{Point2, Vector2};

    #[test]
    fn certificate_refuses_nonstationary_singular_and_nonfinite_witnesses() {
        let query = Point2::default();
        let position = Point2::new(1., 0.);
        for tangent in [
            Vector2::new(1., 0.),
            Vector2::default(),
            Vector2::new(f64::NAN, 0.),
        ] {
            let error =
                certified_minimum(query, 0., (position, tangent, Vector2::default())).unwrap_err();
            assert_eq!(error.exception_type(), "Extrema_UncertifiedStationaryRoot");
        }
    }
    #[test]
    fn only_explicit_legacy_policy_evaluates_the_retained_classifier() {
        let calls = std::cell::Cell::new(0);
        let retained = || {
            calls.set(calls.get() + 1);
            true
        };
        assert!(!PointCurveClassification::DifferentialV2.legacy_seed(retained));
        assert_eq!(calls.get(), 0);
        assert!(PointCurveClassification::LegacyV1.legacy_seed(retained));
        assert_eq!(calls.get(), 1);
    }
}
