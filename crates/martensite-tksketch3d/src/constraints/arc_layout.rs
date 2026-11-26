// SPDX-License-Identifier: GPL-2.0-only
//! Bind authored midpoint witnesses to the sagitta chart without moving the source points.
use super::{ConstraintFailure as Failure, ConstraintPolicy, RelationId, poll};
use crate::{ArcDefinition, ArcGeometry, ArcId, EntityId, MinimumLength, PointId, Snapshot};
use martensite_tkmath::predicates::{
    ArcTripleFailure, ArcTripleMetrics, compare_length_difference_3d,
};
use std::{cmp::Ordering, collections::BTreeSet, f64::consts::PI, fmt};

/// The directed sweep policy retained by an authored arc relation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcBranch {
    /// The through witness selects a sweep less than a half turn.
    Minor,
    /// The through witness selects a sweep greater than a half turn.
    Major,
    /// A Radius captured at an exact half turn permits both directions of sweep change.
    /// The connected layout requires a free witness and at least one free chord endpoint.
    CrossHalfTurn,
}

impl ArcBranch {
    fn strict(sweep: Ordering) -> Option<Self> {
        match sweep {
            Ordering::Less => Some(Self::Minor),
            Ordering::Greater => Some(Self::Major),
            Ordering::Equal => None,
        }
    }

    fn admits(self, sweep: Ordering) -> bool {
        match self {
            Self::Minor => sweep.is_lt(),
            Self::Major => sweep.is_gt(),
            Self::CrossHalfTurn => true,
        }
    }

    pub(super) fn matches(self, sagitta: f64, half_chord: f64) -> bool {
        match self {
            Self::Minor => sagitta < half_chord,
            Self::Major => sagitta > half_chord,
            Self::CrossHalfTurn => true,
        }
    }
}

/// Cause of refusal in the midpoint-witness constraint chart.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum ArcConstraintFailure {
    /// A center-authored recipe has no constraint chart in this solver.
    CenterDefinition,
    /// The chordwise offset of the witness exceeds the stated length limit.
    WitnessOffset(MinimumLength),
    /// The through witness lies exactly on the directed half-turn boundary.
    Semicircle,
    /// The stored nominal trim and exact authored triangle disagree on their branch.
    CarrierBranch(ArcBranch, f64),
    /// An exact authored half turn lacks a native end angle of PI.
    HalfTurnCarrier(f64),
    /// Only a Radius relation can carry an explicit half-turn crossing policy.
    CrossingRequiresRadius,
    /// A strict sweep policy conflicts with the arc's authored crossing permission.
    ConflictingSweep(ArcBranch),
    /// A second arc uses a crossing point and introduces its own sweep constraint.
    AliasedCrossing(ArcId),
    /// Both chord endpoints are pinned, leaving disconnected branches at a fixed radius.
    LockedCrossing([PointId; 2]),
    /// A crossing requires the witness to be free.
    CrossingPinnedWitness(PointId),
    /// Another incident curve is outside the connected crossing layout.
    CrossingSharedEntity(EntityId),
    /// The relation prevents certification of a connected crossing layout.
    CrossingCoupledRelation(RelationId),
    /// A bare half turn has no authored Radius permitting departure.
    HalfTurnDeparture(ArcBranch),
    /// A solve or restored relation would switch the authored branch.
    BranchChanged {
        /// Branch carried by the accepted geometry or persisted relation.
        expected: ArcBranch,
        /// Branch of the proposed geometry.
        actual: ArcBranch,
    },
    /// The authored triangle could not be measured exactly.
    Metric(ArcTripleFailure),
}

impl fmt::Display for ArcConstraintFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "midpoint arc constraint chart declined: {self:?}"
        )
    }
}

impl std::error::Error for ArcConstraintFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Metric(source) => Some(source),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ArcLayout {
    pub(super) id: ArcId,
    pub(super) points: [PointId; 3],
    sweep: Ordering,
}

impl ArcLayout {
    pub(super) fn capture(arc: ArcGeometry, policy: ConstraintPolicy) -> Result<Self, Failure> {
        let recipe = arc.recipe();
        let sweep = checked_sweep(arc, policy.length_tolerance)
            .map_err(|cause| Failure::Arc(recipe.id, cause))?;
        Ok(Self {
            id: recipe.id,
            points: recipe.points(),
            sweep,
        })
    }

    pub(super) fn midpoint_key(self) -> ([PointId; 2], PointId) {
        let [start, end, witness] = self.points;
        let mut chord = [start, end];
        chord.sort_unstable();
        (chord, witness)
    }

    pub(super) fn circle_key(self) -> [PointId; 3] {
        let mut points = self.points;
        points.sort_unstable();
        points
    }

    pub(super) fn require_branch(self, expected: ArcBranch) -> Result<(), Failure> {
        if expected.admits(self.sweep) {
            return Ok(());
        }
        let actual = self.strict_branch()?;
        Err(Failure::Arc(
            self.id,
            ArcConstraintFailure::BranchChanged { expected, actual },
        ))
    }

    pub(super) fn strict_branch(self) -> Result<ArcBranch, Failure> {
        ArcBranch::strict(self.sweep).ok_or(Failure::Arc(self.id, ArcConstraintFailure::Semicircle))
    }

    pub(super) fn radius_branch(self) -> ArcBranch {
        ArcBranch::strict(self.sweep).unwrap_or(ArcBranch::CrossHalfTurn)
    }

    fn require_transition(self, previous: Ordering) -> Result<(), Failure> {
        if let Some(expected) = ArcBranch::strict(previous) {
            return self.require_branch(expected);
        }
        if let Some(actual) = ArcBranch::strict(self.sweep) {
            return Err(Failure::Arc(
                self.id,
                ArcConstraintFailure::HalfTurnDeparture(actual),
            ));
        }
        Ok(())
    }
}

pub(super) fn operand(
    snapshot: &Snapshot,
    relation: RelationId,
    arc: ArcId,
) -> Result<ArcGeometry, Failure> {
    snapshot
        .arc(arc)
        .ok_or(Failure::MissingOperand(relation, EntityId::Arc(arc)))
}

pub(super) fn verify_adopted(
    before: &Snapshot,
    after: &Snapshot,
    crossings: &BTreeSet<ArcId>,
    policy: ConstraintPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    for (&id, &arc) in &after.arcs {
        poll(cancelled)?;
        let Some(&previous) = before.arcs.get(&id) else {
            ArcLayout::capture(arc, policy)?;
            continue;
        };
        if arc != previous {
            let actual = ArcLayout::capture(arc, policy)?;
            // Previously published poses already passed the exact triangle/native agreement.
            if !crossings.contains(&id) {
                actual.require_transition(nominal_sweep(previous))?;
            }
        }
    }
    Ok(())
}

pub(super) fn radius_matches(
    arc: ArcGeometry,
    target: MinimumLength,
    branch: ArcBranch,
    policy: ConstraintPolicy,
) -> bool {
    let tolerance = policy.length_tolerance.value();
    on_branch(arc, branch).is_some_and(|metrics| {
        metrics.radius_in_band(target.value(), tolerance) == Ok(true)
            && nominal_radii_agree(arc.radius(), target.value(), tolerance)
    })
}

pub(super) fn equal_radius_matches(
    [first, second]: [ArcGeometry; 2],
    [first_branch, second_branch]: [ArcBranch; 2],
    policy: ConstraintPolicy,
) -> bool {
    let (Some(a), Some(b)) = (
        on_branch(first, first_branch),
        on_branch(second, second_branch),
    ) else {
        return false;
    };
    let tolerance = policy.length_tolerance.value();
    matches!(
        a.compare_radius_difference(&b, tolerance),
        Ok(Ordering::Less | Ordering::Equal)
    ) && nominal_radii_agree(first.radius(), second.radius(), tolerance)
}

pub(super) fn branch_matches(arc: ArcGeometry, branch: ArcBranch) -> bool {
    on_branch(arc, branch).is_some()
}

fn on_branch(arc: ArcGeometry, branch: ArcBranch) -> Option<ArcTripleMetrics> {
    let metrics = metrics(arc).ok()?;
    (branch.admits(metrics.branch()) && nominal_sweep(arc) == metrics.branch()).then_some(metrics)
}

fn nominal_radii_agree(first: f64, second: f64, tolerance: f64) -> bool {
    let segment = |length| [[0.; 3], [length, 0., 0.]];
    matches!(
        compare_length_difference_3d(segment(first), segment(second), tolerance),
        Ok(Ordering::Less | Ordering::Equal)
    )
}

fn checked_sweep(
    arc: ArcGeometry,
    tolerance: MinimumLength,
) -> Result<Ordering, ArcConstraintFailure> {
    if !matches!(arc.recipe().definition, ArcDefinition::Through { .. }) {
        return Err(ArcConstraintFailure::CenterDefinition);
    }
    let metrics = metrics(arc)?;
    let offset = metrics
        .midpoint_offset(tolerance.value())
        .map_err(ArcConstraintFailure::Metric)?;
    if offset.is_gt() {
        return Err(ArcConstraintFailure::WitnessOffset(tolerance));
    }
    verify_carrier(arc, metrics.branch())?;
    Ok(metrics.branch())
}

fn verify_carrier(arc: ArcGeometry, sweep: Ordering) -> Result<(), ArcConstraintFailure> {
    if nominal_sweep(arc) == sweep {
        return Ok(());
    }
    let stored = arc.range()[1];
    match ArcBranch::strict(sweep) {
        Some(branch) => Err(ArcConstraintFailure::CarrierBranch(branch, stored)),
        None => Err(ArcConstraintFailure::HalfTurnCarrier(stored)),
    }
}

fn nominal_sweep(arc: ArcGeometry) -> Ordering {
    arc.range()[1].total_cmp(&PI)
}

fn metrics(arc: ArcGeometry) -> Result<ArcTripleMetrics, ArcConstraintFailure> {
    ArcTripleMetrics::new(arc.points().map(|point| *point.coordinates()))
        .map_err(ArcConstraintFailure::Metric)
}
