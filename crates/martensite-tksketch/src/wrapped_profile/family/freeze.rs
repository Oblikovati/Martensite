//! Authored IDs determine serialization order; the compound has no fabricated identity.
use super::*;
use crate::surface_sketch::encoding;
use martensite_tkbrep::topods::{Builder, Compound};
pub(in crate::surface_sketch::wrapped_profile) type Frozen =
    (Vec<u8>, Vec<BindingKey>, Shape, Vec<(BindingKey, Shape)>);
#[cfg(test)]
pub(super) fn construct(requests: &[Request]) -> Checked<Frozen> {
    let mut cancelled = |_| false;
    construct_with_control(requests, &mut Control::new(&mut cancelled))
}
pub(in crate::surface_sketch::wrapped_profile) fn construct_with_control(
    requests: &[Request],
    control: &mut Control<'_>,
) -> Checked<Frozen> {
    let (root, bindings) = assemble(requests, control)?;
    control.run(FamilyStage::Encoding, None, |_| encode(root, bindings))
}
fn encode(root: Shape, bindings: Vec<(BindingKey, Shape)>) -> Checked<Frozen> {
    let bytes = encoding::encode(&root, &bindings)
        .map_err(|cause| FamilyFailure::Encoding(Box::new(cause)))?;
    let keys: Vec<_> = bindings.iter().map(|(key, _)| *key).collect();
    let (root, bindings) = encoding::decode_with_policies(
        &bytes,
        &keys,
        martensite_tkbrep::bintools::SurfaceFrameReadPolicy::PreserveStoredV1,
        martensite_tkbrep::bintools::LocationTransformReadPolicy::PreserveStoredRigidV1,
    )
    .map_err(|cause| FamilyFailure::Encoding(Box::new(cause)))?;
    Ok((bytes, keys, root, bindings))
}
fn assemble(
    requests: &[Request],
    control: &mut Control<'_>,
) -> Checked<(Shape, Vec<(BindingKey, Shape)>)> {
    let mut ordered: Vec<_> = requests.iter().collect();
    ordered.sort_by_key(|request| request.region);
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut root = compound.into_shape();
    let mut bindings = Vec::new();
    for request in ordered {
        let member = control.run(FamilyStage::Construction, Some(request.region), |_| {
            add_member(&mut root, request)
        })?;
        bindings.extend(member.bindings);
    }
    bindings.sort_by_key(|(key, _)| *key);
    Ok((root, bindings))
}
fn add_member(root: &mut Shape, request: &Request) -> Checked<Constructed> {
    let member =
        native::construct(request).map_err(|cause| native_at(Some(request.region), cause))?;
    Builder
        .add(root, &member.solid)
        .map_err(|cause| native_at(Some(request.region), cause))?;
    Ok(member)
}
fn native_at(region: Option<SurfaceRegionId>, cause: Failure) -> FamilyFailure {
    FamilyFailure::Native {
        region,
        cause: Box::new(cause),
    }
}
