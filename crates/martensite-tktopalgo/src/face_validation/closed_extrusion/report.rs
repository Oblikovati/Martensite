//! Only the completed ordered pipeline can publish an owned certificate.
use super::{boundary_report::ClosedFaceBoundaryReport, types::*};
use martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot;
use martensite_tkmath::predicates::roots::closed_extrusion::ClosedExtrusionReport;
/// Immutable face-input validation outcome; `report.certificate()` requires all obligations.
#[derive(Debug)]
pub struct ClosedFaceReport {
    pub(super) input: FaceInputSnapshot,
    pub(super) policy: ClosedFacePolicy,
    pub(super) completion: ClosedFaceCompletion,
    pub(super) coverage: ClosedFaceCoverage,
    pub(super) work: ClosedFaceWork,
    pub(super) errors: Vec<ClosedFaceErrorBound>,
    pub(super) support: Option<ClosedExtrusionReport>,
    pub(super) boundary: Option<super::boundary_report::ClosedFaceBoundaryReport>,
}
impl ClosedFaceReport {
    /// Read the owned input, without live geometry; `report.input()`.
    pub fn input(&self) -> &FaceInputSnapshot {
        &self.input
    }
    /// Read the unchanged requested policy; `report.policy()`.
    pub fn policy(&self) -> ClosedFacePolicy {
        self.policy
    }
    /// Read actual termination; `report.completion()`.
    pub fn completion(&self) -> &ClosedFaceCompletion {
        &self.completion
    }
    /// Read completed ordered checks; `report.coverage()`.
    pub fn coverage(&self) -> ClosedFaceCoverage {
        self.coverage
    }
    /// Read observed work; `report.work()`.
    pub fn work(&self) -> ClosedFaceWork {
        self.work
    }
    /// Read certified error uppers and their original limits; `report.error_bounds()`.
    pub fn error_bounds(&self) -> &[ClosedFaceErrorBound] {
        &self.errors
    }
    /// Read the actual support proof, including incomplete outcomes; `report.support()`.
    pub fn support(&self) -> Option<&ClosedExtrusionReport> {
        self.support.as_ref()
    }
    /// Read the separately bound actual-boundary proof, including incomplete work; `report.boundary()`.
    pub fn boundary(&self) -> Option<&super::boundary_report::ClosedFaceBoundaryReport> {
        self.boundary.as_ref()
    }
    /// Read the non-destructive validation history; `report.history()`.
    pub fn history(&self) -> ClosedFaceHistory {
        ClosedFaceHistory::NoGeometryChanges
    }
    /// Borrow a complete certificate; `report.certificate().is_some()` is stronger than capture.
    pub fn certificate(&self) -> Option<ClosedFaceCertificate<'_>> {
        let c = self.coverage;
        if !matches!(self.completion, ClosedFaceCompletion::Certified)
            || !c.topology
            || !c.geometry
            || !c.self_exclusion
            || !c.tolerances
            || self.support.as_ref()?.certificate().is_none()
            || !self.required_boundary_verified()
        {
            return None;
        }
        Some(ClosedFaceCertificate {
            report: self,
            mode: self.policy.check_mode()?,
        })
    }
    fn required_boundary_verified(&self) -> bool {
        required_boundary(self.policy.version, self.boundary.as_ref())
    }
}
/// Complete represented-face proof bound to its immutable input, not a later live handle.
pub struct ClosedFaceCertificate<'a> {
    report: &'a ClosedFaceReport,
    mode: ClosedFaceCheckMode,
}
impl ClosedFaceCertificate<'_> {
    /// Read the exact represented-geometry contract proved; `certificate.check_mode()`.
    pub fn check_mode(&self) -> ClosedFaceCheckMode {
        self.mode
    }
    /// Read the actual certified input; `certificate.input()`.
    pub fn input(&self) -> &FaceInputSnapshot {
        &self.report.input
    }
    /// Read the certificate's ordered obligation coverage; `certificate.coverage()`.
    pub fn coverage(&self) -> ClosedFaceCoverage {
        self.report.coverage
    }
}

fn required_boundary(version: u32, boundary: Option<&ClosedFaceBoundaryReport>) -> bool {
    match version {
        1 => true,
        2 | 3 => boundary.is_some_and(|proof| proof.complete()),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placed_publication_refuses_missing_or_unfinished_boundary() {
        assert!(!required_boundary(3, None));
        let mut boundary = unfinished_boundary();
        assert!(!required_boundary(3, Some(&boundary)));
        boundary.relations_verified = true;
        boundary.transverse_coordinate = Some(2);
        assert!(!required_boundary(3, Some(&boundary)));
    }
    fn unfinished_boundary() -> ClosedFaceBoundaryReport {
        ClosedFaceBoundaryReport {
            edge_owner: 17,
            occurrence: 1,
            reference_parameter: -1.,
            transverse_coordinate: None,
            relations_verified: false,
            proof: None,
        }
    }
}
