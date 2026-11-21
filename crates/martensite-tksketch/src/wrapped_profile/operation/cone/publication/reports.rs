//! Publication reads retained receipts without copying the operation's evidence maps.
use super::*;
type Member = super::super::super::stages::QualifiedMember<Geometry>;
type ConeReport = WrappedConeMemberReport;

pub(super) fn members(
    family: &ConeBudgeted,
) -> ConeChecked<BTreeMap<SurfaceRegionId, WrappedConeMemberReport>> {
    family
        .family
        .members
        .iter()
        .map(|(region, member)| indexed_report(family, *region, member))
        .collect()
}

fn indexed_report(
    family: &ConeBudgeted,
    region: SurfaceRegionId,
    member: &Member,
) -> ConeChecked<(SurfaceRegionId, ConeReport)> {
    let planar = family
        .family
        .selection
        .planar
        .get(&region)
        .ok_or_else(|| missing(region))?;
    report(member, planar, &member.geometry)
        .map(|report| (region, report))
        .map_err(|cause| with_owner(cause, region))
}

fn report(
    member: &Member,
    planar: &planar::PlanarEvidence,
    geometry: &Geometry,
) -> ConeChecked<ConeReport> {
    let storage = storage_bounds(geometry)?;
    Ok(ConeReport {
        planar_area: planar.area,
        coverage: coverage(geometry),
        validation: member.native.report.clone(),
        boundary_error: bound(boundary_error(geometry)?),
        cap_trace_error: bound(trace_error(geometry)),
        native_tolerance: native_tolerance(geometry),
        depth_storage_error: storage.0.map(bound),
        ruled_storage_error: bound(storage.1),
        storage_allowance: storage.2,
        authored_volume: measures::range(geometry.authored.volume())?,
        boundary_flux: api::reports::range(geometry.flux.enclosure()),
        reference_volume: api::reports::range(geometry.reference_volume.enclosure()),
        volume: measures::member_volume(geometry)?,
        native_volume: native_estimate(member),
        arithmetic_operations: geometry.arithmetic_operations,
        join_arithmetic_operations: geometry.joins.operations(),
        material: material_report(geometry),
    })
}

pub(super) fn storage_bounds(geometry: &Geometry) -> ConeChecked<([f64; 2], f64, f64)> {
    let first = &geometry.spans.first().ok_or_else(missing_storage)?.storage;
    Ok(geometry.spans.iter().fold(
        (first.depth_edges, first.ruled_side, first.allowance),
        |(depths, ruling, allowance), span| {
            let proof = &span.storage;
            let depths = std::array::from_fn(|depth| depths[depth].max(proof.depth_edges[depth]));
            (
                depths,
                ruling.max(proof.ruled_side),
                allowance.min(proof.allowance),
            )
        },
    ))
}

fn missing_storage() -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Scalar {
            field: "stored span count",
            actual: 0.,
            expected: "nonempty retained storage receipts",
        },
    )
    .at(WrappedStage::Publication)
}

fn material_report(geometry: &Geometry) -> WrappedConeMaterialReport {
    let material = &geometry.material;
    WrappedConeMaterialReport {
        planar_proof: material.proof_version(),
        planar: material.census,
        planar_work: material.planar_work,
        chart_admission_tolerance: material.chart_admission_tolerance(),
        cap_realization_error: bound(material.spatial_error()),
    }
}

fn native_tolerance(geometry: &Geometry) -> f64 {
    geometry
        .spans
        .iter()
        .map(|span| span.maximum_tolerance)
        .fold(0., f64::max)
}

fn coverage(geometry: &Geometry) -> WrappedIncidenceCoverage {
    let graph = &geometry.coverage;
    WrappedIncidenceCoverage {
        bindings: graph.binding_count,
        faces: graph.faces,
        wires: graph.wires,
        edges: graph.edges,
        vertices: graph.vertices,
        edge_uses: graph.edge_uses,
        euler_characteristic: graph.euler_characteristic,
        cap_pcurves: geometry.spans.iter().map(|span| span.cap_pcurves).sum(),
        side_pcurves: geometry.spans.iter().map(|span| span.side_pcurves).sum(),
    }
}

fn native_estimate(member: &Member) -> WrappedNativeVolumeEstimate {
    WrappedNativeVolumeEstimate {
        value: member.native.volume_estimate,
        integration_error: member.native.integration_estimate,
    }
}

fn trace_error(geometry: &Geometry) -> f64 {
    geometry
        .spans
        .iter()
        .flat_map(|span| &span.trims)
        .map(|trim| trim.residual_bounds()[2])
        .fold(0., f64::max)
}

fn boundary_error(geometry: &Geometry) -> ConeChecked<f64> {
    geometry.spans.iter().try_fold(0_f64, |maximum, span| {
        measures::upper(span.boundary.total())
            .map(|upper| maximum.max(upper))
            .map_err(|mut cause| {
                cause.span = Some(span.span);
                cause
            })
    })
}

fn with_owner(mut cause: ConeFailure, region: SurfaceRegionId) -> ConeFailure {
    cause.region.get_or_insert(region);
    cause
}

pub(super) fn budget(
    request: &WrappedConeNormalRequest,
    family: &ConeBudgeted,
) -> WrappedConeBudgetReport {
    WrappedConeBudgetReport {
        planar: family.family.selection.census,
        source_planar_work: family.family.selection.work,
        planar_work: family.budget.planar_work,
        arithmetic_operations: family.budget.arithmetic_operations,
        integration_subintervals: family.budget.integration_subintervals,
        requested_spatial_error: request.options.spatial_error,
        achieved_spatial_error: bound(family.budget.spatial_error),
        reference_developed_error: bound(family.budget.reference.developed_error()),
        reference_filled_error: bound(family.budget.reference.filled_error()),
        reference_storage_error: bound(family.budget.reference.storage_error()),
        cap_reference_proxy_error: bound(family.budget.cap_reference_proxy_error),
        volume_policy: request.options.volume,
        volume: family.budget.volume.clone(),
    }
}

fn bound(upper: f64) -> WrappedCertifiedBound {
    WrappedCertifiedBound { upper }
}

fn missing(region: SurfaceRegionId) -> ConeFailure {
    ConeFailure::new(
        Some(region),
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Missing(region),
    )
    .at(WrappedStage::Publication)
}
