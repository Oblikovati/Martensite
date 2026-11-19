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
    let ends: Vec<[u64; 2]> = owned
        .edges
        .iter()
        .map(|edge| [edge.start.0, edge.end.0])
        .collect();
    let sources: Vec<_> = edges.iter().map(ConstructedProfileEdge::source).collect();
    let request = GraphRegionRequest {
        bridge,
        sources: &sources,
        ends: &ends,
        options: owned.policy.region,
    };
    match access {
        crate::sketch_region_certification::SourceAccess::Live => inspect_plane_graph(&request, cancelled),
        crate::sketch_region_certification::SourceAccess::Owned(_) => crate::sketch_region_prism::region_certificate::loop_region::graph::inspect_plane_graph_with_access(&request, cancelled, access),
    }
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
