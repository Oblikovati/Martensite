// SPDX-License-Identifier: GPL-2.0-only
//! Executed V5 integral programs retain ordered source roles and actual generation loans.
use super::{binding::SegmentBinding, integral_program::*, types::*};
use crate::sketch_region::RegionSourceId;
use martensite_tkmath::predicates::roots::metric_bounds::*;
use std::sync::Arc;
mod binding;
mod error;
pub(crate) use error::ProgramFailure;
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum IntegralRole {
    Boundary(usize),
    Face(u64),
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IntegralSource {
    pub id: RegionSourceId,
    pub poles: Vec<[f64; 2]>,
    pub weights: Option<Vec<f64>>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum IntegralStep {
    Direct {
        role: IntegralRole,
        loops: Vec<Vec<AuthoredLoopUse>>,
        proof: Arc<CertifiedIntegralProgram>,
    },
    Reverse {
        parent: usize,
        uses: Vec<AuthoredLoopUse>,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GraphIntegralEvidence {
    pub options: IntegralProgramOptions,
    pub sources: Vec<IntegralSource>,
    pub steps: Vec<IntegralStep>,
    pub consumed: u64,
}
pub(crate) struct ProgramTrace {
    ledger: IntegralLedger,
    evidence: GraphIntegralEvidence,
}
impl ProgramTrace {
    pub(super) fn new(options: IntegralProgramOptions, source: &SegmentBinding<'_>) -> Self {
        let sources = captured_sources(source);
        Self {
            ledger: IntegralLedger::new(options),
            evidence: GraphIntegralEvidence {
                options,
                sources,
                steps: Vec::new(),
                consumed: 0,
            },
        }
    }
    pub(crate) fn consumed(&self) -> u64 {
        self.ledger.consumed()
    }
    pub(crate) fn position(&self) -> usize {
        self.evidence.steps.len()
    }
    pub(crate) fn reverse(&mut self, parent: usize, uses: Vec<AuthoredLoopUse>) {
        self.evidence
            .steps
            .push(IntegralStep::Reverse { parent, uses });
    }
    pub(crate) fn boundary(
        &mut self,
        orbit: usize,
        uses: Vec<AuthoredLoopUse>,
        curves: &[OrientedMetricCurve<'_>],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Arc<CertifiedIntegralProgram>, ProgramFailure> {
        let result = self.ledger.boundary(curves, cancelled);
        let proof = result.map_err(|cause| {
            ProgramFailure::new(
                IntegralRole::Boundary(orbit),
                vec![uses.clone()],
                self.ledger.consumed(),
                cause,
            )
        })?;
        Ok(self.retain(IntegralRole::Boundary(orbit), vec![uses], proof))
    }
    pub(crate) fn face(
        &mut self,
        name: u64,
        uses: Vec<Vec<AuthoredLoopUse>>,
        curves: &[&[OrientedMetricCurve<'_>]],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Arc<CertifiedIntegralProgram>, ProgramFailure> {
        let result = self.ledger.face(curves, cancelled);
        let proof = result.map_err(|cause| {
            ProgramFailure::new(
                IntegralRole::Face(name),
                uses.clone(),
                self.ledger.consumed(),
                cause,
            )
        })?;
        Ok(self.retain(IntegralRole::Face(name), uses, proof))
    }
    fn retain(
        &mut self,
        role: IntegralRole,
        loops: Vec<Vec<AuthoredLoopUse>>,
        proof: CertifiedIntegralProgram,
    ) -> Arc<CertifiedIntegralProgram> {
        let proof = Arc::new(proof);
        self.evidence.steps.push(IntegralStep::Direct {
            role,
            loops,
            proof: Arc::clone(&proof),
        });
        proof
    }
    pub(crate) fn finish(mut self) -> GraphIntegralEvidence {
        self.evidence.consumed = self.ledger.consumed();
        self.evidence
    }
}

fn captured_sources(source: &SegmentBinding<'_>) -> Vec<IntegralSource> {
    source
        .sources
        .iter()
        .map(|id| {
            let segment = source.segment(*id);
            IntegralSource {
                id: *id,
                poles: segment.controls.clone(),
                weights: segment.weights.clone(),
            }
        })
        .collect()
}
