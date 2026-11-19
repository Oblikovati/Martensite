// SPDX-License-Identifier: GPL-2.0-only
//! #555: fresh native handles never leave this raw scalar graph execution.
use super::*;
use crate::sketch_region_certification::{BoundaryCensus, SourceAccess};
use crate::sketch_region_geometry_capture::CapturedMetricCurve;
use crate::sketch_region_prism::region_certificate::loop_region::graph::GraphRegionCertificate;
use crate::sketch_region_source_conformance::CapturedSpaceRepresentation;
pub(super) struct FreshGraphRequest {
    request: OwnedProfileRequest,
    #[cfg(test)]
    captures: std::cell::Cell<usize>,
}
impl FreshGraphRequest {
    pub(super) fn access<'a>(
        &'a self,
        edges: &'a [ConstructedProfileEdge],
    ) -> OwnedGraphAccess<'a> {
        OwnedGraphAccess::fresh(
            &self.request,
            edges,
            #[cfg(test)]
            &self.captures,
        )
    }
}
pub(crate) struct OwnedGraphAccess<'a> {
    request: &'a OwnedProfileRequest,
    edges: &'a [ConstructedProfileEdge],
    #[cfg(test)]
    captures: &'a std::cell::Cell<usize>,
}
impl<'a> OwnedGraphAccess<'a> {
    fn fresh(
        request: &'a OwnedProfileRequest,
        edges: &'a [ConstructedProfileEdge],
        #[cfg(test)] captures: &'a std::cell::Cell<usize>,
    ) -> Self {
        Self {
            request,
            edges,
            #[cfg(test)]
            captures,
        }
    }
    #[cfg(test)]
    pub(crate) fn observed_capture(&self) {
        self.captures.set(self.captures.get() + 1);
    }
    pub(crate) fn binds(&self, census: &BoundaryCensus) -> bool {
        binding::matches(self, census)
    }
}
mod binding;

pub(super) fn run(
    request: OwnedProfileRequest,
    progress: &mut dyn OwnedPrismProgress,
) -> Result<GraphRegionCertificate, super::certify::ProfileProofRefusal> {
    run_observed(request, progress, |_| {})
}
fn run_observed(
    request: OwnedProfileRequest,
    progress: &mut dyn OwnedPrismProgress,
    completed: impl FnOnce(usize),
) -> Result<GraphRegionCertificate, super::certify::ProfileProofRefusal> {
    let owner = FreshGraphRequest {
        request,
        #[cfg(test)]
        captures: std::cell::Cell::new(0),
    };
    let (outcome, stop, phase) =
        super::pipeline::run_preparation(&owner.request, progress, Some(&owner));
    let prepared = super::pipeline::finished(outcome, stop, phase, owner.request)
        .map_err(super::certify::preparation_refusal)?;
    let result = inspect_prepared(
        &prepared,
        #[cfg(test)]
        &owner.captures,
    );
    #[cfg(test)]
    completed(owner.captures.get());
    #[cfg(not(test))]
    completed(0);
    result
}

fn inspect_prepared(
    prepared: &PreparedProfile,
    #[cfg(test)] captures: &std::cell::Cell<usize>,
) -> Result<GraphRegionCertificate, super::certify::ProfileProofRefusal> {
    let access = OwnedGraphAccess::fresh(
        &prepared.request,
        &prepared.edges,
        #[cfg(test)]
        captures,
    );
    super::regions::inspect_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        SourceAccess::Owned(&access),
    )
    .map_err(super::certify::region_refusal)
}

#[cfg(test)]
mod tests;
