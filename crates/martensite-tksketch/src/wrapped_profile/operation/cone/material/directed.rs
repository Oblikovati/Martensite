//! Bind and replay the opt-in exact directed cap authority from the actual projected sources.
use super::*;
use martensite_tkbo::body_check::{CertifiedPlanarMaterial, DirectedCapPolicy, DirectedCapWork};
use martensite_tkbo::profile_extrusion::{
    LoopUse, PlanarProofWorkLimit, ProfileCurveId, ProfileLoop, RegionCertificate, SketchProfile,
};

pub(super) fn certify(
    request: &ConeRun<'_>,
    uv: &DevelopedRegionRequest,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<CapturedUvMaterial> {
    let region = uv.region;
    let (profile, outer, holes) = capture(uv)?;
    let policy = policy(request, uv, work)?;
    let result = CertifiedPlanarMaterial::from_directed_cycles_v1(
        profile,
        outer,
        holes,
        pipeline::resolution(uv.tolerance),
        policy,
        cancelled,
    );
    let observed = result
        .as_ref()
        .map_or_else(|failure| failure.observed, |(_, actual)| Some(*actual));
    let charged = observed
        .map(|actual| charge(request, region, actual, work))
        .transpose();
    let (selected, actual) = result
        .map_err(|cause| storage::failure(region, None, ConeCause::DirectedCap(Box::new(cause))))?;
    charged?;
    Ok(CapturedUvMaterial {
        uv: UvMaterial::Directed(actual),
        selected: Some(selected),
        census: RegionCertificate {
            pair_domains: actual.pair_domains,
            carrier_pairs: actual.carrier_pairs,
            loops: 1,
            components: actual.loops,
        },
        work: actual.pair,
        version: version(actual),
    })
}

fn charge(
    request: &ConeRun<'_>,
    region: SurfaceRegionId,
    actual: DirectedCapWork,
    work: &mut storage::Work,
) -> ConeChecked<()> {
    let pair = work.charge_planar(region, actual.pair);
    let integration =
        work.charge_integration(region, actual.integration_cells, request.proof.integration);
    pair.and(integration)
}

fn policy(
    request: &ConeRun<'_>,
    uv: &DevelopedRegionRequest,
    work: &storage::Work,
) -> ConeChecked<DirectedCapPolicy> {
    let (max_components, requested_integration_cells) =
        caps(uv.region, request.proof.planar_proof)?;
    let pair_bisections = match work.planar_remaining(request.options.planar_work) {
        PlanarProofWorkLimit::PairBisections(limit) => limit,
        PlanarProofWorkLimit::CurrentDefault => {
            martensite_tkmath::predicates::roots::RootIsolationOptions::new(0.001).max_bisections
        }
    };
    let remaining_integration_cells = u64::from(
        work.integration(request.proof.integration)
            .max_total_subintervals,
    );
    Ok(DirectedCapPolicy {
        pair_bisections,
        max_components,
        requested_integration_cells,
        remaining_integration_cells,
    })
}
fn caps(region: SurfaceRegionId, version: PlanarProofVersion) -> ConeChecked<(u16, u64)> {
    let PlanarProofVersion::GeometricCompositeV6 {
        max_components,
        max_total_subintervals,
    } = version
    else {
        return Err(mismatch(
            region,
            None,
            "directed cap requires GeometricCompositeV6 limits",
        ));
    };
    Ok((max_components, max_total_subintervals))
}
fn version(work: DirectedCapWork) -> PlanarProofVersion {
    PlanarProofVersion::DirectedSelectedCapV1 {
        max_components: work.requested.max_components,
        max_total_subintervals: work.effective_integration_cells,
    }
}

fn capture(
    uv: &DevelopedRegionRequest,
) -> ConeChecked<(SketchProfile, ProfileLoop, Vec<ProfileLoop>)> {
    let profile = planar::combined_profile(std::slice::from_ref(uv))
        .map_err(|cause| planar_failure(uv.region, cause))?;
    let mut loops = uv.loops.iter().map(|wire| ProfileLoop {
        uses: wire
            .iter()
            .map(|piece| LoopUse {
                curve: ProfileCurveId(piece.span.curve.0),
                reversed: false,
            })
            .collect(),
    });
    let outer = loops
        .next()
        .ok_or_else(|| mismatch(uv.region, None, "directed cap requires an outer cycle"))?;
    Ok((profile, outer, loops.collect()))
}

pub(super) fn validate(
    uv: &DevelopedRegionRequest,
    material: &MaterialEvidence,
    work: DirectedCapWork,
) -> ConeChecked<()> {
    let requested = caps(uv.region, material.requested_planar_proof)?;
    if requested
        != (
            work.requested.max_components,
            work.requested.requested_integration_cells,
        )
        || work.effective_integration_cells
            != requested.1.min(work.requested.remaining_integration_cells)
        || material.planar_proof != version(work)
        || material.planar_work != work.pair
    {
        return Err(mismatch(uv.region, None, "retained directed cap policy"));
    }
    let (profile, outer, holes) = capture(uv)?;
    if !material
        .selected_material
        .as_ref()
        .is_some_and(|proof| proof.matches_directed_cycles(&profile, &outer, &holes))
    {
        return Err(mismatch(
            uv.region,
            None,
            "retained directed cap source and material roles",
        ));
    }
    Ok(())
}
