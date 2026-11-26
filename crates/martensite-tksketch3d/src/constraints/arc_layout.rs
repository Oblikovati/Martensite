// SPDX-License-Identifier: GPL-2.0-only
//! Bind authored midpoint witnesses to the sagitta chart without moving the source points.
use super::{ConstraintFailure as Failure, ConstraintPolicy, RelationId, poll};
use crate::{ArcDefinition, ArcGeometry, ArcId, EntityId, MinimumLength, PointId, Snapshot};
use martensite_tkmath::predicates::{
    ArcTripleFailure, ArcTripleMetrics, compare_length_difference_3d,
};
use std::{cmp::Ordering, f64::consts::PI, fmt};

/// The directed sweep branch retained by a constrained three-point arc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcBranch {
    /// The through witness selects a sweep less than a half turn.
    Minor,
    /// The through witness selects a sweep greater than a half turn.
    Major,
}

impl ArcBranch {
    fn from_order(order: Ordering) -> Result<Self, ArcConstraintFailure> {
        match order {
            Ordering::Less => Ok(Self::Minor),
            Ordering::Greater => Ok(Self::Major),
            Ordering::Equal => Err(ArcConstraintFailure::Semicircle),
        }
    }

    pub(super) fn matches(self, sagitta: f64, half_chord: f64) -> bool {
        match self {
            Self::Minor => sagitta < half_chord,
            Self::Major => sagitta > half_chord,
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
    pub(super) branch: ArcBranch,
}

impl ArcLayout {
    pub(super) fn capture(arc: ArcGeometry, policy: ConstraintPolicy) -> Result<Self, Failure> {
        let recipe = arc.recipe();
        let branch = checked_branch(arc, policy.length_tolerance)
            .map_err(|cause| Failure::Arc(recipe.id, cause))?;
        Ok(Self {
            id: recipe.id,
            points: recipe.points(),
            branch,
        })
    }

    pub(super) fn require_branch(self, expected: ArcBranch) -> Result<(), Failure> {
        if self.branch != expected {
            return Err(Failure::Arc(
                self.id,
                ArcConstraintFailure::BranchChanged {
                    expected,
                    actual: self.branch,
                },
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
            // Published source arcs already had their exact branch checked against the carrier.
            let expected = nominal_branch(previous).map_err(|cause| Failure::Arc(id, cause))?;
            ArcLayout::capture(arc, policy)?.require_branch(expected)?;
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
    let Ok(metrics) = metrics(arc) else {
        return false;
    };
    let segment = |length| [[0.; 3], [length, 0., 0.]];
    metrics.branch() == branch_order(branch)
        && nominal_branch(arc) == Ok(branch)
        && metrics.radius_in_band(target.value(), tolerance) == Ok(true)
        && matches!(
            compare_length_difference_3d(segment(arc.radius()), segment(target.value()), tolerance),
            Ok(Ordering::Less | Ordering::Equal)
        )
}

fn branch_order(branch: ArcBranch) -> Ordering {
    match branch {
        ArcBranch::Minor => Ordering::Less,
        ArcBranch::Major => Ordering::Greater,
    }
}

fn checked_branch(
    arc: ArcGeometry,
    tolerance: MinimumLength,
) -> Result<ArcBranch, ArcConstraintFailure> {
    if !matches!(arc.recipe().definition, ArcDefinition::Through { .. }) {
        return Err(ArcConstraintFailure::CenterDefinition);
    }
    let metrics = metrics(arc)?;
    let branch = ArcBranch::from_order(metrics.branch())?;
    let offset = metrics
        .midpoint_offset(tolerance.value())
        .map_err(ArcConstraintFailure::Metric)?;
    if offset.is_gt() {
        return Err(ArcConstraintFailure::WitnessOffset(tolerance));
    }
    if nominal_branch(arc) != Ok(branch) {
        return Err(ArcConstraintFailure::CarrierBranch(branch, arc.range()[1]));
    }
    Ok(branch)
}

fn nominal_branch(arc: ArcGeometry) -> Result<ArcBranch, ArcConstraintFailure> {
    ArcBranch::from_order(arc.range()[1].total_cmp(&PI))
}

fn metrics(arc: ArcGeometry) -> Result<ArcTripleMetrics, ArcConstraintFailure> {
    ArcTripleMetrics::new(arc.points().map(|point| *point.coordinates()))
        .map_err(ArcConstraintFailure::Metric)
}
