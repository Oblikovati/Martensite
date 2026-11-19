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
    let (prepared, captured) = prepare_observed(request, progress)?;
    finish_owned(&prepared, captured, || false, completed)
}
pub(super) fn run_with_cancel(
    request: OwnedProfileRequest,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<GraphRegionCertificate, super::certify::ProfileProofRefusal> {
    let (prepared, captured) =
        prepare_observed(request, &mut super::certify::PollProgress(cancelled))?;
    finish_owned(&prepared, captured, cancelled, |_| {})
}
fn prepare_observed(
    request: OwnedProfileRequest,
    progress: &mut dyn OwnedPrismProgress,
) -> Result<(PreparedProfile, usize), super::certify::ProfileProofRefusal> {
    let owner = FreshGraphRequest {
        request,
        #[cfg(test)]
        captures: std::cell::Cell::new(0),
    };
    let (outcome, stop, phase, work) =
        super::pipeline::run_preparation(&owner.request, progress, Some(&owner));
    let prepared = super::pipeline::finished(outcome, stop, phase, work, owner.request)
        .map_err(super::certify::preparation_refusal)?;
    #[cfg(test)]
    let captured = owner.captures.get();
    #[cfg(not(test))]
    let captured = 0;
    Ok((prepared, captured))
}
fn finish_owned(
    prepared: &PreparedProfile,
    captured: usize,
    cancelled: impl FnMut() -> bool,
    completed: impl FnOnce(usize),
) -> Result<GraphRegionCertificate, super::certify::ProfileProofRefusal> {
    #[cfg(test)]
    let captures = std::cell::Cell::new(captured);
    #[cfg(not(test))]
    let _ = captured;
    let access = OwnedGraphAccess::fresh(
        &prepared.request,
        &prepared.edges,
        #[cfg(test)]
        &captures,
    );
    let result = inspect_owned(prepared, cancelled, &access);
    #[cfg(test)]
    completed(captures.get());
    #[cfg(not(test))]
    completed(0);
    result
}

fn inspect_owned(
    prepared: &PreparedProfile,
    cancelled: impl FnMut() -> bool,
    access: &OwnedGraphAccess<'_>,
) -> Result<GraphRegionCertificate, super::certify::ProfileProofRefusal> {
    let Some(options) = prepared.request.integral_program else {
        return super::regions::inspect_graph(
            &prepared.request,
            &prepared.edges,
            &prepared.bridge,
            cancelled,
            SourceAccess::Owned(access),
        )
        .map_err(|failure| super::certify::inspection_refusal(prepared, failure));
    };
    super::regions::inspect_program_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        cancelled,
        options,
        SourceAccess::Owned(access),
    )
    .map_err(|failure| super::certify::program_inspection_refusal(prepared, failure))
}

#[cfg(test)]
mod tests;
