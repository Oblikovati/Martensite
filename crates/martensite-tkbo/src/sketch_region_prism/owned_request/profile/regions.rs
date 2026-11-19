// SPDX-License-Identifier: GPL-2.0-only
//! Run the general loop region proof on a prepared profile's own transaction.
use super::*;
use crate::sketch_region_prism::region_certificate::loop_region::graph::{
    GraphRegionCertificate, GraphRegionRequest, inspect_plane_graph,
};
use crate::sketch_region_prism::region_certificate::loop_region::{
    AuthoredLoop, AuthoredLoopUse, AuthoredRegion, LoopRegionCertificate, LoopRegionCheck,
    LoopRegionRequest, inspect_loop_regions,
};

impl PreparedProfile {
    /// Certify the authored loops and selected regions; `prepared.inspect_regions(|| false)`.
    pub(in crate::sketch_region_prism) fn inspect_regions(
        &self,
        cancelled: impl FnMut() -> bool,
    ) -> LoopRegionCheck<LoopRegionCertificate> {
        let loops: Vec<AuthoredLoop> = self.request.loops.iter().map(authored_loop).collect();
        let regions: Vec<AuthoredRegion> = self
            .request
            .regions
            .iter()
            .map(|region| AuthoredRegion {
                name: region.name,
                seed: region.seed,
            })
            .collect();
        let sources = self.sources();
        let request = LoopRegionRequest {
            bridge: &self.bridge,
            sources: &sources,
            loops: &loops,
            regions: &regions,
            options: self.request.policy.region,
        };
        inspect_loop_regions(&request, cancelled)
    }
}
impl PreparedProfile {
    /// Certify every bounded face of a version-2 plane graph;
    /// `prepared.inspect_plane_graph(|| false)`.
    pub(in crate::sketch_region_prism) fn inspect_plane_graph(
        &self,
        cancelled: impl FnMut() -> bool,
    ) -> LoopRegionCheck<GraphRegionCertificate> {
        inspect_graph(
            &self.request,
            &self.edges,
            &self.bridge,
            cancelled,
            crate::sketch_region_certification::SourceAccess::Live,
        )
    }
}
pub(super) fn inspect_graph(
    owned: &OwnedProfileRequest,
    edges: &[ConstructedProfileEdge],
    bridge: &PairedContactTransaction,
    cancelled: impl FnMut() -> bool,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> LoopRegionCheck<GraphRegionCertificate> {
    bind_integral_selection(owned, None)?;
    super::selection::bind(owned.version, &bridge.census)?;
    with_graph_request(owned, edges, bridge, |request| {
        match access {
        crate::sketch_region_certification::SourceAccess::Live => inspect_plane_graph(request, cancelled),
        crate::sketch_region_certification::SourceAccess::Owned(_) => crate::sketch_region_prism::region_certificate::loop_region::graph::inspect_plane_graph_with_access(request, cancelled, access),
    }
    })
}
pub(super) fn inspect_program_graph(
    owned: &OwnedProfileRequest,
    edges: &[ConstructedProfileEdge],
    bridge: &PairedContactTransaction,
    cancelled: impl FnMut() -> bool,
    options: martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> Result<GraphRegionCertificate, Box<crate::sketch_region_prism::ProgramGraphFailure>> {
    let failure = |failure| program_refusal(failure, options);
    bind_integral_selection(owned, Some(options)).map_err(failure)?;
    super::selection::bind(owned.version, &bridge.census).map_err(failure)?;
    with_graph_request(owned, edges, bridge, |request| {
        match access {
        crate::sketch_region_certification::SourceAccess::Live => crate::sketch_region_prism::region_certificate::loop_region::graph::inspect_plane_graph_program(request, cancelled, options),
        crate::sketch_region_certification::SourceAccess::Owned(_) => crate::sketch_region_prism::region_certificate::loop_region::graph::inspect_plane_graph_program_with_access(request, cancelled, options, access),
    }
    })
}
fn with_graph_request<T, E>(
    owned: &OwnedProfileRequest,
    edges: &[ConstructedProfileEdge],
    bridge: &PairedContactTransaction,
    inspect: impl FnOnce(&GraphRegionRequest<'_, '_>) -> Result<T, E>,
) -> Result<T, E> {
    let ends = owned
        .edges
        .iter()
        .map(|edge| [edge.start.0, edge.end.0])
        .collect::<Vec<_>>();
    let sources = edges
        .iter()
        .map(ConstructedProfileEdge::source)
        .collect::<Vec<_>>();
    let request = GraphRegionRequest {
        bridge,
        sources: &sources,
        ends: &ends,
        options: owned.policy.region,
    };
    inspect(&request)
}

fn authored_loop(owned: &OwnedProfileLoop) -> AuthoredLoop {
    let uses = owned.uses.iter().map(|directed| AuthoredLoopUse {
        source: RegionSourceId(directed.edge.0),
        reversed: directed.reversed,
    });
    AuthoredLoop {
        name: owned.name,
        uses: uses.collect(),
    }
}

fn bind_integral_selection(
    request: &OwnedProfileRequest,
    expected: Option<martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions>,
) -> crate::sketch_region_prism::region_certificate::loop_region::LoopRegionCheck<()> {
    use crate::sketch_region_prism::region_certificate::loop_region::*;
    let program_version = matches!(
        request.version,
        super::admission::COMPOSITE_GRAPH_PROFILE_VERSION
            | super::admission::GEOMETRIC_GRAPH_PROFILE_VERSION
    );
    if request.integral_program == expected && program_version == expected.is_some() {
        return Ok(());
    }
    Err(LoopRegionFailure {
        stage: LoopRegionStage::Admission,
        cause: LoopRegionCause::IntegralSelection(Box::new(IntegralSelectionMismatch {
            graph_version: request.version,
            retained: request.integral_program,
            expected,
        })),
    })
}

fn program_refusal(
    failure: crate::sketch_region_prism::region_certificate::loop_region::LoopRegionFailure,
    options: martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions,
) -> Box<crate::sketch_region_prism::ProgramGraphFailure> {
    Box::new(crate::sketch_region_prism::ProgramGraphFailure {
        failure,
        options,
        consumed: 0,
    })
}
