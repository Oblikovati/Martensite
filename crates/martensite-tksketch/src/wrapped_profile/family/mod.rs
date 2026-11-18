//! One frozen transport table for all selected material components (#418).
use super::*;
use std::collections::BTreeMap;
mod budgeted;
mod cancellation;
pub(super) mod control;
pub(super) use budgeted::BudgetedFamily;
#[cfg(test)]
pub(super) use budgeted::{construct_budgeted, construct_budgeted_with_cancel};
mod error;
pub(super) use control::{Checkpoint, Control, FamilyStage};
pub(super) mod freeze;
pub(super) use error::FamilyFailure;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod isolation_tests;
#[cfg(test)]
mod replay_tests;
#[cfg(test)]
pub(super) mod tests;
type Checked<T> = Result<T, FamilyFailure>;
pub(super) type UnqualifiedFamily =
    operation::stages::OwnedFamily<selection::QualifiedSelection, pipeline::MemberGeometry>;

#[cfg(test)]
pub(super) fn construct(requests: &[Request]) -> Checked<UnqualifiedFamily> {
    construct_with_cancel(requests, |_| false)
}
#[cfg(test)]
pub(super) fn construct_with_cancel(
    requests: &[Request],
    mut cancelled: impl FnMut(Checkpoint) -> bool + Send,
) -> Checked<UnqualifiedFamily> {
    let mut control = Control::new(&mut cancelled);
    let family = operation::construct_controlled(
        requests,
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
        &mut control,
    )?;
    control.run(FamilyStage::Completion, None, |_| Ok(family))
}
#[cfg(test)]
fn store(
    originals: &[Request],
    prepared: &[Request],
    views: Vec<coverage::MemberView>,
) -> Checked<BTreeMap<SurfaceRegionId, pipeline::StoredMember>> {
    let mut cancelled = |_| false;
    operation::members::store(
        &operation::cylinder::CylinderStages {
            work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
            volume:
                martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit::Report,
        },
        originals,
        prepared,
        views,
        &mut (),
        &mut Control::new(&mut cancelled),
    )
}

pub(super) fn bind(
    originals: &[Request],
    selection: &selection::QualifiedSelection,
    stored: &BTreeMap<SurfaceRegionId, pipeline::StoredMember>,
) -> Checked<()> {
    let proofs = stored
        .iter()
        .map(|(id, member)| (*id, member.storage.as_slice()))
        .collect();
    planar_clearance::bind_many(
        &selection.prepared.boundary_clearance,
        originals,
        &selection.prepared.prepared,
        &proofs,
    )
    .map_err(|cause| FamilyFailure::Binding(Box::new(cause)))
}
pub(super) fn membership(region: SurfaceRegionId) -> FamilyFailure {
    FamilyFailure::Native {
        region: Some(region),
        cause: Box::new(Failure::new(&format!(
            "family member={region:?}; expected one original, prepared and decoded member"
        ))),
    }
}
