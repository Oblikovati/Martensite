// SPDX-License-Identifier: GPL-2.0-only
//! Crate-level entry to the plane graph region proof for the public profile extrusion seam: owned plain inputs in, the
//! certificate or a rendered typed refusal out. It adds no proof step of its own.
use super::admission::GRAPH_PROFILE_VERSION;
use super::*;
use crate::sketch_region_prism::region_certificate::loop_region::graph::GraphRegionCertificate;
use crate::sketch_region_prism::region_certificate::loop_region::{
    LoopRegionCause, LoopRegionCertificate, LoopRegionFailure, LoopRegionOptions,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::roots::RootIsolationOptions;
use martensite_tkmath::predicates::roots::metric_bounds::RationalIntegralOptions;

/// One Bézier edge of the proof request in world coordinates.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProofEdge {
    pub id: u64,
    pub world_poles: Vec<[f64; 3]>,
    pub weights: Option<Vec<f64>>,
    pub ends: [u64; 2],
}

/// A refusal of the proof, typed where the public seam distinguishes it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ProfileProofCause {
    /// Two edges meet away from an authored join.
    UnexpectedContact { edges: [u64; 2], point: [f64; 2] },
    /// Branches leave `vertex` in one direction, so their order is undecided.
    TangentBranches { vertex: u64 },
    /// A face or hole boundary revisits `vertex`.
    Pinched { vertex: u64, hole: bool },
    /// Any other refused or incomplete obligation, rendered with its stage.
    Incomplete(String),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProfileProofRefusal {
    pub cause: ProfileProofCause,
    pub root_failure: Option<(
        [u64; 2],
        martensite_tkmath::predicates::roots::RootIsolationError,
    )>,
    pub work: Option<crate::sketch_region_certification::CensusWorkReceipt>,
    pub cancelled: bool,
}

/// Plane graph inputs: edges with their vertex ids, no loops or seeds.
pub(crate) struct ProfileGraphInput {
    pub edges: Vec<ProofEdge>,
    pub plane: Plane3,
    /// Spatial limit for census contacts and transported endpoints.
    pub spatial_limit: f64,
    /// Explicit shared pair work; None retains the existing child-policy default.
    pub pair_bisections: Option<u16>,
}

/// Prepare and certify a plane graph; `certify_profile_graph(input)?` returns every
/// bounded face with its holes and certified area.
pub(crate) fn certify_profile_graph(
    input: ProfileGraphInput,
) -> Result<GraphRegionCertificate, ProfileProofRefusal> {
    super::owned_graph::run(owned_graph_request(input), &mut NeverStop)
}

struct NeverStop;
impl OwnedPrismProgress for NeverStop {
    fn checkpoint(&mut self, _: OwnedPrismCheckpoint) -> Result<bool, Failure> {
        Ok(false)
    }
}
pub(super) struct PollProgress<'a, F>(pub(super) &'a mut F);
impl<F: FnMut() -> bool + Send> OwnedPrismProgress for PollProgress<'_, F> {
    fn checkpoint(&mut self, _: OwnedPrismCheckpoint) -> Result<bool, Failure> {
        Ok((self.0)())
    }
}

pub(crate) fn certify_profile_graph_with_cancel(
    input: ProfileGraphInput,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<GraphRegionCertificate, ProfileProofRefusal> {
    certify_profile_graph_version_with_cancel(input, GRAPH_PROFILE_VERSION, cancelled)
}
/// Retain the explicitly selected graph behavior in the owned request.
pub(crate) fn certify_profile_graph_version_with_cancel(
    input: ProfileGraphInput,
    version: u32,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<GraphRegionCertificate, ProfileProofRefusal> {
    let mut request = owned_graph_request(input);
    request.version = version;
    super::owned_graph::run_with_cancel(request, &mut cancelled)
}
pub(super) fn inspection_refusal(
    prepared: &PreparedProfile,
    failure: LoopRegionFailure,
) -> ProfileProofRefusal {
    let census = &prepared.bridge.census;
    ProfileProofRefusal {
        root_failure: refused_pair_root(census, &failure.cause),
        cancelled: matches!(failure.cause, LoopRegionCause::Cancelled),
        cause: region_cause(failure),
        work: Some(crate::sketch_region_certification::CensusWorkReceipt::capture(census)),
    }
}
fn refused_pair_root(
    census: &crate::sketch_region_certification::BoundaryCensus,
    cause: &LoopRegionCause,
) -> Option<(
    [u64; 2],
    martensite_tkmath::predicates::roots::RootIsolationError,
)> {
    let LoopRegionCause::PairUnavailable { sources } = cause else {
        return None;
    };
    let sources = sources.map(|source| source.0);
    crate::sketch_region_certification::pair_root_failure(census, sources)
        .map(|cause| (sources, cause))
}
fn owned_graph_request(input: ProfileGraphInput) -> OwnedProfileRequest {
    OwnedProfileRequest {
        version: GRAPH_PROFILE_VERSION,
        operation: CallerOperationId(1),
        edges: input.edges.into_iter().map(owned_edge).collect(),
        loops: Vec::new(),
        regions: Vec::new(),
        declared_plane: input.plane,
        policy: policy(input.spatial_limit, input.pair_bisections),
    }
}

pub(super) fn preparation_refusal(failure: Box<ProfilePreparationFailure>) -> ProfileProofRefusal {
    let cancelled = failure.class == OwnedPreparationClass::Cancelled;
    let cause = match &failure.cause {
        ProfilePreparationCause::Admission(cause) => format!("{cause:?}"),
        ProfilePreparationCause::Stage(_) => "stage refusal".to_string(),
    };
    let (class, phase) = (failure.class, failure.phase);
    ProfileProofRefusal {
        cancelled,
        root_failure: None,
        cause: ProfileProofCause::Incomplete(format!(
            "profile preparation {class:?} at {phase:?}: {cause}"
        )),
        work: failure.pair_work,
    }
}

fn region_cause(failure: LoopRegionFailure) -> ProfileProofCause {
    match failure.cause {
        LoopRegionCause::UnexpectedContact { sources, point } => {
            let edges = sources.map(|source| source.0);
            ProfileProofCause::UnexpectedContact { edges, point }
        }
        LoopRegionCause::TangentBranches { vertex } => {
            ProfileProofCause::TangentBranches { vertex }
        }
        LoopRegionCause::PinchedFace { vertex } => ProfileProofCause::Pinched {
            vertex,
            hole: false,
        },
        LoopRegionCause::PinchedHole { vertex } => {
            ProfileProofCause::Pinched { vertex, hole: true }
        }
        cause => ProfileProofCause::Incomplete(format!("{:?}: {cause:?}", failure.stage)),
    }
}

fn owned_edge(edge: ProofEdge) -> OwnedProfileEdge {
    OwnedProfileEdge {
        source: CallerBoundaryId(edge.id),
        revision: CallerSourceRevision(1),
        world_poles: edge.world_poles,
        world_weights: edge.weights,
        start: CallerVertexId(edge.ends[0]),
        end: CallerVertexId(edge.ends[1]),
    }
}

/// The version-1 child policies with the caller's spatial limit.
fn policy(limit: f64, pair_bisections: Option<u16>) -> OwnedProfilePolicy {
    let roots = RootIsolationOptions::new(0.001);
    OwnedProfilePolicy {
        census: pair_census_policy(limit, roots, pair_bisections),
        source: exact_source_policy(roots),
        contacts: contact_policy(limit, roots),
        region: graph_region_policy(roots),
    }
}
fn pair_census_policy(
    limit: f64,
    mut roots: RootIsolationOptions,
    requested: Option<u16>,
) -> CensusPolicy {
    if let Some(maximum) = requested {
        roots.max_bisections = maximum;
    }
    CensusPolicy {
        roots,
        max_world_error: limit,
    }
}
fn exact_source_policy(roots: RootIsolationOptions) -> SourceConformancePolicy {
    SourceConformancePolicy {
        roots,
        original_plane_limit: 0.,
        construction_limit: 0.,
    }
}
fn graph_region_policy(roots: RootIsolationOptions) -> LoopRegionOptions {
    LoopRegionOptions {
        version: 1,
        winding: roots,
        area: RationalIntegralOptions::default(),
    }
}

fn contact_policy(limit: f64, roots: RootIsolationOptions) -> ContactTransportPolicy {
    ContactTransportPolicy {
        native_parameter_limits: [limit; 2],
        world_endpoint_limit: limit,
        roots,
    }
}

#[cfg(test)]
#[path = "certify_work_tests.rs"]
mod work_tests;

#[cfg(test)]
#[path = "filtered_work_tests.rs"]
mod filtered_work_tests;
