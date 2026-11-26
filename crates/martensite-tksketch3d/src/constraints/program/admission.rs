// SPDX-License-Identifier: GPL-2.0-only
//! Limit the prepared solver to the admitted authored geometry and coordinates.
use super::{BTreeMap, ConstraintPolicy, EntityId, Failure, Relation, RelationId, Snapshot};
use crate::constraints::PolicyQuantity;

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
