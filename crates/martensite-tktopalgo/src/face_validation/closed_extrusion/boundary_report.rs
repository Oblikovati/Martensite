//! The carrier witness names its actual input occurrence and retains its independent proof.
use martensite_tkmath::predicates::roots::closed_extrusion::ClosedExtrusionReport;
/// Immutable actual-boundary work, separate from the ideal surface; `report.boundary()`.
#[derive(Debug)]
pub struct ClosedFaceBoundaryReport {
    pub(super) edge_owner: usize,
    pub(super) occurrence: usize,
    pub(super) reference_parameter: f64,
    pub(super) transverse_coordinate: Option<usize>,
    pub(super) relations_verified: bool,
    pub(super) proof: Option<ClosedExtrusionReport>,
}
impl ClosedFaceBoundaryReport {
    /// Read the invocation-local topology owner; `boundary.edge_owner()` is not a persistent ID.
    pub fn edge_owner(&self) -> usize {
        self.edge_owner
    }
    /// Read the actual reference-cap occurrence address; `boundary.edge_occurrence()`.
    pub fn edge_occurrence(&self) -> usize {
        self.occurrence
    }
    /// Read its unchanged parameter level; `boundary.reference_parameter()`.
    pub fn reference_parameter(&self) -> f64 {
        self.reference_parameter
    }
    /// Read the proven transverse coordinate functional; `boundary.transverse_coordinate()`.
    pub fn transverse_coordinate(&self) -> Option<usize> {
        self.transverse_coordinate
    }
    /// Read completion of all exact boundary relations; `boundary.relations_verified()`.
    pub fn relations_verified(&self) -> bool {
        self.relations_verified
    }
    /// Read actual independent support work and termination; `boundary.proof()`.
    pub fn proof(&self) -> Option<&ClosedExtrusionReport> {
        self.proof.as_ref()
    }
    pub(super) fn complete(&self) -> bool {
        self.relations_verified
            && self.transverse_coordinate.is_some()
            && self
                .proof
                .as_ref()
                .is_some_and(|proof| proof.certificate().is_some())
    }
}
