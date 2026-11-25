// SPDX-License-Identifier: GPL-2.0-only
//! #544: graph selectors bind one shared census to its actual generated-sign mode.
use super::admission::{
    COMPOSITE_GRAPH_PROFILE_VERSION, EXCLUSION_GRAPH_PROFILE_VERSION,
    FILTERED_GRAPH_PROFILE_VERSION, GEOMETRIC_GRAPH_PROFILE_VERSION,
};
use crate::sketch_region_certification::{
    BoundaryCensus, BoundaryInputs, CensusCause, CensusPolicy, bind_graph_policy,
    enumerate_carrier_census, enumerate_exclusion_census, enumerate_filtered_exclusion_census,
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
    live_census(version, inputs, (policy, source), cancelled)
}
pub(super) fn census_with_access(
    version: u32,
    inputs: BoundaryInputs,
    (policy, source): (CensusPolicy, SourceConformancePolicy),
    cancelled: impl FnMut() -> bool,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> Result<BoundaryCensus, CensusCause> {
    match access {
        crate::sketch_region_certification::SourceAccess::Live => {
            census(version, inputs, (policy, source), cancelled)
        }
        crate::sketch_region_certification::SourceAccess::Owned(_) => {
            owned_census(version, inputs, (policy, source), cancelled, access)
        }
    }
}
fn live_census(
    version: u32,
    inputs: BoundaryInputs,
    (policy, source): (CensusPolicy, SourceConformancePolicy),
    cancelled: impl FnMut() -> bool,
) -> Result<BoundaryCensus, CensusCause> {
    match version {
        GEOMETRIC_GRAPH_PROFILE_VERSION => {
            crate::sketch_region_certification::enumerate_geometric_census_with_access(
                inputs,
                policy,
                source,
                cancelled,
                crate::sketch_region_certification::SourceAccess::Live,
            )
        }
        EXCLUSION_GRAPH_PROFILE_VERSION => {
            enumerate_exclusion_census(inputs, policy, source, cancelled)
        }
        FILTERED_GRAPH_PROFILE_VERSION | COMPOSITE_GRAPH_PROFILE_VERSION => {
            enumerate_filtered_exclusion_census(inputs, policy, source, cancelled)
        }
        _ => enumerate_carrier_census(inputs, policy, source, cancelled),
    }
}
fn owned_census(
    version: u32,
    inputs: BoundaryInputs,
    (policy, source): (CensusPolicy, SourceConformancePolicy),
    cancelled: impl FnMut() -> bool,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> Result<BoundaryCensus, CensusCause> {
    use crate::sketch_region_certification::*;
    match version {
        GEOMETRIC_GRAPH_PROFILE_VERSION => {
            enumerate_geometric_census_with_access(inputs, policy, source, cancelled, access)
        }
        EXCLUSION_GRAPH_PROFILE_VERSION => {
            enumerate_exclusion_census_with_access(inputs, policy, source, cancelled, access)
        }
        FILTERED_GRAPH_PROFILE_VERSION | COMPOSITE_GRAPH_PROFILE_VERSION => {
            enumerate_filtered_exclusion_census_with_access(
                inputs, policy, source, cancelled, access,
            )
        }
        _ => enumerate_carrier_census_with_access(inputs, policy, source, cancelled, access),
    }
}
pub(super) fn bind(version: u32, census: &BoundaryCensus) -> LoopRegionCheck<()> {
    bind_graph_policy(census, version).map_err(|mismatch| LoopRegionFailure {
        stage: LoopRegionStage::Admission,
        cause: LoopRegionCause::GraphPolicy(mismatch),
    })
}
