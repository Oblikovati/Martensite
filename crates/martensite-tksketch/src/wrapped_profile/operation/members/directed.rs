//! Explicit certified mode completes family geometry before ordered member checks.
use super::*;
type GeometryMembers<S> = BTreeMap<SurfaceRegionId, (<S as SupportStages>::Geometry, Shape)>;

pub(super) fn finish<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    selection: &S::Selection,
    mut stored: BTreeMap<SurfaceRegionId, S::Stored>,
    context: &mut S::Context,
    control: &mut Control<'_>,
) -> Result<Members<S::Geometry>, S::Error> {
    let prepared = S::prepared(selection);
    let geometry = geometry(support, originals, prepared, &mut stored, context, control)?;
    let evidence = control.run_for::<_, S::Error>(FamilyStage::MemberChecks, None, |poll| {
        support.certify_native_family(originals, selection, &geometry, context, poll)
    })?;
    qualify(support, originals, prepared, geometry, &evidence, control)
}
fn geometry<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    prepared: &[S::Prepared],
    stored: &mut BTreeMap<SurfaceRegionId, S::Stored>,
    context: &mut S::Context,
    control: &mut Control<'_>,
) -> Result<GeometryMembers<S>, S::Error> {
    let mut geometry = BTreeMap::new();
    for (original, retained) in originals.iter().zip(prepared) {
        let region = original.as_ref().region;
        let member = stored.remove(&region).ok_or_else(|| S::missing(region))?;
        let value =
            control.run_for::<_, S::Error>(FamilyStage::MemberChecks, Some(region), |poll| {
                support.geometry(original, retained, member, context, poll)
            })?;
        geometry.insert(region, value);
    }
    Ok(geometry)
}
fn qualify<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    prepared: &[S::Prepared],
    mut geometry: GeometryMembers<S>,
    evidence: &S::NativeEvidence,
    control: &mut Control<'_>,
) -> Result<Members<S::Geometry>, S::Error> {
    let mut members = BTreeMap::new();
    for (original, retained) in originals.iter().zip(prepared) {
        let region = original.as_ref().region;
        let (geometry, shape) = geometry.remove(&region).ok_or_else(|| S::missing(region))?;
        let native =
            control.run_for::<_, S::Error>(FamilyStage::MemberChecks, Some(region), |_| {
                support.qualify_native(original, retained, &geometry, &shape, evidence)
            })?;
        members.insert(region, QualifiedMember { geometry, native });
    }
    Ok(members)
}
