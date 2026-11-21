//! Stable authored-key construction and one retained archive round trip.
use super::{stages::SupportStages, *};
use crate::surface_sketch::encoding;
use family::{Control, FamilyStage};
use martensite_tkbrep::topods::{Builder, Compound};
pub(in crate::surface_sketch::wrapped_profile) fn construct<S: SupportStages>(
    support: &S,
    prepared: &[S::Prepared],
    control: &mut Control<'_>,
) -> Result<family::freeze::Frozen, S::Error> {
    let (root, bindings) = assemble(support, prepared, control)?;
    control.run_for::<_, S::Error>(FamilyStage::Encoding, None, |_| encode::<S>(root, bindings))
}
fn assemble<S: SupportStages>(
    support: &S,
    prepared: &[S::Prepared],
    control: &mut Control<'_>,
) -> Result<frozen_placement::DecodedGraph, S::Error> {
    let mut ordered: Vec<_> = prepared.iter().collect();
    ordered.sort_by_key(|request| request.as_ref().region);
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut root = compound.into_shape();
    let mut bindings = Vec::new();
    for request in ordered {
        let region = request.as_ref().region;
        let member =
            control.run_for::<_, S::Error>(FamilyStage::Construction, Some(region), |_| {
                let member = support.construct(request)?;
                Builder
                    .add(&mut root, &member.solid)
                    .map_err(|cause| S::assembly(region, cause))?;
                Ok(member)
            })?;
        bindings.extend(member.bindings);
    }
    bindings.sort_by_key(|(key, _)| *key);
    Ok((root, bindings))
}
fn encode<S: SupportStages>(
    root: Shape,
    bindings: Vec<(BindingKey, Shape)>,
) -> Result<family::freeze::Frozen, S::Error> {
    let bytes = encoding::encode(&root, &bindings).map_err(S::encoding)?;
    let keys: Vec<_> = bindings.iter().map(|(key, _)| *key).collect();
    let (root, bindings) = encoding::decode_with_policies(
        &bytes,
        &keys,
        martensite_tkbrep::bintools::SurfaceFrameReadPolicy::PreserveStoredV1,
        martensite_tkbrep::bintools::LocationTransformReadPolicy::PreserveStoredRigidV1,
    )
    .map_err(S::encoding)?;
    Ok((bytes, keys, root, bindings))
}
