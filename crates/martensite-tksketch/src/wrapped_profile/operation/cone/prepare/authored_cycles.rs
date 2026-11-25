//! Prove the directed authored family before fitting the attached tool.
use super::*;
use martensite_tkbo::{
    body_check::{CertifiedDirectedPlanarFamily, DirectedCapPolicy, DirectedPlanarFace},
    profile_extrusion::{
        LoopUse, PlanarProofVersion, PlanarProofWorkLimit, ProfileCurve, ProfileCurveId,
        ProfileLoop, SketchProfile,
    },
};
use martensite_tkmath::predicates::roots::RootIsolationOptions;

pub(in super::super) struct Authority {
    family: CertifiedDirectedPlanarFamily,
    regions: Vec<SurfaceRegionId>,
}

impl Authority {
    pub(in super::super) fn rebind(
        &self,
        request: &ConeRun<'_>,
        originals: &[DevelopedRegionRequest],
    ) -> ConeChecked<()> {
        let (profile, faces) = capture(originals)?;
        if !self
            .regions
            .iter()
            .copied()
            .eq(originals.iter().map(|source| source.region))
            || !self.family.matches(
                &profile,
                &faces,
                pipeline::resolution(request.options.spatial_error),
                self.family.work().requested,
            )
        {
            return Err(mismatch("directed authored source and selected material"));
        }
        Ok(())
    }

    pub(in super::super) fn work(&self) -> martensite_tkbo::body_check::DirectedCapWork {
        self.family.work()
    }
}

pub(in super::super) fn certify(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    selection: WrappedConeExecutionSelection,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Authority> {
    let first = originals
        .first()
        .ok_or_else(|| mismatch("authored family loan owner"))?;
    let policy = policy(request, selection, work)?;
    let (profile, faces) = capture(originals)?;
    let resolution = pipeline::resolution(request.options.spatial_error);
    let issued =
        CertifiedDirectedPlanarFamily::issue(profile, faces, resolution, policy, cancelled);
    let observed = issued
        .as_ref()
        .map_or_else(|refusal| refusal.observed, |proof| Some(proof.work()));
    let charged = observed
        .map(|proof| {
            let pair = work.charge_planar(first.region, proof.pair);
            let area = work.charge_integration(
                first.region,
                proof.integration_cells,
                request.proof.integration,
            );
            pair.and(area)
        })
        .transpose();
    let family =
        issued.map_err(|cause| failure(None, None, ConeCause::DirectedCap(Box::new(cause))))?;
    charged?;
    Ok(Authority {
        family,
        regions: originals.iter().map(|source| source.region).collect(),
    })
}

fn policy(
    request: &ConeRun<'_>,
    selection: WrappedConeExecutionSelection,
    work: &Work,
) -> ConeChecked<DirectedCapPolicy> {
    let Some(PlanarProofVersion::GeometricCompositeV6 {
        max_components,
        max_total_subintervals,
    }) = selection.authored_planar_program()
    else {
        return Err(mismatch("bounded directed authored policy"));
    };
    let pair_bisections = match work.planar_remaining(request.options.planar_work) {
        PlanarProofWorkLimit::PairBisections(limit) => limit,
        PlanarProofWorkLimit::CurrentDefault => RootIsolationOptions::new(0.001).max_bisections,
    };
    Ok(DirectedCapPolicy {
        pair_bisections,
        max_components,
        requested_integration_cells: max_total_subintervals,
        remaining_integration_cells: u64::from(
            work.integration(request.proof.integration)
                .max_total_subintervals,
        ),
    })
}

fn capture(
    originals: &[DevelopedRegionRequest],
) -> ConeChecked<(SketchProfile, Vec<DirectedPlanarFace>)> {
    let mut profile = planar::combined_profile(originals)
        .map_err(|cause| failure(None, None, ConeCause::Planar(Box::new(cause))))?;
    literal_lines(&mut profile)?;
    let faces = originals.iter().map(face).collect::<ConeChecked<_>>()?;
    Ok((profile, faces))
}

fn literal_lines(profile: &mut SketchProfile) -> ConeChecked<()> {
    let by_id: BTreeMap<_, _> = profile
        .vertices
        .iter()
        .map(|vertex| (vertex.id, vertex.point))
        .collect();
    for curve in &mut profile.curves {
        if let ProfileCurve::Line { id, start, end } = *curve {
            let a = by_id
                .get(&start)
                .ok_or_else(|| mismatch("authored line start"))?;
            let b = by_id
                .get(&end)
                .ok_or_else(|| mismatch("authored line end"))?;
            *curve = ProfileCurve::Spline {
                id,
                start,
                end,
                degree: 1,
                poles: vec![*a, *b],
                knots: vec![0., 0., 1., 1.],
                weights: None,
            };
        }
    }
    Ok(())
}

fn face(original: &DevelopedRegionRequest) -> ConeChecked<DirectedPlanarFace> {
    let mut loops = original.loops.iter().map(|wire| ProfileLoop {
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
        .ok_or_else(|| mismatch("authored outer cycle"))?;
    Ok(DirectedPlanarFace {
        outer,
        holes: loops.collect(),
    })
}

fn mismatch(field: &'static str) -> ConeFailure {
    failure(
        None,
        None,
        ConeCause::Binding {
            field,
            actual: vec![],
            expected: vec![1],
        },
    )
}
