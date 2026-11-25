// SPDX-License-Identifier: GPL-2.0-only
//! Immutable spatial sketch constraints with separate apply and drag paths.
mod adopt;
mod angle;
mod binding;
mod continuity;
mod error;
mod incidence;
mod initialization;
mod line_edit;
mod metric;
mod motion;
mod on_face;
mod preferred;
mod program;
mod residual;
mod restore;

use crate::{LineId, MinimumLength, PointId, Snapshot};
use martensite_tkmath::gp::{Direction3, Point3};
use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroU64,
    sync::Arc,
};

pub use angle::{InvalidLineAngle, LineAngle};
pub use continuity::{
    ContinuityFailure, ContinuityJoin, CubicCubicJoin, CurvatureTolerance, LineCubicJoin,
};
pub use error::{ConstraintFailure, PolicyQuantity};
pub use martensite_tksolve::{nonlinear::SolveOptions, preview::Limits as SolverLimits};
pub use motion::{RigidMotion, RigidMotionFailure};
pub use on_face::{
    PlanarAttachedSketch, PlanarFaceBinding, PlanarFaceConstraintFailure, PlanarFaceDragUpdate,
    PlanarFaceRelation, PlanarFaceRevision,
};
pub use preferred::{PreferenceReport, PreferenceSettlement, PreferredDragUpdate};
use program::Program;

/// Caller-reserved ID from a namespace separate from points and lines.
/// The caller reserves a fresh ID across removals, undo and document revisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelationId(NonZeroU64);
impl RelationId {
    /// Wrap a reserved relation key. Example: `RelationId::new(key)`.
    pub const fn new(key: NonZeroU64) -> Self {
        Self(key)
    }
    /// Read the caller's relation key. Example: `id.get()`.
    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// Branch based on the authored order of both lines' endpoint identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParallelDirection {
    /// The two start-to-end directions have positive dot product.
    Same,
    /// The two start-to-end directions have negative dot product.
    Opposite,
}

/// One coordinate axis of the sketch's model-space frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateAxis {
    /// Model X axis.
    X,
    /// Model Y axis.
    Y,
    /// Model Z axis.
    Z,
}

impl CoordinateAxis {
    fn index(self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}

/// Sign of a directed line's component along a coordinate axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisDirection {
    /// The authored end lies in the positive direction from its start.
    Positive,
    /// The authored end lies in the negative direction from its start.
    Negative,
}

/// Intent for a newly authored relation. Fix captures positions from the accepted snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RelationSpec {
    /// Fix one point at its current exact position.
    FixPoint(PointId),
    /// Fix both ordered endpoints of an existing line at their current exact positions.
    FixLine(LineId),
    /// Constrain distinct points to coincide without merging their IDs.
    Coincident([PointId; 2]),
    /// Constrain a distinct point to an infinite line support, leaving its axial coordinate free.
    PointOnLine(PointId, LineId),
    /// Constrain two ordered lines to the same support on the specified direction branch.
    Colinear([LineId; 2], ParallelDirection),
    /// Constrain two ordered lines to parallel directions on the specified branch.
    Parallel([LineId; 2], ParallelDirection),
    /// Drive the length of one line in model-space units.
    Length(LineId, MinimumLength),
    /// Constrain a line to the chosen coordinate axis and direction sign.
    ParallelToAxis(LineId, CoordinateAxis, AxisDirection),
    /// Keep a line parallel to every plane with this normal, leaving its offset free.
    ParallelToPlane(LineId, Direction3),
    /// Constrain the direction angle of two lines to a right angle. Their positions stay free.
    Perpendicular([LineId; 2]),
    /// Constrain two lines to equal physical lengths.
    EqualLength([LineId; 2]),
    /// Drive the angle between two directed lines in radians. Their positions stay free.
    Angle([LineId; 2], LineAngle),
    /// Align the directed endpoint tangents of two connected curves.
    Tangent(ContinuityJoin),
    /// Align the directed tangents and match physical endpoint curvature.
    G2(ContinuityJoin, CurvatureTolerance),
}

/// The persisted relation value, with the anchors captured for Fix relations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Relation {
    /// Fixed point ID and exact authored position.
    FixPoint(PointId, Point3),
    /// Fixed line ID and exact positions in authored endpoint order.
    FixLine(LineId, [Point3; 2]),
    /// Two point identities whose positions must coincide within the length tolerance.
    Coincident([PointId; 2]),
    /// Point identity and its infinite line support.
    PointOnLine(PointId, LineId),
    /// Two ordered line identities and their stored direction branch.
    /// Shared endpoint identities supply incidence and reduce the equation count.
    Colinear([LineId; 2], ParallelDirection),
    /// Two line identities and their stored direction branch.
    Parallel([LineId; 2], ParallelDirection),
    /// A line identity and its target model-space length.
    Length(LineId, MinimumLength),
    /// Line identity, coordinate axis, and stored direction sign.
    ParallelToAxis(LineId, CoordinateAxis, AxisDirection),
    /// Line identity and the model-space plane normal.
    ParallelToPlane(LineId, Direction3),
    /// Two line identities whose directions form a right angle.
    Perpendicular([LineId; 2]),
    /// Two line identities whose physical lengths agree.
    EqualLength([LineId; 2]),
    /// Two directed line identities and their open-interval angle in radians.
    Angle([LineId; 2], LineAngle),
    /// Directed endpoint tangency at a common authored point.
    Tangent(ContinuityJoin),
    /// Directed tangent and curvature continuity at a common authored point.
    G2(ContinuityJoin, CurvatureTolerance),
}

/// A relation edit in an atomic authoring batch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RelationEdit {
    /// Insert a relation at a fresh ID.
    Insert(RelationId, RelationSpec),
    /// Remove the relation at an existing ID.
    Remove(RelationId),
}

/// Caller-selected units, acceptance tolerances and work envelope for one model revision.
#[derive(Clone, Copy, Debug)]
pub struct ConstraintPolicy {
    /// Scale for model length residuals and spatial variable steps, in model units.
    pub length_scale: MinimumLength,
    /// Inclusive tolerance in model units for coincidence, length relations and rigid placement.
    pub length_tolerance: MinimumLength,
    /// Maximum sine of the deviation from the required angular condition, in (0, 1).
    /// Angle relations also use this value as the exclusive minimum sine at seeds and trials.
    pub parallel_sine_tolerance: f64,
    /// The solver's dimensionless tolerances and iteration limits.
    pub solve: SolveOptions,
    /// The prepared solver's structural and per-tick limits.
    pub preview: SolverLimits,
    /// Maximum number of entities and authored relations traversed for final admission.
    pub validation_items: usize,
    /// Maximum charged scalar work for built-in residual evaluators in a preview tick.
    pub residual_work: usize,
}

/// Numerical rank and remaining variable directions at an accepted sketch state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstraintRank {
    /// Number of independent hard equations, including distinct fixed coordinates.
    pub rank: usize,
    /// Remaining numerical coordinate directions.
    pub degrees_of_freedom: usize,
    /// Number of authored scalar equations, including repeated fixed coordinates.
    pub equations: usize,
    /// Coordinates fixed by captured external geometry, independent of authored relations.
    pub frozen_coordinates: usize,
}

/// An atomically accepted preview and its numerical evidence.
pub struct DragUpdate {
    /// The moved sketch with the original geometry and relation identities.
    pub sketch: ConstrainedSketch,
    /// Numerical constraint rank on the accepted coordinates.
    pub rank: ConstraintRank,
    /// Largest dimensionless hard residual reported by the solver on the accepted coordinates.
    pub solver_hard_error: f64,
    /// Charged scalar work bound for the built-in residuals, including rejected trials.
    pub residual_work_bound: usize,
}

/// An independently admitted geometry snapshot and its immutable prepared relation program.
/// Every published snapshot satisfies the supplied physical tolerances and exact Fix anchors.
#[derive(Clone)]
pub struct ConstrainedSketch {
    geometry: Arc<Snapshot>,
    relations: Arc<BTreeMap<RelationId, Relation>>,
    program: Arc<Program>,
    policy: ConstraintPolicy,
}

impl ConstrainedSketch {
    /// Prepare existing admitted geometry with an empty relation set.
    /// Example: `ConstrainedSketch::new(snapshot, policy)?`.
    pub fn new(geometry: Snapshot, policy: ConstraintPolicy) -> Result<Self, ConstraintFailure> {
        let relations = BTreeMap::new();
        let program = Program::new(&geometry, &relations, policy)?;
        Ok(Self {
            geometry: Arc::new(geometry),
            relations: Arc::new(relations),
            program: Arc::new(program),
            policy,
        })
    }

    /// Borrow the admitted geometry. Example: `sketch.geometry().point(point)`.
    pub fn geometry(&self) -> &Snapshot {
        &self.geometry
    }

    /// Enumerate authored relations in relation-key order. Example: `sketch.relations().count()`.
    pub fn relations(&self) -> impl Iterator<Item = (RelationId, Relation)> + '_ {
        self.relations.iter().map(|(&id, &relation)| (id, relation))
    }

    /// Apply an authoring batch and solve it noninteractively before adoption.
    /// Plane-parallel and perpendicular relations may initialize a flat seed at a free endpoint.
    /// Example: `sketch.apply(&[RelationEdit::Insert(id, intent)], || false)?`.
    pub fn apply(
        &self,
        edits: &[RelationEdit],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, ConstraintFailure> {
        poll(&mut cancelled)?;
        let relations = self.stage(edits, &mut cancelled)?;
        let program = Arc::new(Program::new(&self.geometry, &relations, self.policy)?);
        let references = edits.iter().filter_map(|edit| match edit {
            RelationEdit::Insert(id, _) => match relations.get(id)? {
                Relation::Tangent(join) | Relation::G2(join, _) => Some((*id, join.reference())),
                _ => None,
            },
            RelationEdit::Remove(_) => None,
        });
        let coordinates = program.establish(
            &self.geometry,
            references,
            self.policy.solve,
            &mut cancelled,
        )?;
        let geometry = program.adopt(
            &self.geometry,
            &relations,
            &coordinates,
            self.policy,
            &mut cancelled,
        )?;
        let program = program
            .reframe(&geometry, self.policy, None, &mut cancelled)?
            .program;
        Ok(Self {
            geometry: Arc::new(geometry),
            relations: Arc::new(relations),
            program,
            policy: self.policy,
        })
    }

    /// Move one point with a bounded preview using this accepted revision as the warm state.
    /// Example: `let moved = sketch.drag(point, target, || false)?.sketch;`.
    pub fn drag(
        &self,
        point: PointId,
        target: Point3,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DragUpdate, ConstraintFailure> {
        poll(&mut cancelled)?;
        let output = self
            .program
            .drag(&self.geometry, point, target, &mut cancelled)?;
        let geometry = self.program.adopt(
            &self.geometry,
            &self.relations,
            &output.values,
            self.policy,
            &mut cancelled,
        )?;
        let next =
            self.program
                .reframe(&geometry, self.policy, Some(output.work), &mut cancelled)?;
        let mut output = output;
        output.work = next.work;
        output.hard_error = output.hard_error.max(next.hard_error);
        Ok(self.drag_update(geometry, next.program, output))
    }

    fn drag_update(
        &self,
        geometry: Snapshot,
        program: Arc<Program>,
        output: martensite_tksolve::preview::Update,
    ) -> DragUpdate {
        let rank = program.rank_report(output.rank, output.dof);
        let sketch = Self {
            geometry: Arc::new(geometry),
            program,
            ..self.clone()
        };
        DragUpdate {
            sketch,
            rank,
            solver_hard_error: output.hard_error,
            residual_work_bound: residual::work_bound(output.work.work),
        }
    }

    /// Run the explicit noninteractive rank check on the accepted geometry.
    /// Example: `let dof = sketch.inspect_rank()?.degrees_of_freedom;`.
    pub fn inspect_rank(&self) -> Result<ConstraintRank, ConstraintFailure> {
        self.program.inspect(&self.geometry)
    }

    fn stage(
        &self,
        edits: &[RelationEdit],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<BTreeMap<RelationId, Relation>, ConstraintFailure> {
        let mut relations = (*self.relations).clone();
        let mut retired = BTreeSet::new();
        for &edit in edits {
            poll(cancelled)?;
            match edit {
                RelationEdit::Insert(id, _)
                    if relations.contains_key(&id) || retired.contains(&id) =>
                {
                    return Err(ConstraintFailure::DuplicateId(id));
                }
                RelationEdit::Insert(id, spec) => {
                    relations.insert(id, binding::capture(&self.geometry, id, spec)?);
                }
                RelationEdit::Remove(id) => {
                    relations
                        .remove(&id)
                        .ok_or(ConstraintFailure::MissingRelation(id))?;
                    retired.insert(id);
                }
            }
        }
        Ok(relations)
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), ConstraintFailure> {
    if cancelled() {
        return Err(ConstraintFailure::Cancelled);
    }
    Ok(())
}
