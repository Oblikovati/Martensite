//! Publication is bound to the explicit tolerance-resolved contract and complete witnesses.
use super::super::{ClosedFaceErrorBound, ClosedFaceHistory, ClosedFaceWork};
use super::types::*;
use martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot;
impl ToleranceResolvedFaceReport {
    /// Read the complete owned source; example: `report.input().cells()`.
    pub fn input(&self) -> &FaceInputSnapshot {
        &self.input
    }
    /// Read explicit behavior and requested limits; example: `report.policy().version == 4`.
    pub fn policy(&self) -> ToleranceResolvedFacePolicy {
        self.policy
    }
    /// Read actual termination; example: `report.completion()`.
    pub fn completion(&self) -> &ToleranceResolvedFaceCompletion {
        &self.completion
    }
    /// Read ordered contract coverage; example: `report.coverage().geometry`.
    pub fn coverage(&self) -> ToleranceResolvedFaceCoverage {
        self.coverage
    }
    /// Read actual original pipeline work; example: `report.work().queries`.
    pub fn work(&self) -> ClosedFaceWork {
        self.work
    }
    /// Read shared incidence work; example: `report.incidence_work()`.
    pub fn incidence_work(&self) -> u64 {
        self.incidence_work
    }
    /// Read unchanged raw-limit error bounds; example: `report.error_bounds()`.
    pub fn error_bounds(&self) -> &[ClosedFaceErrorBound] {
        &self.errors
    }
    /// Read the ideal support proof; example: `report.support()`.
    pub fn support(&self) -> Option<&ToleranceResolvedSupportProof> {
        self.support.as_ref()
    }
    /// Read the independently bound actual carrier proof; example: `report.carrier()`.
    pub fn carrier(&self) -> Option<&ToleranceResolvedSupportProof> {
        self.carrier.as_ref()
    }
    /// Read every actual contact witness; example: `report.incidences()`.
    pub fn incidences(&self) -> &[ToleranceResolvedIncidence] {
        &self.incidences
    }
    /// Read the exact nonzero common height functional; example: `report.transverse_coordinate()`.
    pub fn transverse_coordinate(&self) -> Option<usize> {
        self.transverse_coordinate
    }
    /// Read unchanged-geometry history; example: `report.history()`.
    pub fn history(&self) -> ClosedFaceHistory {
        ClosedFaceHistory::NoGeometryChanges
    }
    /// Borrow only complete tolerance-resolved evidence; example: `report.certificate()`.
    pub fn certificate(&self) -> Option<ToleranceResolvedFaceCertificate<'_>> {
        if !self.complete() {
            return None;
        }
        Some(ToleranceResolvedFaceCertificate { report: self })
    }
    fn complete(&self) -> bool {
        let c = self.coverage;
        matches!(self.completion, ToleranceResolvedFaceCompletion::Certified)
            && self.policy.version == 4
            && c.topology
            && c.geometry
            && c.support
            && c.represented_noninterference
            && c.tolerances
            && self.support.as_ref().is_some_and(|p| p.complete())
            && self.carrier.as_ref().is_some_and(|p| p.complete())
            && self.transverse_coordinate.is_some()
            && self.incidences.len() == 4
            && self.incidences.iter().all(|p| p.proof.contacts().is_some())
            && self.disjoint.as_ref().is_some_and(|p| p.separated())
    }
}
/// A tolerance-resolved represented face, not an exact raw-wire embedding or a valid enclosing body.
pub struct ToleranceResolvedFaceCertificate<'a> {
    report: &'a ToleranceResolvedFaceReport,
}
impl ToleranceResolvedFaceCertificate<'_> {
    /// Read the explicit behavior; example: `certificate.behavior_version() == 4`.
    pub fn behavior_version(&self) -> u32 {
        4
    }
    /// Read the immutable source actually proved; example: `certificate.input()`.
    pub fn input(&self) -> &FaceInputSnapshot {
        &self.report.input
    }
    /// Read the full operation evidence; example: `certificate.report().incidences()`.
    pub fn report(&self) -> &ToleranceResolvedFaceReport {
        self.report
    }
}
