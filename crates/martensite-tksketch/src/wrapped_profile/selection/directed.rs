// SPDX-License-Identifier: GPL-2.0-only
//! Use the caller's directed material instead of enumerating every bounded planar cell.
use super::*;
use martensite_tkbo::body_check::{CertifiedDirectedPlanarFamily, DirectedCapPolicy};
use martensite_tkmath::predicates::roots::RootIsolationOptions;

pub(in crate::surface_sketch::wrapped_profile) struct AttachmentSelection {
    pub(in crate::surface_sketch::wrapped_profile) prepared: Vec<Request>,
    pub(in crate::surface_sketch::wrapped_profile) boundary_clearance:
        planar_clearance::OriginalClearance,
}

pub(super) struct DirectedSelection {
    pub(super) attachment: AttachmentSelection,
    family: CertifiedDirectedPlanarFamily,
    regions: Vec<SurfaceRegionId>,
}

pub(in crate::surface_sketch::wrapped_profile) fn prepare_attachment(
    requests: &[Request],
    work: PlanarProofWorkLimit,
    preparation: Preparation,
    cancelled: &mut CancelPoll<'_>,
) -> Result<AttachmentSelection, SelectionFailure> {
    if cancelled() {
        return Err(SelectionFailure::Cancelled);
    }
    let tolerance = admission::admit(requests)?;
    if may_nest_across_regions(requests) {
        // The arrangement also supports a selected island inside another region's hole.
        let selected =
            certify_with_proof(requests, Default::default(), work, preparation, cancelled)?;
        return Ok(AttachmentSelection {
            prepared: selected.prepared.prepared,
            boundary_clearance: selected.prepared.boundary_clearance,
        });
    }
    let family = prove(requests, tolerance, work, preparation.proof, cancelled)?;
    let prepared = prepare_requests(requests, preparation.maximum_fit_error, cancelled)?;
    let boundary_clearance = prepare_boundary(requests, &prepared, cancelled)?;
    let selected = DirectedSelection {
        attachment: AttachmentSelection {
            prepared,
            boundary_clearance,
        },
        family,
        regions: requests.iter().map(|source| source.region).collect(),
    };
    material::certify_directed(requests, &selected, Default::default(), cancelled)
        .map_err(|cause| SelectionFailure::Material(Box::new(cause)))?;
    Ok(selected.attachment)
}

fn may_nest_across_regions(requests: &[Request]) -> bool {
    let bounds: Vec<_> = requests.iter().map(outer_bounds).collect();
    requests.iter().enumerate().any(|(owner, request)| {
        request.loops.iter().any(|wire| {
            let witness = wire[0].poles[0];
            bounds.iter().enumerate().any(|(other, limits)| {
                owner != other
                    && (0..2).all(|axis| {
                        limits[axis][0] <= witness[axis] && witness[axis] <= limits[axis][1]
                    })
            })
        })
    })
}

fn outer_bounds(request: &Request) -> [[f64; 2]; 2] {
    let mut limits = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    for point in request.loops[0].iter().flat_map(|piece| &piece.poles) {
        for (bounds, &coordinate) in limits.iter_mut().zip(point) {
            bounds[0] = bounds[0].min(coordinate);
            bounds[1] = bounds[1].max(coordinate);
        }
    }
    limits
}

fn prove(
    requests: &[Request],
    tolerance: f64,
    work: PlanarProofWorkLimit,
    proof: PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
) -> Result<CertifiedDirectedPlanarFamily, SelectionFailure> {
    let policy = policy(work, proof)?;
    let (profile, faces) = capture(requests)?;
    CertifiedDirectedPlanarFamily::issue(
        profile,
        faces,
        pipeline::resolution(tolerance),
        policy,
        cancelled,
    )
    .map_err(|failure| {
        if failure.cancelled {
            SelectionFailure::Cancelled
        } else {
            SelectionFailure::Directed(Box::new(failure))
        }
    })
}

impl DirectedSelection {
    pub(super) fn rebind(
        &self,
        requests: &[Request],
        tolerance: f64,
    ) -> Result<(), SelectionFailure> {
        let (profile, faces) = capture(requests)?;
        let resolution = pipeline::resolution(tolerance);
        let requested = self.family.work().requested;
        let same_regions = self
            .regions
            .iter()
            .copied()
            .eq(requests.iter().map(|source| source.region));
        if same_regions && self.family.matches(&profile, &faces, resolution, requested) {
            return Ok(());
        }
        Err(SelectionFailure::DirectedSource)
    }
}

fn capture(
    requests: &[Request],
) -> Result<
    (
        martensite_tkbo::profile_extrusion::SketchProfile,
        Vec<martensite_tkbo::body_check::DirectedPlanarFace>,
    ),
    SelectionFailure,
> {
    planar::directed::capture(requests).map_err(|cause| SelectionFailure::Capture(Box::new(cause)))
}

fn policy(
    work: PlanarProofWorkLimit,
    proof: PlanarProofVersion,
) -> Result<DirectedCapPolicy, SelectionFailure> {
    let PlanarProofVersion::GeometricCompositeV6 {
        max_components,
        max_total_subintervals,
    } = proof
    else {
        return Err(SelectionFailure::DirectedPolicy(proof));
    };
    let pair_bisections = match work {
        PlanarProofWorkLimit::PairBisections(limit) => limit,
        PlanarProofWorkLimit::CurrentDefault => RootIsolationOptions::new(0.001).max_bisections,
    };
    Ok(DirectedCapPolicy {
        pair_bisections,
        max_components,
        requested_integration_cells: max_total_subintervals,
        remaining_integration_cells: max_total_subintervals,
    })
}
