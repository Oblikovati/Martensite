// SPDX-License-Identifier: GPL-2.0-only
//! An authored cubic with exact whole-domain speed regularity and immutable coefficients.
mod evaluation;

use crate::{BatchPhase, EditFailure, MinimumLength, PointId, Snapshot};
pub use evaluation::{ControlSplineJet, ControlSplineJetFailure};
use martensite_tkmath::{
    gp::Point3,
    predicates::{
        NonFiniteCoordinate, compare_squared_length_3d,
        roots::{
            RootIsolationError, RootIsolationOptions,
            metric_bounds::{
                BezierRegularityCause, BezierRegularityCompletion, BezierRegularityDecision,
                BezierRegularityFailure, BezierRegularityOptions, BezierRegularityQuery,
                BezierRegularityReport, BezierRegularityStage, certify_bezier_regularity,
            },
        },
    },
};
use std::{cmp::Ordering, fmt, num::NonZeroU64, sync::Arc};

/// Caller-reserved identity in the shared point and curve namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlSplineId(NonZeroU64);

impl ControlSplineId {
    /// Wrap a reserved identity. Example: `ControlSplineId::new(id)`.
    pub const fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    /// Return the reserved identity. Example: `spline_id.get()`.
    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// Caller-selected parameter precision and subdivision work for one cubic's proof.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlSplineProofPolicy {
    parameter_width: f64,
    max_bisections: u16,
}

impl ControlSplineProofPolicy {
    /// Set the maximum bracket width in dimensionless curve parameters and the subdivision limit.
    /// A zero budget permits proofs that need no subdivision.
    /// Example: `ControlSplineProofPolicy::new(1e-8, 128)?`.
    pub fn new(parameter_width: f64, max_bisections: u16) -> Result<Self, ControlSplineFailure> {
        if !parameter_width.is_finite() || parameter_width <= 0. {
            return Err(ControlSplineFailure::InvalidParameterWidth(parameter_width));
        }
        Ok(Self {
            parameter_width,
            max_bisections,
        })
    }

    /// Return the requested bracket width. Example: `policy.parameter_width()`.
    pub const fn parameter_width(self) -> f64 {
        self.parameter_width
    }

    /// Return the subdivision limit. Example: `policy.max_bisections()`.
    pub const fn max_bisections(self) -> u16 {
        self.max_bisections
    }

    fn options(self) -> BezierRegularityOptions {
        // A polynomial cubic has a degree-three control net and a degree-four squared speed.
        BezierRegularityOptions {
            version: 1,
            max_stored_degree: 3,
            roots: RootIsolationOptions {
                max_bisections: self.max_bisections,
                max_degree: 4,
                ..RootIsolationOptions::new(self.parameter_width)
            },
        }
    }
}

/// Authored nonperiodic, unweighted cubic with a single normalized clamped span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SketchControlSpline {
    /// Caller-reserved spline identity.
    pub id: ControlSplineId,
    /// Ordered control points. The first and last identities are the directed endpoints.
    pub poles: [PointId; 4],
    /// Requested parameter proof policy, fixed within this recipe.
    pub proof: ControlSplineProofPolicy,
}

impl SketchControlSpline {
    /// Return shared directed endpoint identities. Example: `recipe.endpoints()[0]`.
    pub const fn endpoints(self) -> [PointId; 2] {
        [self.poles[0], self.poles[3]]
    }
}

/// Validated cubic with a nonzero exact derivative over its closed parameter range.
///
/// The regularity certificate concerns the represented coefficients and their derivative.
/// It gives no curve injectivity or downstream modelling evidence.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlSplineGeometry {
    recipe: SketchControlSpline,
    poles: [Point3; 4],
    regularity: Arc<BezierRegularityReport>,
}

impl ControlSplineGeometry {
    /// Return the authored recipe. Example: `curve.recipe().endpoints()`.
    pub const fn recipe(&self) -> SketchControlSpline {
        self.recipe
    }

    /// Return the exact supplied ordered control point values. Example: `curve.poles()[1]`.
    pub const fn poles(&self) -> [Point3; 4] {
        self.poles
    }

    /// Return the closed normalized clamped span. Example: `curve.range()[1]`.
    pub const fn range(&self) -> [f64; 2] {
        [0., 1.]
    }

    /// Borrow the exact source-bound regularity certificate. Example: `curve.regularity().decision()`.
    pub fn regularity(&self) -> &BezierRegularityReport {
        &self.regularity
    }
}

/// Cause of a declined cubic control recipe.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum ControlSplineFailure {
    /// Root brackets require a positive finite normalized parameter width.
    InvalidParameterWidth(f64),
    /// The directed endpoints repeat an identity.
    SameEndpoint(PointId),
    /// A control point has no corresponding point in the draft.
    MissingPole(PointId),
    /// The supplied endpoint distance is below the spatial sketch minimum.
    ShortChord(MinimumLength),
    /// The exact endpoint comparison refused a coordinate.
    ChordInput(NonFiniteCoordinate),
    /// The exact parameterization has a zero derivative somewhere on the closed span.
    Stationary {
        /// Number of distinct isolated stationary parameters, or none for an identically zero derivative.
        isolated_parameters: Option<usize>,
        /// Subdivisions spent completing the classification.
        bisections: u16,
    },
    /// Complete root classification exceeded the requested subdivision work.
    WorkLimit {
        /// Requested subdivision limit.
        limit: u16,
        /// Completed subdivisions.
        bisections: u16,
    },
    /// The exact regularity authority refused the fixed cubic query.
    ProofRefused {
        /// Invalid, declined or incomplete classification.
        completion: BezierRegularityCompletion,
        /// Failed stage if root isolation identified it.
        stage: Option<BezierRegularityStage>,
        /// Completed subdivisions.
        bisections: u16,
    },
}

impl fmt::Display for ControlSplineFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::ShortChord(minimum) = self {
            return write!(
                formatter,
                "control spline chord must reach {}",
                minimum.value()
            );
        }
        write!(formatter, "control spline admission refused: {self:?}")
    }
}

impl std::error::Error for ControlSplineFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChordInput(source) => Some(source),
            _ => None,
        }
    }
}

pub(super) fn admit(
    snapshot: &Snapshot,
    recipe: SketchControlSpline,
    previous: Option<&ControlSplineGeometry>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<ControlSplineGeometry, EditFailure> {
    let poles = read_poles(snapshot, recipe).map_err(|cause| refusal(recipe.id, cause))?;
    if let Some(old) = previous.filter(|old| source_matches(old, recipe, poles)) {
        return Ok(old.clone());
    }
    check_chord(poles, snapshot.minimum).map_err(|cause| refusal(recipe.id, cause))?;
    let regularity = certify(recipe, poles, cancelled)?;
    Ok(ControlSplineGeometry {
        recipe,
        poles,
        regularity: Arc::new(regularity),
    })
}

fn read_poles(
    snapshot: &Snapshot,
    recipe: SketchControlSpline,
) -> Result<[Point3; 4], ControlSplineFailure> {
    let [start, end] = recipe.endpoints();
    if start == end {
        return Err(ControlSplineFailure::SameEndpoint(start));
    }
    let mut poles = [Point3::default(); 4];
    for (pole, id) in poles.iter_mut().zip(recipe.poles) {
        *pole = snapshot
            .point(id)
            .ok_or(ControlSplineFailure::MissingPole(id))?
            .position;
    }
    Ok(poles)
}

fn source_matches(
    previous: &ControlSplineGeometry,
    recipe: SketchControlSpline,
    poles: [Point3; 4],
) -> bool {
    previous.recipe == recipe
        && previous.poles.iter().zip(poles).all(|(old, new)| {
            old.coordinates().map(f64::to_bits) == new.coordinates().map(f64::to_bits)
        })
}

fn check_chord(poles: [Point3; 4], minimum: MinimumLength) -> Result<(), ControlSplineFailure> {
    let [start, _, _, end] = poles;
    let comparison =
        compare_squared_length_3d(*start.coordinates(), *end.coordinates(), minimum.value())
            .map_err(ControlSplineFailure::ChordInput)?;
    if comparison == Ordering::Less {
        return Err(ControlSplineFailure::ShortChord(minimum));
    }
    Ok(())
}

fn certify(
    recipe: SketchControlSpline,
    poles: [Point3; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BezierRegularityReport, EditFailure> {
    let coefficients = poles.map(|pole| *pole.coordinates());
    let query =
        BezierRegularityQuery::new(recipe.id.get().get(), &coefficients, recipe.proof.options());
    let report = certify_bezier_regularity(query, cancelled)
        .map_err(|failure| proof_refusal(recipe, failure))?;
    let isolated_parameters = match report.decision() {
        BezierRegularityDecision::Regular { .. } => return Ok(report),
        BezierRegularityDecision::Stationary {
            zero_speed_isolation,
        } => Some(zero_speed_isolation.roots().len()),
        BezierRegularityDecision::EverywhereStationary => None,
    };
    Err(refusal(
        recipe.id,
        ControlSplineFailure::Stationary {
            isolated_parameters,
            bisections: report.bisections(),
        },
    ))
}

fn proof_refusal(recipe: SketchControlSpline, failure: BezierRegularityFailure) -> EditFailure {
    let completion = failure.completion();
    if completion == BezierRegularityCompletion::Cancelled {
        return EditFailure::Cancelled(BatchPhase::Validation);
    }
    let bisections = failure.bisections();
    let stage = match failure.cause() {
        BezierRegularityCause::Roots {
            cause: RootIsolationError::SubdivisionLimit { .. },
            ..
        } => {
            return refusal(
                recipe.id,
                ControlSplineFailure::WorkLimit {
                    limit: recipe.proof.max_bisections,
                    bisections,
                },
            );
        }
        BezierRegularityCause::Roots { stage, .. } => Some(*stage),
        _ => None,
    };
    refusal(
        recipe.id,
        ControlSplineFailure::ProofRefused {
            completion,
            stage,
            bisections,
        },
    )
}

fn refusal(spline: ControlSplineId, source: ControlSplineFailure) -> EditFailure {
    EditFailure::ControlSpline { spline, source }
}
