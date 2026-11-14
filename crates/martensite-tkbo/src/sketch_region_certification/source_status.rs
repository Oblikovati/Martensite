// SPDX-License-Identifier: GPL-2.0-only
use super::BoundaryCensus;
use crate::sketch_region_source_conformance::*;
use martensite_tkmath::predicates::roots::{
    RootIsolationError, metric_bounds::ResidualError, signs::RootSignError,
};
pub(super) fn representation_matches(census: &BoundaryCensus) -> bool {
    let Some(binding) = &census.source_binding else {
        return false;
    };
    let Some(policy) = census.source_policy else {
        return false;
    };
    binding.representation_certified()
        && source_binding_faults(&census.inputs, policy, binding.inspection()).is_empty()
}
pub(super) fn source_cancelled(report: &SourceConformanceInspection) -> bool {
    report
        .entries
        .iter()
        .flat_map(|row| &row.stages)
        .any(|stage| {
            matches!(
                &stage.decision,
                Err(SourceConformanceCause::Exact(ResidualError::Roots(
                    RootIsolationError::Cancelled
                ))) | Err(SourceConformanceCause::Exact(ResidualError::Signs(
                    RootSignError::Root {
                        cause: RootIsolationError::Cancelled,
                        ..
                    }
                )))
            )
        })
}

/// Scalar matrices are immutable under this borrow; every native representation stays live.
pub(super) struct CurrentSourceRepresentation<'a> {
    census: &'a BoundaryCensus,
    constructions: Vec<&'a CapturedBoundaryConstruction>,
}
impl<'a> CurrentSourceRepresentation<'a> {
    pub(super) fn new(census: &'a BoundaryCensus) -> Option<Self> {
        if !representation_structure_matches(census) {
            return None;
        }
        let constructions = source_constructions(census)?;
        Some(Self {
            census,
            constructions,
        })
    }
    pub(super) fn current(&self) -> bool {
        self.census
            .inputs
            .segments
            .iter()
            .zip(&self.constructions)
            .all(|(segment, construction)| {
                current_construction_correspondence(&self.census.inputs, construction, segment)
                    .is_ok()
            })
    }
}

fn source_constructions(census: &BoundaryCensus) -> Option<Vec<&CapturedBoundaryConstruction>> {
    census
        .inputs
        .segments
        .iter()
        .map(|segment| {
            census
                .inputs
                .constructions
                .iter()
                .find(|row| row.occurrence == segment.occurrence)
        })
        .collect()
}

fn representation_structure_matches(census: &BoundaryCensus) -> bool {
    let Some(binding) = &census.source_binding else {
        return false;
    };
    let Some(policy) = census.source_policy else {
        return false;
    };
    binding.representation_certified()
        && source_binding_structure_faults(&census.inputs, policy, binding.inspection()).is_empty()
}
