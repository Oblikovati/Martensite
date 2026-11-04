//! Private construction keeps a certificate bound to its owned source and actual work.
use super::super::bezier_pair::hull_separation::HullSeparation;
use super::super::sketch_incidence::spans::closed_bridge::ProjectedQuadraticSpan;
use super::input::ClosedQuadraticExtrusionInput;
use super::self_span::SelfSpanWitness;
use super::types::{ClosedExtrusionCompletion, ClosedExtrusionPolicy, ClosedExtrusionWork};
#[derive(Clone, Debug, PartialEq)]
pub(super) enum ExclusionWitness {
    SelfSpan {
        span: usize,
        witness: SelfSpanWitness,
    },
    Pair {
        first: usize,
        second: usize,
        witness: Box<HullSeparation>,
    },
}
#[derive(Clone, Debug)]
/// Owned outcome; a sampled pass and missing obligations cannot construct a certificate.
pub struct ClosedExtrusionReport {
    pub(super) input: ClosedQuadraticExtrusionInput,
    pub(super) policy: ClosedExtrusionPolicy,
    pub(super) completion: ClosedExtrusionCompletion,
    pub(super) work: ClosedExtrusionWork,
    pub(super) spans: Vec<ProjectedQuadraticSpan>,
    pub(super) witnesses: Vec<ExclusionWitness>,
}
impl ClosedExtrusionReport {
    /// Read the actual completion; `report.completion()` keeps incomplete distinct.
    pub fn completion(&self) -> &ClosedExtrusionCompletion {
        &self.completion
    }
    /// Read requested finite settings; `report.policy().max_proof_steps`.
    pub fn policy(&self) -> ClosedExtrusionPolicy {
        self.policy
    }
    /// Read observed counts; `report.work().finished_obligations`.
    pub fn work(&self) -> ClosedExtrusionWork {
        self.work
    }
    /// Borrow the immutable represented support; `report.input().poles()`.
    pub fn input(&self) -> &ClosedQuadraticExtrusionInput {
        &self.input
    }
    /// Borrow an actual completed absence proof; `report.certificate().is_some()`.
    pub fn certificate(&self) -> Option<ClosedExtrusionCertificate<'_>> {
        if !complete_coverage(
            &self.completion,
            self.work,
            self.spans.len(),
            self.witnesses.len(),
        ) {
            return None;
        }
        Some(ClosedExtrusionCertificate { report: self })
    }
}
pub(in crate::predicates::roots) fn complete_coverage(
    completion: &ClosedExtrusionCompletion,
    work: ClosedExtrusionWork,
    spans: usize,
    witnesses: usize,
) -> bool {
    *completion == ClosedExtrusionCompletion::Certified
        && spans == work.spans
        && witnesses == work.scheduled_obligations
        && work.finished_obligations == work.scheduled_obligations
}

#[derive(Clone, Copy)]
/// A proved support on the endpoint-seam quotient; trims and mutable entities remain separate.
pub struct ClosedExtrusionCertificate<'a> {
    report: &'a ClosedExtrusionReport,
}
impl ClosedExtrusionCertificate<'_> {
    /// Read the exact owned input covered by the proof; `proof.input().direction()`.
    pub fn input(&self) -> &ClosedQuadraticExtrusionInput {
        &self.report.input
    }
    /// Read complete obligations; `proof.work().finished_obligations`.
    pub fn work(&self) -> ClosedExtrusionWork {
        self.report.work
    }
}
