// SPDX-License-Identifier: GPL-2.0-only
//! Fixed-station interpolation of one open clamped cubic.
mod prepare;
mod validation;

use crate::{
    BatchPhase, ControlSplineFailure, ControlSplineJet, ControlSplineJetFailure,
    ControlSplineProofPolicy, EditFailure, MinimumLength, PointId, Snapshot,
    control_spline::{CubicEvidence, CubicEvidenceFailure, capture_evidence},
};
use martensite_tkmath::{
    gp::{Point3, Vector3},
    predicates::roots::{
        CubicChordDistortion, CubicFitEnd, CubicFitNumericError, CubicFitTangent,
        metric_bounds::BezierRegularityReport,
    },
};
use prepare::PreparedFit;
use std::{fmt, num::NonZeroU64, sync::Arc};

/// Caller-reserved identity in the shared point and curve namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FitSplineId(NonZeroU64);

impl FitSplineId {
    /// Wrap a reserved identity. Example: `FitSplineId::new(id)`.
    pub const fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    /// Return the reserved identity. Example: `spline_id.get()`.
    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// Fixed normalized parameters for four distinct station operands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FitSplineParameters([f64; 4]);

impl FitSplineParameters {
    /// Admit `[0,t1,t2,1]` with `0 < t1 < t2 < 1`.
    /// Example: `FitSplineParameters::new([0., 0.25, 0.75, 1.])?`.
    pub fn new(values: [f64; 4]) -> Result<Self, FitSplineFailure> {
        if values[0] != 0. || values[3] != 1. || !values.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(FitSplineFailure::Parameters(values));
        }
        Ok(Self(values))
    }

    /// Return the authored normalized parameters. Example: `parameters.values()[1]`.
    pub const fn values(self) -> [f64; 4] {
        self.0
    }
}

/// Separate spatial, conditioning and parameter proof limits for fit admission.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FitSplinePolicy {
    /// Finite maximum ratio of fastest to slowest adjacent chord speeds, at least one.
    pub max_distortion: f64,
    /// Finite upper bound on maximum affine inverse row L1 norm, at least one.
    pub max_inverse_gain: f64,
    /// Maximum station identity row L1 error, finite and in `[0,1)`.
    pub max_station_identity_error: f64,
    /// Maximum spatial miss from the actual stored curve, measured by an L1 upper bound.
    pub max_station_miss: MinimumLength,
    /// Whole-domain squared-speed proof for the actual stored polynomial.
    pub regularity: ControlSplineProofPolicy,
}

/// Authored four-station open cubic with an immutable parameter recipe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SketchFitSpline {
    /// Caller-reserved spline identity.
    pub id: FitSplineId,
    /// Directed station identities, including the exact shared endpoints at indices zero and three.
    pub stations: [PointId; 4],
    /// Fixed parameters reused after every coordinate edit.
    pub parameters: FitSplineParameters,
    /// Explicit fit and proof limits retained with the authored recipe.
    pub policy: FitSplinePolicy,
}

impl SketchFitSpline {
    /// Return the directed endpoint identities. Example: `recipe.endpoints()[0]`.
    pub const fn endpoints(self) -> [PointId; 2] {
        [self.stations[0], self.stations[3]]
    }
}

/// Bounds for the immutable affine solve, separate from stored geometry error.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FitSplineOperatorReport {
    /// Certified upper bound for the maximum inverse row L1 norm.
    pub inverse_gain_upper: f64,
    /// Certified upper bound for the maximum L1 error row of the station identity.
    pub station_identity_upper: f64,
}

/// An admitted stored cubic, with an immutable station-to-pole operator.
#[derive(Clone, Debug, PartialEq)]
pub struct FitSplineGeometry {
    recipe: SketchFitSpline,
    stations: [Point3; 4],
    poles: [Point3; 4],
    operator: Arc<PreparedFit>,
    cubic: Arc<CubicEvidence>,
    distortion: CubicChordDistortion,
    pole_rounding_upper: f64,
    station_miss_upper: [f64; 4],
}

impl FitSplineGeometry {
    /// Return the immutable authored recipe. Example: `spline.recipe().parameters`.
    pub const fn recipe(&self) -> SketchFitSpline {
        self.recipe
    }

    /// Return the exact authored station positions. Example: `spline.stations()[0]`.
    pub const fn stations(&self) -> [Point3; 4] {
        self.stations
    }

    /// Return the actually stored pole coordinates. Example: `spline.poles()[2]`.
    pub const fn poles(&self) -> [Point3; 4] {
        self.poles
    }

    /// Borrow the measured immutable operator bounds. Example: `spline.operator_report()`.
    pub fn operator_report(&self) -> &FitSplineOperatorReport {
        &self.operator.report
    }

    /// Return the exact-input chord spread receipt. Example: `spline.distortion().ratio_upper`.
    pub const fn distortion(&self) -> CubicChordDistortion {
        self.distortion
    }

    /// Bound position error over the whole span from rounding the real affine poles to storage.
    /// Example: `spline.pole_rounding_upper()`.
    pub const fn pole_rounding_upper(&self) -> f64 {
        self.pole_rounding_upper
    }

    /// Return per-station outward L1 miss bounds for the actually stored curve.
    /// Example: `spline.station_miss_upper()[1]`.
    pub const fn station_miss_upper(&self) -> [f64; 4] {
        self.station_miss_upper
    }

    /// Borrow the whole-domain speed regularity proof for the actually stored cubic.
    /// Example: `spline.regularity().decision()`.
    pub fn regularity(&self) -> &BezierRegularityReport {
        &self.cubic.regularity
    }

    /// Evaluate the stored curve, preserving authored endpoint bits.
    /// Example: `spline.value(0.5)?`.
    pub fn value(&self, parameter: f64) -> Result<Point3, ControlSplineJetFailure> {
        self.cubic.value(self.endpoint_values(), parameter)
    }

    /// Evaluate the first derivative of the stored curve in normalized parameter units.
    /// Example: `spline.first_derivative(0.5)?`.
    pub fn first_derivative(&self, parameter: f64) -> Result<Vector3, ControlSplineJetFailure> {
        self.cubic.first_derivative(parameter)
    }

    /// Evaluate D0, D1 and D2 of the actual stored polynomial.
    /// Example: `spline.jet(0.5)?.second`.
    pub fn jet(&self, parameter: f64) -> Result<ControlSplineJet, ControlSplineJetFailure> {
        self.cubic.jet(self.endpoint_values(), parameter)
    }

    /// Return the rounded analytic D0, D1 and D2 sensitivities in directed station order.
    /// These are derivatives of the retained real affine map before its poles round to storage.
    /// Pole materialization is a finite rounding operation, whose error is checked independently.
    /// Example: `spline.station_weights(0.5)?[1][2]`.
    pub fn station_weights(&self, parameter: f64) -> Result<[[f64; 4]; 3], CubicFitNumericError> {
        self.operator.operator.weight_jet(parameter)
    }

    pub(crate) fn solver_tangent(&self, end: CubicFitEnd) -> CubicFitTangent {
        self.operator.operator.endpoint_tangent(end)
    }

    fn endpoint_values(&self) -> [Point3; 2] {
        [self.stations[0], self.stations[3]]
    }
}

/// Quantity constrained by an explicit bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FitSplineQuantity {
    /// Dimensionless adjacent chord speed ratio.
    Distortion,
    /// Dimensionless inverse row L1 norm.
    InverseGain,
    /// Dimensionless station identity row L1 error.
    StationIdentity,
    /// Spatial miss at the supplied station index, in model length units.
    StationMiss(usize),
}

/// Fixed banded preparation step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FitSplineBandStage {
    /// Build the collocation band.
    Build,
    /// Factor the collocation band.
    Factor,
    /// Solve the station columns of the inverse.
    Solve,
}

/// Cause of an atomically refused four-station edit.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum FitSplineFailure {
    /// Parameters must strictly increase from zero to one.
    Parameters([f64; 4]),
    /// A policy limit is invalid for the named quantity.
    InvalidLimit(FitSplineQuantity, f64),
    /// The station identity appeared more than once.
    RepeatedStation(PointId),
    /// The station identity has no point in the draft.
    MissingStation(PointId),
    /// Distinct station point identities occupy the same position.
    CoincidentStations([PointId; 2]),
    /// A chord is shorter than the sketch minimum.
    ShortChord {
        /// Chord endpoints.
        stations: [PointId; 2],
        /// Spatial limit in model units.
        minimum: MinimumLength,
    },
    /// A finite upper bound exceeded its policy.
    Limit {
        /// Named metric and units.
        quantity: FitSplineQuantity,
        /// Measured conservative upper bound.
        upper: f64,
        /// Configured admissible maximum.
        limit: f64,
    },
    /// Native collocation preparation refused the fixed layout.
    Banded {
        /// Failed stage.
        stage: FitSplineBandStage,
        /// Native integer status when one was returned.
        status: Option<i32>,
        /// One-based bad pivot row, when returned.
        pivot: Option<i32>,
    },
    /// Exact numeric evaluation or measurement refused an input or bound.
    Numeric(CubicFitNumericError),
    /// The stored polynomial failed the common regularity gate.
    Regularity(ControlSplineFailure),
}

impl fmt::Display for FitSplineFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(source) => source.fmt(formatter),
            Self::Regularity(source) => source.fmt(formatter),
            _ => write!(formatter, "four-station fit admission refused: {self:?}"),
        }
    }
}

impl std::error::Error for FitSplineFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numeric(source) => Some(source),
            Self::Regularity(source) => Some(source),
            _ => None,
        }
    }
}

pub(super) fn admit(
    snapshot: &Snapshot,
    recipe: SketchFitSpline,
    previous: Option<&FitSplineGeometry>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<FitSplineGeometry, EditFailure> {
    let old = previous.filter(|old| old.recipe == recipe);
    let stations =
        validation::read_stations(snapshot, recipe).map_err(|cause| refusal(recipe.id, cause))?;
    if let Some(unchanged) = old.filter(|old| same_positions(old.stations, stations)) {
        return Ok(unchanged.clone());
    }
    validation::check_policy(recipe.policy).map_err(|cause| refusal(recipe.id, cause))?;
    let distortion = validation::check_chords(recipe, stations, snapshot.minimum)
        .map_err(|cause| refusal(recipe.id, cause))?;
    poll(cancelled)?;
    let operator = match old {
        Some(old) => Arc::clone(&old.operator),
        None => Arc::new(prepare::build(recipe).map_err(|cause| refusal(recipe.id, cause))?),
    };
    finish(recipe, stations, distortion, operator, cancelled)
}

fn finish(
    recipe: SketchFitSpline,
    stations: [Point3; 4],
    distortion: CubicChordDistortion,
    operator: Arc<PreparedFit>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<FitSplineGeometry, EditFailure> {
    poll(cancelled)?;
    let material = operator
        .operator
        .materialize(stations.map(|point| *point.coordinates()))
        .map_err(|cause| refusal(recipe.id, FitSplineFailure::Numeric(cause)))?;
    let poles = material.poles.map(|[x, y, z]| Point3::new(x, y, z));
    let cubic = capture_evidence(
        recipe.id.get().get(),
        poles,
        recipe.policy.regularity,
        cancelled,
    )
    .map_err(|cause| match cause {
        CubicEvidenceFailure::Cancelled => EditFailure::Cancelled(BatchPhase::Validation),
        CubicEvidenceFailure::Refused(source) => {
            refusal(recipe.id, FitSplineFailure::Regularity(source))
        }
    })?;
    let station_miss_upper = validation::measure_stations(recipe, stations, &cubic, cancelled)?;
    Ok(FitSplineGeometry {
        recipe,
        stations,
        poles,
        operator,
        cubic: Arc::new(cubic),
        distortion,
        pole_rounding_upper: material.pole_rounding_upper,
        station_miss_upper,
    })
}

fn same_positions(old: [Point3; 4], new: [Point3; 4]) -> bool {
    old.iter()
        .zip(new)
        .all(|(a, b)| a.coordinates().map(f64::to_bits) == b.coordinates().map(f64::to_bits))
}

fn refusal(spline: FitSplineId, source: FitSplineFailure) -> EditFailure {
    EditFailure::FitSpline { spline, source }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), EditFailure> {
    if cancelled() {
        return Err(EditFailure::Cancelled(BatchPhase::Validation));
    }
    Ok(())
}
