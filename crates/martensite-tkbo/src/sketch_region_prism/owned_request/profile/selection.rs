// SPDX-License-Identifier: GPL-2.0-only
//! #544: graph selectors bind one shared census to its actual generated-sign mode.
use super::admission::{EXCLUSION_GRAPH_PROFILE_VERSION, FILTERED_GRAPH_PROFILE_VERSION};
use crate::sketch_region_certification::{
    BoundaryCensus, BoundaryInputs, CensusCause, CensusPolicy, enumerate_carrier_census,
    enumerate_exclusion_census, enumerate_filtered_exclusion_census, graph_policy_current,
};
use crate::sketch_region_prism::region_certificate::loop_region::{
    LoopRegionCause, LoopRegionCheck, LoopRegionFailure, LoopRegionStage,
};
use crate::sketch_region_source_conformance::SourceConformancePolicy;
pub(super) fn census(
    version: u32,
    inputs: BoundaryInputs,
    (policy, source): (CensusPolicy, SourceConformancePolicy),
    cancelled: impl FnMut() -> bool,
) -> Result<BoundaryCensus, CensusCause> {
    match version {
        EXCLUSION_GRAPH_PROFILE_VERSION => {
            enumerate_exclusion_census(inputs, policy, source, cancelled)
        }
        FILTERED_GRAPH_PROFILE_VERSION => {
            enumerate_filtered_exclusion_census(inputs, policy, source, cancelled)
        }
        _ => enumerate_carrier_census(inputs, policy, source, cancelled),
    }
}
pub(super) fn bind(version: u32, census: &BoundaryCensus) -> LoopRegionCheck<()> {
    if graph_policy_current(census, version) {
        return Ok(());
    }
    Err(LoopRegionFailure {
        stage: LoopRegionStage::Admission,
        cause: LoopRegionCause::Obligation {
            quantity: "owned graph selector and actual census/sign policy binding",
        },
    })
}
