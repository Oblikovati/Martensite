//! Rebind the complete cap family and shared receipts before publication.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::stages::OwnedFamily;

pub(in super::super) fn validate(
    family: &ConeBudgeted,
    binding: &Binding,
    limit: u64,
) -> ConeChecked<()> {
    let prepared = &family.family.selection.prepared;
    let region = prepared
        .first()
        .ok_or_else(|| {
            refused(
                None,
                Refusal::Clearance(planar_clearance::ClearanceFailure::EmptySelection),
            )
        })?
        .source
        .region;
    let reference = &family.budget.reference;
    let caps = reference
        .decoded_caps()
        .filter(|caps| caps.len() == prepared.len())
        .ok_or_else(|| mismatch(region, None, "complete published decoded atlas"))?;
    let joins = prepared.iter().filter_map(|prepared| {
        family
            .family
            .members
            .get(&prepared.source.region)
            .map(|member| &member.geometry.joins)
    });
    if !reference.shares_decoded_join_issuance(joins) {
        return Err(mismatch(
            region,
            None,
            "complete decoded atlas reference join issuance",
        ));
    }
    relations(reference, binding, region, caps.len())?;
    let mut cells = PublishedCells {
        integral: None,
        total: 0,
    };
    for (prepared, cap) in prepared.iter().zip(caps) {
        cells.member(&family.family, prepared, cap, limit)?;
    }
    if cells.total != u64::from(family.budget.integration_subintervals) {
        return Err(mismatch(
            region,
            None,
            "one decoded atlas UV program and retained flux cell total",
        ));
    }
    Ok(())
}
fn relations(
    reference: &ConicalReferenceEvidence,
    binding: &Binding,
    region: SurfaceRegionId,
    count: usize,
) -> ConeChecked<()> {
    let scoped = binding.material.as_ref().map_or_else(
        || reference.has_separated_decoded_fill(),
        |material| material.probes.probes().len() == material.relations.len(),
    );
    if !binding
        .boundary
        .periodic
        .matches_boundary(&binding.boundary.certificate)
        || !scoped
        || (count > 1 && reference.decoded_solid_separation().is_none())
    {
        return Err(mismatch(
            region,
            None,
            "complete decoded atlas periodic and native solid relations",
        ));
    }
    Ok(())
}
struct PublishedCells<'a> {
    integral: Option<&'a martensite_tkbo::profile_extrusion::PlanarIntegralEvidence>,
    total: u64,
}
impl<'a> PublishedCells<'a> {
    fn member(
        &mut self,
        family: &'a OwnedFamily<Selection, Geometry>,
        prepared: &Prepared,
        cap: &ConicalDecodedCapEvidence,
        limit: u64,
    ) -> ConeChecked<()> {
        let region = prepared.source.region;
        let geometry = &family
            .members
            .get(&region)
            .ok_or_else(|| mismatch(region, None, "published atlas member"))?
            .geometry;
        let original =
            family.selection.planar.get(&region).ok_or_else(|| {
                mismatch(region, None, "published atlas original planar evidence")
            })?;
        let material = &geometry.material;
        if !original.matches_request(&material.original)
            || geometry.reference_volume.enclosure() != volume_bounds(cap)?
            || material.spatial_error().to_bits() != cap_error(cap)?.to_bits()
        {
            return Err(mismatch(
                region,
                None,
                "published atlas material and normal measures",
            ));
        }
        self.add(region, geometry, limit)
    }
    fn add(
        &mut self,
        region: SurfaceRegionId,
        geometry: &'a Geometry,
        limit: u64,
    ) -> ConeChecked<()> {
        let proof = geometry
            .material
            .integral_program_evidence()
            .ok_or_else(|| mismatch(region, None, "decoded atlas planar integral receipt"))?;
        if let Some(first) = self.integral {
            if !first.shares_issuance(proof) {
                return Err(mismatch(
                    region,
                    None,
                    "shared atlas planar integral issuance",
                ));
            }
        } else {
            program::validate_entry(region, proof, limit, 0)?;
            self.total = proof.integration_subintervals();
            self.integral = Some(proof);
        }
        self.total = program::member_cells(region, self.total, 0, geometry)?;
        Ok(())
    }
}
