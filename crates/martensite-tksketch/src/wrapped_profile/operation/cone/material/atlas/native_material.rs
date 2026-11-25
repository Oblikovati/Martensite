//! Share one checked planar program across all native caps and their source owners.
use super::*;
use martensite_tkbo::{
    body_check::CertifiedPlanarMaterial,
    profile_extrusion::{PlanarRegionSet, ProfileCurveId},
};
pub(super) type Selected = BTreeMap<SurfaceRegionId, (CertifiedPlanarMaterial, NativeCurveMap)>;

pub(super) fn select(
    members: &[Member<'_>],
    profiles: &[Request],
    planar: &periodic::PlanarSelectionEvidence,
    set: &PlanarRegionSet,
) -> ConeChecked<Selected> {
    let region = profiles[0].region;
    let ids = profiles
        .iter()
        .map(|source| {
            planar
                .members
                .get(&source.region)
                .map(|proof| proof.selected_region)
                .ok_or_else(|| mismatch(source.region, None, "selected periodic cap material"))
        })
        .collect::<ConeChecked<Vec<_>>>()?;
    let materials = CertifiedPlanarMaterial::from_periodic_regions(set, &ids)
        .map_err(|error| storage::failure(region, None, ConeCause::Native(Box::new(error))))?;
    members
        .iter()
        .zip(profiles)
        .zip(materials)
        .map(|(((prepared, _), profile), material)| {
            Ok((profile.region, (material, curve_ids(prepared, profile)?)))
        })
        .collect()
}

fn curve_ids(prepared: &Prepared, profile: &Request) -> ConeChecked<NativeCurveMap> {
    let region = prepared.source.region;
    let family = prepared
        .directed_family
        .as_deref()
        .ok_or_else(|| mismatch(region, None, "directed atlas native inventory"))?;
    let source = prepared.source.loops.iter().flatten();
    let names = profile.loops.iter().flatten();
    source
        .zip(names)
        .map(|(piece, name)| {
            let curve = ProfileCurveId(name.span.curve.0);
            let side = family.inventory.side(region, piece.span)?;
            Ok((curve, side))
        })
        .collect()
}
