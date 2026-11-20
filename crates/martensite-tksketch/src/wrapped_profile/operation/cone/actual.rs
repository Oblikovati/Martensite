//! One family certificate is rebound inside each ordered native qualification.
use super::*;
use martensite_tkbo::body_check::{
    CertifiedConicalBody, ConicalNativeCap, ConicalNativeMember, bind_conical_actual_family,
    capture_conical_actual_family,
};
use std::sync::Arc;
pub(super) mod binding;
mod errors;
mod queries;
mod validation;
pub(in crate::surface_sketch::wrapped_profile) type NativeEvidence = Option<Arc<NativeFamily>>;
pub(in crate::surface_sketch::wrapped_profile) struct NativeFamily {
    body: Arc<CertifiedConicalBody>,
    members: BTreeMap<SurfaceRegionId, ConicalNativeMember>,
}
pub(super) fn certify_family(
    _request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    selection: &Selection,
    members: &BTreeMap<SurfaceRegionId, (Geometry, Shape)>,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<NativeEvidence> {
    let directed = queries::family(selection)?;
    let native = binding::members(selection, members, work)?;
    let captured = capture_conical_actual_family(&native)
        .map_err(|cause| errors::binding(&directed.inventory, cause))?;
    let sides = queries::sides(originals, selection, members, &captured)?;
    let options = remaining(work, originals)?;
    let certificate = certify(&captured, &sides, directed, options, work, cancelled)?;
    Ok(retain(work, certificate, native))
}
fn remaining(
    work: &storage::Work,
    originals: &[DevelopedRegionRequest],
) -> ConeChecked<ConeClearanceOptions> {
    let owner = originals
        .first()
        .map(|original| original.region)
        .ok_or_else(|| global_missing("actual family source owner"))?;
    work.remaining_checked(owner, None)
}
fn retain(
    work: &mut storage::Work,
    certificate: Arc<CertifiedConicalBody>,
    native: Vec<ConicalNativeMember>,
) -> NativeEvidence {
    work.actual_certificate = Some(Arc::clone(&certificate));
    let members = native
        .into_iter()
        .map(|member| (SurfaceRegionId(member.id.0), member))
        .collect();
    Some(Arc::new(NativeFamily {
        body: certificate,
        members,
    }))
}
fn certify(
    captured: &martensite_tkbo::body_check::ConicalNativeGeometry,
    sides: &[ConicalActualSideQuery<'_>],
    family: &prepare::directed::DirectedFamily,
    options: ConeClearanceOptions,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Arc<CertifiedConicalBody>> {
    captured.with_cap_queries(|caps, adjacent| {
        let query = ConicalActualPatchQuery {
            sides,
            caps,
            adjacent,
            gamma: &family.gamma,
            refinement: ConicalPrimitiveRefinement::HalfSplitV1,
            options,
        };
        issue(captured, query, &family.inventory, work, cancelled)
    })
}

fn issue(
    captured: &martensite_tkbo::body_check::ConicalNativeGeometry,
    query: ConicalActualPatchQuery<'_>,
    inventory: &prepare::directed::Inventory,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Arc<CertifiedConicalBody>> {
    let result = certify_conical_actual_patches(query, &mut *cancelled);
    let operations = result
        .as_ref()
        .map_or_else(|cause| cause.operations, |proof| proof.operations());
    errors::charge(work, operations, &result)?;
    let proof = result.map_err(|cause| errors::math(inventory, cause))?;
    let bound = bind_conical_actual_family(captured, &query, Arc::new(proof))
        .map_err(|cause| errors::binding(inventory, cause))?;
    Ok(Arc::new(bound))
}
pub(super) fn qualify(
    original: &DevelopedRegionRequest,
    _geometry: &Geometry,
    shape: &Shape,
    evidence: &NativeEvidence,
) -> ConeChecked<pipeline::NativeQualification> {
    let family = evidence
        .as_ref()
        .ok_or_else(|| missing(original.region, "complete family SI authority"))?;
    let member = family
        .members
        .get(&original.region)
        .ok_or_else(|| missing(original.region, "certified native member"))?;
    if !member.shape.is_same(shape) {
        return Err(missing(original.region, "native member root identity"));
    }
    let qualified = pipeline::qualify_native_with_certified_conical(original, member, &family.body)
        .map_err(|cause| {
            storage::failure(
                original.region,
                None,
                ConeCause::NativeQualification(Box::new(cause)),
            )
        })?;
    validation::require(original, &qualified.report)?;
    Ok(qualified)
}
fn missing(region: SurfaceRegionId, field: &'static str) -> ConeFailure {
    storage::mismatch(region, None, field, vec![], vec![1])
}

pub(in crate::surface_sketch::wrapped_profile) fn placed_members(
    result: &WrappedConeNormalResult,
    bindings: &[(BindingKey, Shape)],
) -> Checked<BTreeMap<SurfaceRegionId, ConicalNativeMember>> {
    binding::placed_members(&result.family.family, bindings).map_err(ConeFailure::public)
}

pub(super) fn validate_family(
    family: &super::super::stages::OwnedFamily<Selection, Geometry>,
    work: &storage::Work,
) -> ConeChecked<()> {
    if !family.selection.executed_selection.is_directed() {
        return Ok(());
    }
    let body = work
        .actual_certificate
        .as_ref()
        .ok_or_else(|| global_missing("complete actual family publication certificate"))?;
    let bindings = binding::published_members(family, work)?;
    let inventory = &queries::family(&family.selection)?.inventory;
    body.rebind_family(&bindings)
        .map_err(|cause| errors::binding(inventory, cause))
}

fn global_missing(field: &'static str) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Binding {
            field,
            actual: vec![],
            expected: vec![1],
        },
    )
}
