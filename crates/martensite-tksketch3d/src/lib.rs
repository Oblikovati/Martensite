// SPDX-License-Identifier: GPL-2.0-only
//! Owned spatial sketch geometry with atomic edit validation.
mod admission;
mod arc;
pub mod constraints;
mod control_spline;
mod edit;
mod failure;
mod source;
pub mod split_plane;
pub mod wire;

pub use arc::{ArcDefinition, ArcFailure, ArcFitTolerance, ArcGeometry, ArcId, SketchArc};
pub use control_spline::{
    ControlSplineFailure, ControlSplineGeometry, ControlSplineId, ControlSplineJet,
    ControlSplineJetFailure, ControlSplineProofPolicy, SketchControlSpline,
};
pub use failure::{BatchPhase, EditFailure, InvalidMinimumLength};
use martensite_tkmath::gp::{Line3, Point3};
pub use source::{FrozenPointSource, FrozenSourceRevision};
use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroU64,
};

/// Caller-reserved identity for one point. All entities share a numeric namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointId(NonZeroU64);

impl PointId {
    /// Wrap a reserved identity. Example: `PointId::new(id)`.
    pub const fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    /// Return the caller's original identity. Example: `point_id.get()`.
    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// Caller-reserved identity for one line. All entities share a numeric namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineId(NonZeroU64);

impl LineId {
    /// Wrap a reserved identity. Example: `LineId::new(id)`.
    pub const fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    /// Return the caller's original identity. Example: `line_id.get()`.
    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// Typed identity of an owned sketch entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityId {
    /// Point identity.
    Point(PointId),
    /// Line identity.
    Line(LineId),
    /// Circular arc identity.
    Arc(ArcId),
    /// Open cubic control spline identity.
    ControlSpline(ControlSplineId),
}

/// A sketch curve whose modelling participation can change without replacing its identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurveId {
    /// A straight segment.
    Line(LineId),
    /// A circular arc.
    Arc(ArcId),
    /// An open cubic control spline.
    ControlSpline(ControlSplineId),
}

impl CurveId {
    fn entity(self) -> EntityId {
        match self {
            Self::Line(id) => EntityId::Line(id),
            Self::Arc(id) => EntityId::Arc(id),
            Self::ControlSpline(id) => EntityId::ControlSpline(id),
        }
    }
}

/// An authored curve's participation in modelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveRole {
    /// A curve eligible for use in operations such as wire construction.
    Model,
    /// A reference curve available to the sketch and excluded from modelling wires.
    Construction,
}

/// Authored role, independent of the number of lines using the point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointRole {
    /// A defining point has no separate entity row.
    Defining,
    /// A standalone sketch marker can also define one or more lines.
    SketchMarker,
    /// A standalone center marker can also define one or more lines.
    CenterMarker,
}

/// Authored point value in model coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SketchPoint {
    /// The point's caller-reserved identity.
    pub id: PointId,
    /// The point's authored role.
    pub role: PointRole,
    /// Position in the same model-space unit as the spatial minimum.
    pub position: Point3,
}

/// Authored line with ordered endpoints. Point identities can be shared by other lines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SketchLine {
    /// The line's caller-reserved identity.
    pub id: LineId,
    /// Start and end identities in directed order.
    pub endpoints: [PointId; 2],
}

/// Exact comparison of an admitted segment against the supplied minimum spatial length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinimumComparison {
    /// The exact squared distance equals the square of the minimum.
    Equal,
    /// The exact squared distance exceeds the square of the minimum.
    Greater,
}

/// Validated line geometry, with independent exact admission and a measured length estimate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineGeometry {
    /// The authored identity and endpoint references.
    pub line: SketchLine,
    /// The authored finite segment endpoints in directed order. Use them for exact checks.
    pub points: [Point3; 2],
    /// Nominal binary64 support through the first endpoint with a rounded direction.
    pub carrier: Line3,
    /// Rounded binary64 length estimate, with no certified error enclosure.
    pub length_estimate: f64,
    /// Result of comparing the exact supplied coordinates with the caller's minimum.
    pub minimum_comparison: MinimumComparison,
}

/// Positive finite, caller-supplied minimum spatial length. Equality is admitted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimumLength(f64);

impl MinimumLength {
    /// Validate a limit in model-space units. Example: `MinimumLength::new(1e-6)?`.
    pub fn new(value: f64) -> Result<Self, InvalidMinimumLength> {
        if !value.is_finite() || value <= 0. {
            return Err(InvalidMinimumLength { value });
        }
        Ok(Self(value))
    }

    /// Return the limit in model-space units. Example: `minimum.value()`.
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// One authored operation in an ordered edit batch.
/// Insertions require an identity fresh in the current batch and allocated by the caller
/// for the document's history, including removals and undo.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Edit {
    /// Add a point at an unused identity.
    InsertPoint(SketchPoint),
    /// Add a read-only point at the captured model-space position of an external point.
    InsertFrozenPoint {
        /// Destination point, with the source position expressed in destination model units.
        point: SketchPoint,
        /// Source occurrence and captured revision.
        source: FrozenPointSource,
    },
    /// Change a point's position without changing its identity or role.
    MovePoint {
        /// Existing point identity.
        point: PointId,
        /// Requested position.
        position: Point3,
    },
    /// Add an ordered line. Its endpoint points must exist by the end of the batch.
    InsertLine(SketchLine),
    /// Add a circular arc. Its placement points must exist by the end of the batch.
    InsertArc(SketchArc),
    /// Add an open cubic. Its control points must exist by the end of the batch.
    InsertControlSpline(SketchControlSpline),
    /// Change a curve's modelling participation while retaining its authored geometry.
    SetCurveRole {
        /// Existing curve identity.
        curve: CurveId,
        /// Role to store in the accepted snapshot.
        role: CurveRole,
    },
    /// Make a frozen point editable while keeping its identity, coordinates and incident curves.
    DetachFrozenPoint(PointId),
    /// Remove a point and its incident curves. Cascaded curves cannot be removed again.
    RemovePoint(PointId),
    /// Remove a line. A batch with any deletion retires all unused defining points.
    RemoveLine(LineId),
    /// Remove an arc. A batch with any deletion retires all unused defining points.
    RemoveArc(ArcId),
    /// Remove a control spline. A batch with any deletion retires all unused defining points.
    RemoveControlSpline(ControlSplineId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Entity {
    Point(SketchPoint),
    Line(SketchLine),
    Arc(SketchArc),
    ControlSpline(SketchControlSpline),
}

impl Entity {
    fn id(self) -> EntityId {
        match self {
            Self::Point(point) => EntityId::Point(point.id),
            Self::Line(line) => EntityId::Line(line.id),
            Self::Arc(arc) => EntityId::Arc(arc.id),
            Self::ControlSpline(spline) => EntityId::ControlSpline(spline.id),
        }
    }
    fn curve(self) -> Option<CurveId> {
        match self {
            Self::Point(_) => None,
            Self::Line(line) => Some(CurveId::Line(line.id)),
            Self::Arc(arc) => Some(CurveId::Arc(arc.id)),
            Self::ControlSpline(spline) => Some(CurveId::ControlSpline(spline.id)),
        }
    }
}

impl EntityId {
    fn raw(self) -> NonZeroU64 {
        match self {
            Self::Point(point) => point.get(),
            Self::Line(line) => line.get(),
            Self::Arc(arc) => arc.get(),
            Self::ControlSpline(spline) => spline.get(),
        }
    }
}

/// An immutable, independently owned sketch.
///
/// The kernel enforces ID uniqueness within the current snapshot and batch. The caller
/// reserves IDs across snapshots and issues fresh ones after removal, including across undo.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    minimum: MinimumLength,
    entities: BTreeMap<NonZeroU64, Entity>,
    geometry: BTreeMap<LineId, LineGeometry>,
    arcs: BTreeMap<ArcId, ArcGeometry>,
    control_splines: BTreeMap<ControlSplineId, ControlSplineGeometry>,
    construction: BTreeSet<CurveId>,
    frozen: BTreeMap<PointId, FrozenPointSource>,
}

impl Snapshot {
    /// Create an empty snapshot under the caller's policy. Example: `Snapshot::empty(limit)`.
    pub fn empty(minimum: MinimumLength) -> Self {
        Self {
            minimum,
            entities: BTreeMap::new(),
            geometry: BTreeMap::new(),
            arcs: BTreeMap::new(),
            control_splines: BTreeMap::new(),
            construction: BTreeSet::new(),
            frozen: BTreeMap::new(),
        }
    }

    /// Return an owned point value. Example: `snapshot.point(point_id)`.
    pub fn point(&self, key: PointId) -> Option<SketchPoint> {
        match self.entities.get(&key.get()) {
            Some(Entity::Point(point)) => Some(*point),
            _ => None,
        }
    }

    /// Look up the source stamp for a frozen point. Example: `snapshot.frozen_source(point_id)`.
    pub fn frozen_source(&self, key: PointId) -> Option<FrozenPointSource> {
        self.frozen.get(&key).copied()
    }

    /// Return validated directed geometry. Example: `snapshot.line(line_id)`.
    pub fn line(&self, key: LineId) -> Option<LineGeometry> {
        self.geometry.get(&key).copied()
    }

    /// Return a validated circular arc value. Example: `snapshot.arc(arc_id)`.
    pub fn arc(&self, key: ArcId) -> Option<ArcGeometry> {
        self.arcs.get(&key).copied()
    }

    /// Borrow validated cubic geometry. Example: `snapshot.control_spline(spline_id)`.
    pub fn control_spline(&self, key: ControlSplineId) -> Option<&ControlSplineGeometry> {
        self.control_splines.get(&key)
    }

    /// Return the authored role of an existing curve. Example: `snapshot.curve_role(curve_id)`.
    pub fn curve_role(&self, curve: CurveId) -> Option<CurveRole> {
        let entity = self.entities.get(&curve.entity().raw())?;
        if entity.curve() != Some(curve) {
            return None;
        }
        let role = if self.construction.contains(&curve) {
            CurveRole::Construction
        } else {
            CurveRole::Model
        };
        Some(role)
    }

    /// Enumerate all owned IDs in ascending raw-key order. Example: `snapshot.owned_ids().count()`.
    pub fn owned_ids(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.entities.values().map(|entity| entity.id())
    }

    /// Enumerate entities with a standalone row in ascending raw-key order.
    /// Example: `snapshot.visible_ids().count()`.
    pub fn visible_ids(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.entities.values().filter_map(|entity| match entity {
            Entity::Point(point) if point.role == PointRole::Defining => None,
            entity => Some(entity.id()),
        })
    }
}
