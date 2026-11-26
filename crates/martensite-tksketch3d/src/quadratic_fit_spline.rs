// SPDX-License-Identifier: GPL-2.0-only
//! Owned open quadratic fitted at exactly three authored stations.
use crate::{
    BatchPhase, EditFailure, FitSplineFailure, FitSplineId, FitSplineOperatorReport,
    FitSplinePolicy, FitSplineQuantity, PointId, Snapshot,
    control_spline::{CubicEvidenceFailure, certify},
    fit_spline::{check_limit, check_minimum_chords, check_policy, read_station_points},
};
use martensite_tkmath::{
    gp::Point3,
    predicates::roots::{
        CubicChordDistortion, CubicFitNumericError, evaluate_fit_quadratic,
        materialize_fit_quadratic, metric_bounds::BezierRegularityReport,
        quadratic_fit_chord_distortion,
    },
};
use std::sync::Arc;

/// A three-station fit at the fixed normalized parameters 0, 1/2 and 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SketchQuadraticFitSpline {
    /// Caller-reserved identity in the ordinary shared fit namespace.
    pub id: FitSplineId,
    /// Three directed authored point identities.
    pub stations: [PointId; 3],
    /// Ordinary spatial, conditioning and regularity limits.
    pub policy: FitSplinePolicy,
}

impl SketchQuadraticFitSpline {
    /// Return the two authored endpoint identities. Example: `recipe.endpoints()`.
    pub const fn endpoints(self) -> [PointId; 2] {
        [self.stations[0], self.stations[2]]
    }
}

/// Validated actual quadratic and its original station and fit evidence.
#[derive(Clone, Debug, PartialEq)]
pub struct QuadraticFitSplineGeometry {
    recipe: SketchQuadraticFitSpline,
    stations: [Point3; 3],
    poles: [Point3; 3],
    regularity: Arc<BezierRegularityReport>,
    distortion: CubicChordDistortion,
    pole_rounding_upper: f64,
    station_miss_upper: [f64; 3],
}

impl QuadraticFitSplineGeometry {
    /// Read the exact authored recipe. Example: `curve.recipe().stations`.
    pub const fn recipe(&self) -> SketchQuadraticFitSpline {
        self.recipe
    }

    /// Read the ordered actual authored stations. Example: `curve.stations()[1]`.
    pub const fn stations(&self) -> [Point3; 3] {
        self.stations
    }

    /// Read the actual quadratic controls. Example: `curve.poles()[1]`.
    pub const fn poles(&self) -> [Point3; 3] {
        self.poles
    }

    /// Read the exact three-station collocation bounds. Example: `curve.operator_report()`.
    pub const fn operator_report(&self) -> FitSplineOperatorReport {
        FitSplineOperatorReport {
            inverse_gain_upper: 3.,
            station_identity_upper: 0.,
        }
    }

    /// Read the outward adjacent chord-speed ratio. Example: `curve.distortion().ratio_upper`.
    pub const fn distortion(&self) -> CubicChordDistortion {
        self.distortion
    }

    /// Read the actual all-span pole rounding bound. Example: `curve.pole_rounding_upper()`.
    pub const fn pole_rounding_upper(&self) -> f64 {
        self.pole_rounding_upper
    }

    /// Read actual position misses at all three stations. Example: `curve.station_miss_upper()`.
    pub const fn station_miss_upper(&self) -> [f64; 3] {
        self.station_miss_upper
    }

    /// Borrow the nonzero-speed certificate. Example: `curve.regularity().decision()`.
    pub fn regularity(&self) -> &BezierRegularityReport {
        &self.regularity
    }

    /// Evaluate the actually stored quadratic. Example: `curve.value(0.5)?`.
    pub fn value(&self, parameter: f64) -> Result<Point3, CubicFitNumericError> {
        evaluate_fit_quadratic(self.poles.map(|p| *p.coordinates()), parameter).map(point)
    }
}

pub(super) fn admit(
    snapshot: &Snapshot,
    recipe: SketchQuadraticFitSpline,
    previous: Option<&QuadraticFitSplineGeometry>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<QuadraticFitSplineGeometry, EditFailure> {
    let refuse = |source| EditFailure::QuadraticFitSpline {
        spline: recipe.id,
        source,
    };
    let stations = read_station_points(snapshot, recipe.stations).map_err(refuse)?;
    if let Some(previous) = previous.filter(|old| same_source(old, recipe, stations)) {
        return Ok(previous.clone());
    }
    let coordinates = stations.map(|p| *p.coordinates());
    let distortion = check_admission(recipe, coordinates, snapshot).map_err(refuse)?;
    let retained = materialize_fit_quadratic(coordinates)
        .map_err(FitSplineFailure::Numeric)
        .map_err(refuse)?;
    check_limit(
        FitSplineQuantity::StationMiss(1),
        retained.midpoint_miss_upper,
        recipe.policy.max_station_miss.value(),
    )
    .map_err(refuse)?;
    let regularity = certify(
        recipe.id.get().get(),
        recipe.policy.regularity,
        &retained.poles,
        cancelled,
    )
    .map_err(|error| match error {
        CubicEvidenceFailure::Cancelled => EditFailure::Cancelled(BatchPhase::Validation),
        CubicEvidenceFailure::Refused(cause) => refuse(FitSplineFailure::Regularity(cause)),
    })?;
    Ok(QuadraticFitSplineGeometry {
        recipe,
        stations,
        poles: retained.poles.map(point),
        regularity: Arc::new(regularity),
        distortion,
        pole_rounding_upper: retained.pole_rounding_upper,
        station_miss_upper: [0., retained.midpoint_miss_upper, 0.],
    })
}

fn check_admission(
    recipe: SketchQuadraticFitSpline,
    coordinates: [[f64; 3]; 3],
    snapshot: &Snapshot,
) -> Result<CubicChordDistortion, FitSplineFailure> {
    check_policy(recipe.policy)?;
    check_limit(
        FitSplineQuantity::InverseGain,
        3.,
        recipe.policy.max_inverse_gain,
    )?;
    check_minimum_chords(recipe.stations, coordinates, snapshot.minimum)?;
    let distortion =
        quadratic_fit_chord_distortion(coordinates).map_err(FitSplineFailure::Numeric)?;
    check_limit(
        FitSplineQuantity::Distortion,
        distortion.ratio_upper,
        recipe.policy.max_distortion,
    )?;
    Ok(distortion)
}

fn same_source(
    old: &QuadraticFitSplineGeometry,
    recipe: SketchQuadraticFitSpline,
    stations: [Point3; 3],
) -> bool {
    old.recipe == recipe
        && old
            .stations
            .iter()
            .zip(stations)
            .all(|(a, b)| a.coordinates().map(f64::to_bits) == b.coordinates().map(f64::to_bits))
}

fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}
