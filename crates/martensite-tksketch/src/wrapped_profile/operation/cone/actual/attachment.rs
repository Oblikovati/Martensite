//! Retain the exact family and its individual span sources for attached construction.
use super::*;
use martensite_tkbo::body_check::{CertifiedPlanarMaterial, DirectedCapWork};

pub(in crate::surface_sketch::wrapped_profile) struct AttachmentAuthority {
    body: Arc<CertifiedConicalBody>,
    selected: BTreeMap<SurfaceRegionId, CertifiedPlanarMaterial>,
    prepared: Vec<Prepared>,
    members: BTreeMap<SurfaceRegionId, AttachmentMember>,
    residuals: BTreeMap<BindingKey, (SurfaceRegionId, usize, usize)>,
    work: AttachmentToolWork,
}

struct AttachmentMember {
    wedge: ConicalSectorWedge,
    spans: Vec<storage::SpanProof>,
    carriers: [ConicalTrimCarrier; 2],
}

pub(in crate::surface_sketch::wrapped_profile) struct AttachmentToolWork {
    pub authored: DirectedCapWork,
    pub caps: BTreeMap<SurfaceRegionId, DirectedCapWork>,
    pub planar: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    pub integration_cells: u32,
    pub actual_operations: u32,
    pub arithmetic_operations: u32,
}

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn issue(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    selected: super::super::attachment_tool::Selection,
    views: Vec<coverage::MemberView>,
    work: &mut storage::Work,
    control: &mut family::Control<'_>,
) -> ConeChecked<AttachmentAuthority> {
    let stages = stages::ConeStages {
        request,
        execution_selection: selected.execution,
    };
    let stored =
        operation::members::store(&stages, originals, &selected.prepared, views, work, control)?;
    storage::bind_geometry(
        originals,
        &selected.prepared,
        &selected.domain,
        &selected.clearance,
        &stored,
    )?;
    selected.authored.rebind(request, originals)?;
    let (caps, receipts) = capture_selected(request, &selected.prepared, &stored, work, control)?;
    let body = prove_family(originals, &selected, &stored, &caps, work, control)?;
    let residuals = residual_keys(&stored)?;
    let work = completed_work(work, &selected, receipts)?;
    let members = retain_members(&selected.prepared, stored)?;
    Ok(AttachmentAuthority {
        body,
        selected: caps,
        prepared: selected.prepared,
        members,
        residuals,
        work,
    })
}

fn completed_work(
    work: &storage::Work,
    selected: &super::super::attachment_tool::Selection,
    caps: BTreeMap<SurfaceRegionId, DirectedCapWork>,
) -> ConeChecked<AttachmentToolWork> {
    let authored = selected.authored.work();
    let planar = work
        .planar_receipt()
        .ok_or_else(|| global_missing("attached planar work ledger"))?;
    let cells = caps
        .values()
        .try_fold(authored.integration_cells, |sum, cap| {
            sum.checked_add(cap.integration_cells)
        });
    let pairs = caps
        .values()
        .try_fold(authored.pair.consumed_pair_bisections, |sum, cap| {
            sum.checked_add(cap.pair.consumed_pair_bisections)
        });
    if cells != Some(u64::from(work.integration_cells))
        || pairs != Some(planar.consumed_pair_bisections)
    {
        return Err(global_missing("closed attached authored/cap work ledger"));
    }
    Ok(AttachmentToolWork {
        authored,
        caps,
        planar,
        integration_cells: work.integration_cells,
        actual_operations: work.actual_operations,
        arithmetic_operations: work.consumed,
    })
}

fn prove_family(
    originals: &[DevelopedRegionRequest],
    selected: &super::super::attachment_tool::Selection,
    stored: &BTreeMap<SurfaceRegionId, storage::Stored>,
    caps: &BTreeMap<SurfaceRegionId, CertifiedPlanarMaterial>,
    work: &mut storage::Work,
    control: &mut family::Control<'_>,
) -> ConeChecked<Arc<CertifiedConicalBody>> {
    let views = borrowed_members(stored, caps);
    let selection = SelectedGeometry {
        execution: selected.execution,
        prepared: &selected.prepared,
    };
    let family = control
        .run_for::<_, ConeFailure>(family::FamilyStage::MemberChecks, None, |poll| {
            certify_views(originals, selection, &views, work, poll)
        })?
        .ok_or_else(|| global_missing("issued attached family SI authority"))?;
    Ok(Arc::clone(&family.body))
}

fn capture_selected(
    request: &ConeRun<'_>,
    prepared: &[Prepared],
    stored: &BTreeMap<SurfaceRegionId, storage::Stored>,
    work: &mut storage::Work,
    control: &mut family::Control<'_>,
) -> ConeChecked<(
    BTreeMap<SurfaceRegionId, CertifiedPlanarMaterial>,
    BTreeMap<SurfaceRegionId, DirectedCapWork>,
)> {
    let mut caps = BTreeMap::new();
    let mut receipts = BTreeMap::new();
    for member in prepared {
        let region = member.source.region;
        let native = stored
            .get(&region)
            .ok_or_else(|| missing(region, "attached stored spans"))?;
        let (material, receipt) = control.run_for::<_, ConeFailure>(
            family::FamilyStage::MemberChecks,
            Some(region),
            |poll| material::certify_attachment_uv(request, member, native, work, poll),
        )?;
        caps.insert(region, material);
        receipts.insert(region, receipt);
    }
    Ok((caps, receipts))
}

fn borrowed_members<'a>(
    stored: &'a BTreeMap<SurfaceRegionId, storage::Stored>,
    selected: &'a BTreeMap<SurfaceRegionId, CertifiedPlanarMaterial>,
) -> ActualMembers<'a> {
    stored
        .iter()
        .map(|(region, member)| {
            (
                *region,
                ActualMember {
                    shape: &member.shape,
                    spans: &member.spans,
                    joins: &member.joins,
                    material: CapMaterial {
                        selected: selected.get(region),
                        periodic_curves: None,
                    },
                },
            )
        })
        .collect()
}

fn residual_keys(
    stored: &BTreeMap<SurfaceRegionId, storage::Stored>,
) -> ConeChecked<BTreeMap<BindingKey, (SurfaceRegionId, usize, usize)>> {
    let mut bounds = BTreeMap::new();
    for (&region, member) in stored {
        for (index, span) in member.spans.iter().enumerate() {
            for (row, role) in [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge]
                .into_iter()
                .enumerate()
            {
                let key = BindingKey {
                    source: Source::Span(span.span),
                    role,
                };
                if !matches_trim(span, member.carriers[row], row)
                    || bounds.insert(key, (region, index, row)).is_some()
                {
                    return Err(missing(region, "attached actual cap trim residual source"));
                }
            }
        }
    }
    Ok(bounds)
}

fn matches_trim(span: &storage::SpanProof, carrier: ConicalTrimCarrier, row: usize) -> bool {
    let proof = &span.trims[row];
    let source = ConicalTrimQuery {
        curve: span.curves[row].borrowed(),
        pcurve: &span.pcurves[row],
        carrier,
        spatial_limit: proof.requested_spatial_limit(),
        options: proof.requested_options(),
    };
    trim_upper(proof).is_some() && proof.matches_source(source)
}

fn trim_upper(proof: &CertifiedConicalTrim) -> Option<f64> {
    let upper = proof.residual_bounds()[2];
    (upper.is_finite() && upper >= 0. && upper <= proof.requested_spatial_limit()).then_some(upper)
}

fn retain_members(
    prepared: &[Prepared],
    mut stored: BTreeMap<SurfaceRegionId, storage::Stored>,
) -> ConeChecked<BTreeMap<SurfaceRegionId, AttachmentMember>> {
    prepared
        .iter()
        .map(|member| {
            let region = member.source.region;
            let owner = stored
                .remove(&region)
                .ok_or_else(|| missing(region, "attached retained span owner"))?;
            Ok((
                region,
                AttachmentMember {
                    wedge: member.wedge,
                    spans: owner.spans,
                    carriers: owner.carriers,
                },
            ))
        })
        .collect()
}

impl AttachmentAuthority {
    pub(in crate::surface_sketch::wrapped_profile) fn rebind(
        &self,
        bindings: &[(BindingKey, Shape)],
    ) -> Checked<()> {
        let inventory = &queries::family(&self.prepared)
            .map_err(ConeFailure::public)?
            .inventory;
        let native = binding::replay(&self.prepared, &self.selected, bindings)
            .map_err(ConeFailure::public)?;
        self.body
            .rebind_family(&native)
            .map_err(|cause| errors::binding(inventory, cause).public())
    }

    pub(in crate::surface_sketch::wrapped_profile) fn body(&self) -> &Arc<CertifiedConicalBody> {
        &self.body
    }

    pub(in crate::surface_sketch::wrapped_profile) fn cap_residual_upper(
        &self,
        key: BindingKey,
    ) -> Option<f64> {
        let &(region, index, row) = self.residuals.get(&key)?;
        trim_upper(&self.members.get(&region)?.spans.get(index)?.trims[row])
    }

    pub(in crate::surface_sketch::wrapped_profile) fn work(&self) -> &AttachmentToolWork {
        &self.work
    }

    pub(in crate::surface_sketch::wrapped_profile) fn fitted(
        &self,
        region: SurfaceRegionId,
    ) -> Option<(
        ConicalSectorWedge,
        [ConicalTrimCarrier; 2],
        &[storage::SpanProof],
    )> {
        let member = self.members.get(&region)?;
        Some((member.wedge, member.carriers, &member.spans))
    }
}
