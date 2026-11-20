//! Material disjointness of all original and periodic component families.
mod error;
mod jobs;
mod source;
mod types;
mod winding;
use super::*;
pub(in super::super) use error::MaterialFailure;
use martensite_tkmath::predicates::roots::{
    RootIsolationOptions,
    bezier_line::{
        BezierWireError, BezierWireLocation, BezierWireQuery, BezierWireSegment,
        classify_bezier_wire,
    },
    metric_bounds::{
        CertifiedCylindricalPeriodicClearance, CertifiedCylindricalProbes,
        CylindricalPeriodicProbe, CylindricalProbeError, CylindricalProbeOptions,
        CylindricalProbeSense,
    },
};
use types::Job;
pub(in super::super) use types::{
    LoopLocation, MaterialOptions, MaterialRelation, OutsideReason, PeriodicMaterialEvidence,
};
type Checked<T> = Result<T, MaterialFailure>;

#[cfg(test)]
pub(super) fn certify(
    requests: &[Request],
    prepared: &PreparedSelection,
    options: MaterialOptions,
) -> Checked<PeriodicMaterialEvidence> {
    certify_with_cancel(requests, prepared, options, || false)
}
pub(in super::super) fn certify_with_cancel(
    requests: &[Request],
    prepared: &PreparedSelection,
    options: MaterialOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<PeriodicMaterialEvidence> {
    certify_bound(
        requests,
        RelationInputs {
            prepared: &prepared.prepared,
            planar: &prepared.planar,
            boundary: &prepared.boundary_clearance,
        },
        options,
        &mut cancelled,
    )
    .map_err(|error| {
        debug_assert!(error.operations <= options.probes.max_operations);
        error.cause
    })
}

// Decoded cone cap polynomials already cover their whole parameter intervals.
#[cfg(test)]
pub(in super::super) fn certify_projected_with_cancel(
    requests: &[Request],
    planar: &PlanarSelectionEvidence,
    boundary: &planar_clearance::OriginalClearance,
    options: MaterialOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<PeriodicMaterialEvidence, MaterialWorkFailure> {
    certify_bound(
        requests,
        RelationInputs {
            prepared: requests,
            planar,
            boundary,
        },
        options,
        &mut cancelled,
    )
}

#[derive(Debug)]
pub(in super::super) struct MaterialWorkFailure {
    pub operations: u32,
    pub cause: MaterialFailure,
}
impl From<MaterialFailure> for MaterialWorkFailure {
    fn from(cause: MaterialFailure) -> Self {
        Self {
            operations: 0,
            cause,
        }
    }
}

struct RelationInputs<'a> {
    prepared: &'a [Request],
    planar: &'a PlanarSelectionEvidence,
    boundary: &'a planar_clearance::OriginalClearance,
}

fn certify_bound(
    requests: &[Request],
    inputs: RelationInputs<'_>,
    options: MaterialOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Result<PeriodicMaterialEvidence, MaterialWorkFailure> {
    if cancelled() {
        return Err(MaterialFailure::Cancelled.into());
    }
    admit(options)?;
    let regions = source::bind(requests, &inputs)?;
    let periodic = &inputs.boundary.periodic;
    let (jobs, inputs) = jobs::plan(&regions, periodic, options.probes.max_probes)?;
    let probes = periodic
        .certify_probes_with_work(&inputs, options.probes, &mut *cancelled)
        .map_err(|error| MaterialWorkFailure {
            operations: error.operations,
            cause: MaterialFailure::Probe(Box::new(error.cause)),
        })?;
    let mut work = winding::WindingWork::new(options, cancelled);
    let relations =
        classify(&regions, &jobs, &probes, &mut work).map_err(|cause| MaterialWorkFailure {
            operations: probes.operations(),
            cause,
        })?;
    Ok(material_evidence(probes, relations, work))
}
fn material_evidence(
    probes: CertifiedCylindricalProbes,
    relations: Vec<MaterialRelation>,
    work: winding::WindingWork<'_>,
) -> PeriodicMaterialEvidence {
    PeriodicMaterialEvidence {
        probes,
        relations,
        locations: work.locations,
        bisections: work.bisections,
        options: work.options,
    }
}

fn admit(options: MaterialOptions) -> Checked<()> {
    probe_policy(options)?;
    let r = options.winding;
    if r.version != 1
        || !r.max_width.is_finite()
        || r.max_width <= 0.
        || r.max_degree < 3
        || r.max_degree > 32
        || options.max_locations > 65_536
    {
        return Err(MaterialFailure::Policy(options));
    }
    Ok(())
}
fn probe_policy(options: MaterialOptions) -> Checked<()> {
    let p = options.probes;
    if p.behavior_version != 1
        || !(8..=4096).contains(&p.fractional_bits)
        || p.max_operations > 1_000_000
        || p.max_probes > 65_536
    {
        return Err(MaterialFailure::Policy(options));
    }
    Ok(())
}

fn classify(
    regions: &[&Request],
    jobs: &[Job],
    probes: &CertifiedCylindricalProbes,
    work: &mut winding::WindingWork<'_>,
) -> Checked<Vec<MaterialRelation>> {
    jobs.iter()
        .zip(probes.probes())
        .enumerate()
        .map(|(index, (job, probe))| classify_one(regions, job, probe, index, work))
        .collect()
}
fn classify_one(
    regions: &[&Request],
    job: &Job,
    probe: &martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalProbe,
    index: usize,
    work: &mut winding::WindingWork<'_>,
) -> Checked<MaterialRelation> {
    let target = target(regions, job.target)?;
    let reason =
        locate(target, probe.point(), index, work)?.ok_or_else(|| overlap(job, probe.request()))?;
    Ok(MaterialRelation {
        source: job.source,
        target: job.target,
        witness: job.witness,
        probe: index,
        reason,
    })
}
fn target<'a>(regions: &[&'a Request], id: SurfaceRegionId) -> Checked<&'a Request> {
    let index = regions
        .binary_search_by_key(&id, |region| region.region)
        .map_err(|_| MaterialFailure::Source(id))?;
    Ok(regions[index])
}
fn overlap(job: &Job, probe: CylindricalPeriodicProbe) -> MaterialFailure {
    MaterialFailure::Overlap {
        source: job.source,
        target: job.target,
        witness: job.witness,
        probe,
    }
}

fn locate(
    region: &Request,
    point: [f64; 2],
    probe: usize,
    work: &mut winding::WindingWork<'_>,
) -> Checked<Option<OutsideReason>> {
    let outer = region
        .loops
        .first()
        .ok_or(MaterialFailure::Source(region.region))?;
    if work.locate(region.region, outer, point, probe)?.1 == 0 {
        return Ok(Some(OutsideReason::OutsideOuter));
    }
    for hole in region.loops.iter().skip(1) {
        let (id, winding) = work.locate(region.region, hole, point, probe)?;
        if winding != 0 {
            return Ok(Some(OutsideReason::InsideHole(id)));
        }
    }
    Ok(None)
}
