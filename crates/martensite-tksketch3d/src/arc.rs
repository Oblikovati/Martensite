// SPDX-License-Identifier: GPL-2.0-only
//! Preserve authored points separately from the nominal circular carrier.
mod carrier;

use crate::{EditFailure, PointId, Snapshot, admission::estimated_norm};
use carrier::Carrier;
use martensite_tkmath::{
    gp::{Axis2, Direction3, Point3, Vector3},
    predicates::compare_squared_length_3d,
};
use std::{cmp::Ordering, fmt, num::NonZeroU64};

/// Caller-reserved identity in the shared point and curve namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArcId(NonZeroU64);

impl ArcId {
    /// Wrap a reserved identity. Example: `ArcId::new(id)`.
    pub const fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    /// Return the caller's original identity. Example: `arc_id.get()`.
    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// A positive finite deviation limit for the supplied placement points, in model units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArcFitTolerance(f64);

impl ArcFitTolerance {
    /// Validate the maximum point deviation. Example: `ArcFitTolerance::new(1e-8)?`.
    pub fn new(value: f64) -> Result<Self, ArcFailure> {
        if value.is_finite() && value > 0. {
            return Ok(Self(value));
        }
        Err(ArcFailure::InvalidFitTolerance(value))
    }

    /// Return the requested maximum deviation. Example: `tolerance.value()`.
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Authored placement rule and choice of directed branch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArcDefinition {
    /// The open arc goes from start through this interior witness to end.
    Through {
        /// Authored point on the interior of the arc.
        witness: PointId,
    },
    /// The arc sweeps positively around the normal from start to end.
    Center {
        /// Authored center of the circle.
        center: PointId,
        /// Oriented plane normal. It also specifies the plane of a semicircle.
        normal: Direction3,
    },
}

impl ArcDefinition {
    fn placement(self) -> PointId {
        match self {
            Self::Through { witness } => witness,
            Self::Center { center, .. } => center,
        }
    }
}

/// Authored spatial arc with an ordered endpoint pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SketchArc {
    /// Caller-reserved arc identity.
    pub id: ArcId,
    /// Directed start and end point identities.
    pub endpoints: [PointId; 2],
    /// Placement point and branch rule.
    pub definition: ArcDefinition,
    /// Maximum allowed deviation at a supplied placement point.
    pub fit_tolerance: ArcFitTolerance,
}

impl SketchArc {
    pub(super) fn points(self) -> [PointId; 3] {
        [
            self.endpoints[0],
            self.endpoints[1],
            self.definition.placement(),
        ]
    }
}

/// Validated open circle trim with immutable recipes and placement points.
#[derive(Clone, Copy, Debug)]
pub struct ArcGeometry {
    recipe: SketchArc,
    points: [Point3; 3],
    carrier: Carrier,
    deviation_estimate: f64,
}

impl PartialEq for ArcGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.recipe == other.recipe
            && self.points == other.points
            && self.carrier == other.carrier
            && self.deviation_estimate == other.deviation_estimate
    }
}

impl ArcGeometry {
    /// Return the authored recipe. Example: `arc.recipe().endpoints`.
    pub const fn recipe(self) -> SketchArc {
        self.recipe
    }

    /// Return exact supplied values in start, end, placement order. Example: `arc.points()[1]`.
    pub const fn points(self) -> [Point3; 3] {
        self.points
    }

    /// Return the nominal circle frame. Example: `arc.frame().location()`.
    pub const fn frame(self) -> Axis2 {
        self.carrier.frame
    }

    /// Return the nominal radius, in model units. Example: `arc.radius()`.
    pub const fn radius(self) -> f64 {
        self.carrier.radius
    }

    /// Return the directed closed parameter range in radians. Example: `arc.range()[1]`.
    pub const fn range(self) -> [f64; 2] {
        [0., self.carrier.sweep]
    }

    /// Evaluate the nominal trim. Example: `arc.value(arc.range()[1] / 2.)`.
    pub fn value(self, angle: f64) -> Option<Point3> {
        (0. ..=self.carrier.sweep)
            .contains(&angle)
            .then(|| self.carrier.value(angle))
    }

    /// Return the maximum rounded deviation at the three supplied placement points.
    /// This is a sampled estimate. Example: `arc.deviation_estimate()`.
    pub const fn deviation_estimate(self) -> f64 {
        self.deviation_estimate
    }
}

/// Cause of a refused spatial arc or arc tolerance.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum ArcFailure {
    /// Requested deviation must be positive and finite.
    InvalidFitTolerance(f64),
    /// Multiple placement operands refer to the same point identity.
    RepeatedPoint(PointId),
    /// Placement operand has no corresponding point in the draft.
    MissingPoint(PointId),
    /// Exact chord length is below the snapshot minimum length.
    ShortChord,
    /// Exact distance of the witness to the chord support is below the snapshot minimum.
    SmallWitnessAltitude,
    /// Exact center-to-endpoint distance is below the snapshot minimum.
    ShortRadius(PointId),
    /// The resolved carrier does not have the required directed trim.
    IndeterminateBranch,
    /// The chosen circle is outside the supported finite binary64 range.
    UnrepresentableCarrier,
    /// A nominal placement point misses the authored point by more than the requested limit.
    FitExceeded {
        /// Placement point that failed admission.
        point: PointId,
        /// Rounded separation. The admission comparison uses exact squared binary64 distance.
        estimate: f64,
        /// Maximum point deviation requested by the caller.
        requested: ArcFitTolerance,
    },
}

impl fmt::Display for ArcFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "spatial arc admission refused: {self:?}")
    }
}

impl std::error::Error for ArcFailure {}

pub(super) fn admit(
    snapshot: &Snapshot,
    recipe: SketchArc,
    previous: Option<&ArcGeometry>,
) -> Result<ArcGeometry, EditFailure> {
    let result = (|| {
        let points = read_points(snapshot, recipe)?;
        if let Some(old) = previous.filter(|old| same_inputs(old, recipe, points)) {
            return Ok(*old);
        }
        let minimum = snapshot.minimum.value();
        if distance_comparison(points[0], points[1], minimum)? == Ordering::Less {
            return Err(ArcFailure::ShortChord);
        }
        let carrier = carrier::construct(recipe, points, minimum)?;
        let deviation_estimate = fit(recipe, points, carrier)?;
        Ok(ArcGeometry {
            recipe,
            points,
            carrier,
            deviation_estimate,
        })
    })();
    result.map_err(|source| EditFailure::Arc {
        arc: recipe.id,
        source,
    })
}

fn read_points(snapshot: &Snapshot, recipe: SketchArc) -> Result<[Point3; 3], ArcFailure> {
    let [start, end, placement] = recipe.points();
    if start == end || start == placement {
        return Err(ArcFailure::RepeatedPoint(start));
    }
    if end == placement {
        return Err(ArcFailure::RepeatedPoint(end));
    }
    let read = |point| {
        snapshot
            .point(point)
            .map(|point| point.position)
            .ok_or(ArcFailure::MissingPoint(point))
    };
    Ok([read(start)?, read(end)?, read(placement)?])
}

fn same_inputs(previous: &ArcGeometry, recipe: SketchArc, points: [Point3; 3]) -> bool {
    previous.recipe == recipe
        && previous.points.iter().zip(points).all(|(old, new)| {
            old.coordinates().map(f64::to_bits) == new.coordinates().map(f64::to_bits)
        })
}

fn fit(recipe: SketchArc, points: [Point3; 3], carrier: Carrier) -> Result<f64, ArcFailure> {
    let placement = match carrier.witness {
        Some(angle) => carrier.value(angle),
        None => carrier.frame.location(),
    };
    let samples = [carrier.value(0.), carrier.value(carrier.sweep), placement];
    let mut maximum: f64 = 0.;
    for ((point, authored), nominal) in recipe.points().into_iter().zip(points).zip(samples) {
        maximum = maximum.max(check_fit(point, authored, nominal, recipe.fit_tolerance)?);
    }
    Ok(maximum)
}

fn check_fit(
    point: PointId,
    authored: Point3,
    nominal: Point3,
    requested: ArcFitTolerance,
) -> Result<f64, ArcFailure> {
    let estimate = estimated_norm(*Vector3::between(authored, nominal).xyz());
    if distance_comparison(authored, nominal, requested.value())? == Ordering::Greater {
        return Err(ArcFailure::FitExceeded {
            point,
            estimate,
            requested,
        });
    }
    Ok(estimate)
}

fn distance_comparison(first: Point3, second: Point3, limit: f64) -> Result<Ordering, ArcFailure> {
    compare_squared_length_3d(*first.coordinates(), *second.coordinates(), limit)
        .map_err(|_| ArcFailure::UnrepresentableCarrier)
}
