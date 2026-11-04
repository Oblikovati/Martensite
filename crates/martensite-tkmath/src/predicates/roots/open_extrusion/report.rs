//! Immutable original input and complete evidence jointly gate publication.
use super::super::closed_extrusion::operation::TraversalEvidence;
use super::{
    OpenExtrusionCompletion, OpenExtrusionPolicy, OpenExtrusionWork, OpenQuadraticExtrusionInput,
};

/// Owned finite-domain result; `report.certificate()` excludes incomplete work.
#[derive(Clone, Debug)]
pub struct OpenExtrusionReport {
    input: OpenQuadraticExtrusionInput,
    policy: OpenExtrusionPolicy,
    completion: OpenExtrusionCompletion,
    proof: TraversalEvidence,
}
impl OpenExtrusionReport {
    pub(super) fn from_proof(
        input: OpenQuadraticExtrusionInput,
        policy: OpenExtrusionPolicy,
        proof: TraversalEvidence,
    ) -> Self {
        let completion = OpenExtrusionCompletion::from(proof.completion());
        Self {
            input,
            policy,
            completion,
            proof,
        }
    }
    /// Read original owned storage; `report.input().poles()`.
    pub fn input(&self) -> &OpenQuadraticExtrusionInput {
        &self.input
    }
    /// Read unchanged requested limits; `report.policy()`.
    pub fn policy(&self) -> OpenExtrusionPolicy {
        self.policy
    }
    /// Read actual termination; `report.completion()`.
    pub fn completion(&self) -> &OpenExtrusionCompletion {
        &self.completion
    }
    /// Read whole-invocation work; `report.work()` cannot reset an exhausted allowance.
    pub fn work(&self) -> OpenExtrusionWork {
        self.proof.work()
    }
    /// Borrow a complete support proof; `report.certificate().is_some()`.
    pub fn certificate(&self) -> Option<OpenExtrusionCertificate<'_>> {
        if self.completion != OpenExtrusionCompletion::Certified || !self.proof.complete() {
            return None;
        }
        Some(OpenExtrusionCertificate { report: self })
    }
}

/// Injectivity on the ordinary complete parameter rectangle; no face incidence is implied.
#[derive(Clone, Copy)]
pub struct OpenExtrusionCertificate<'a> {
    report: &'a OpenExtrusionReport,
}
impl OpenExtrusionCertificate<'_> {
    /// Read exactly the owned source covered by this proof; `certificate.input()`.
    pub fn input(&self) -> &OpenQuadraticExtrusionInput {
        &self.report.input
    }
    /// Read the complete obligation counts; `certificate.work()`.
    pub fn work(&self) -> OpenExtrusionWork {
        self.report.work()
    }
}
