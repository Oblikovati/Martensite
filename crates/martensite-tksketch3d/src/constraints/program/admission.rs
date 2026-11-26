// SPDX-License-Identifier: GPL-2.0-only
//! Limit the prepared solver to the admitted authored geometry and coordinates.
use super::{
    BTreeMap, BTreeSet, ConstraintPolicy, EntityId, Failure, Relation, RelationId, Snapshot,
};
use crate::{
    ArcId, PointId,
    constraints::{ArcConstraintFailure, PolicyQuantity},
};

pub(super) fn admit(
    geometry: &Snapshot,
    relations: &BTreeMap<RelationId, Relation>,
    policy: ConstraintPolicy,
    with_attachment: bool,
) -> Result<(), Failure> {
    let sine = policy.parallel_sine_tolerance;
    if !(sine > 0. && sine < 1.) {
        return Err(Failure::InvalidParallelSine(sine));
    }
    if !(1. / policy.length_scale.value()).is_finite() {
        return Err(Failure::InvalidLengthScale(policy.length_scale.value()));
    }
    bound(
        PolicyQuantity::ValidationItems,
        geometry
            .entities
            .len()
            .saturating_add(relations.len())
            .saturating_add(usize::from(with_attachment)),
        policy.validation_items,
    )?;
    let points = geometry
        .owned_ids()
        .filter(|entity| matches!(entity, EntityId::Point(_)))
        .count();
    bound(
        PolicyQuantity::Coordinates,
        points
            .saturating_mul(3)
            .saturating_add(2 * usize::from(with_attachment)),
        policy.preview.coordinates,
    )
}

fn bound(quantity: PolicyQuantity, requested: usize, maximum: usize) -> Result<(), Failure> {
    if requested > maximum {
        return Err(Failure::Limit {
            quantity,
            requested,
            maximum,
        });
    }
    Ok(())
}

pub(super) fn check_crossing_attachment(
    geometry: &Snapshot,
    crossings: &BTreeSet<ArcId>,
    relation: RelationId,
    point: PointId,
) -> Result<(), Failure> {
    // Surface rows are appended after the ordinary crossing scope has been checked.
    let crossing = crossings.iter().copied().find(|arc| {
        geometry
            .arc(*arc)
            .is_some_and(|geometry| geometry.recipe().points().contains(&point))
    });
    if let Some(arc) = crossing {
        return Err(Failure::Arc(
            arc,
            ArcConstraintFailure::CrossingCoupledRelation(relation),
        ));
    }
    Ok(())
}
