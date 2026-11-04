//! Explicit tolerance-resolved incidence never changes the exact replay policies.
use super::super::{ClosedFaceCompletion, ClosedFaceErrorBound, ClosedFacePolicy, ClosedFaceWork};
use martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot;
use martensite_tkmath::predicates::roots::sketch_incidence::vertex_neighborhood::*;
use martensite_tkmath::predicates::roots::{closed_extrusion::*, open_extrusion::*};

/// Finite settings for the separate tolerance-resolved behavior four.
/// Example: `ToleranceResolvedFacePolicy::v4()` preserves raw geometry limits.
#[derive(Clone, Copy, Debug)]
pub struct ToleranceResolvedFacePolicy {
    /// This distinct operation accepts only behavior four.
    pub version: u32,
    /// Existing finite face settings; the required relative-frame behavior is three.
    pub face: ClosedFacePolicy,
    /// One whole-operation incidence/component work allowance.
    pub max_incidence_work: u64,
}
impl ToleranceResolvedFacePolicy {
    /// Select finite behavior four; example: `ToleranceResolvedFacePolicy::v4()`.
    pub fn v4() -> Self {
        Self {
            version: 4,
            face: ClosedFacePolicy::placed_piecewise_boundary(),
            max_incidence_work: 1_000_000,
        }
    }
}
/// Ordered coverage for tolerance-resolved incidence, without a raw exact embedding claim.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToleranceResolvedFaceCoverage {
    /// Exact source topology, complete trim rectangle and declared endpoint classes.
    pub topology: bool,
    /// All existing full-domain spatial consistency checks completed.
    pub geometry: bool,
    /// The complete ideal support has its appropriate open or closed injectivity proof.
    pub support: bool,
    /// Actual carrier, contacts, proper endpoint components and nonincident exclusions completed.
    pub represented_noninterference: bool,
    /// Every accepted bound respects its original raw limits.
    pub tolerances: bool,
}
/// Which immutable analytic domain was actually proved.
#[derive(Clone, Debug)]
pub enum ToleranceResolvedSupportProof {
    /// Exact endpoint quotient, retaining the original closed-domain report.
    Closed(ClosedExtrusionReport),
    /// Unidentified geometric endpoints, retaining the ordinary open-domain report.
    Open(OpenExtrusionReport),
}
impl ToleranceResolvedSupportProof {
    /// Read complete injectivity only; example: `proof.complete()` does not certify a face.
    pub fn complete(&self) -> bool {
        match self {
            Self::Closed(p) => p.certificate().is_some(),
            Self::Open(p) => p.certificate().is_some(),
        }
    }
}
/// Actual noncompletion; no partial geometry is returned as a successful result.
#[derive(Debug)]
pub enum ToleranceResolvedFaceCompletion {
    /// Every obligation of the explicitly tolerance-resolved contract completed.
    Certified,
    /// An unchanged topology, metric, placement or exact-relation gate refused.
    Face(ClosedFaceCompletion),
    /// Actual neighborhood operation refused; its input and original cause are retained.
    Neighborhood {
        /// Actual edge occurrence in the captured traversal.
        edge: usize,
        /// Complete actual neighborhood refusal and owned input.
        report: Box<GivenVertexNeighborhoodReport>,
    },
    /// Complete actual contact accounting could not admit the declared incidence.
    Contacts {
        /// Actual cap occurrence.
        cap: usize,
        /// Actual seam occurrence.
        seam: usize,
        /// Original contact classification and owned operands.
        report: Box<GivenVertexContactsReport>,
    },
    /// The distinct declared vertex neighborhoods are not strictly separated.
    Disjoint(Box<DisjointVertexNeighborhoodReport>),
    /// Actual ideal or carrier support proof remains unfinished.
    Support { owner: usize, role: &'static str },
    /// The one incidence work allowance was exhausted.
    Work {
        /// Unchanged requested allowance.
        limit: u32,
        /// Actually charged work.
        actual: u64,
    },
}
/// Actual curve/line evidence bound by declared traversal addresses, never proximity.
#[derive(Clone, Debug)]
pub struct ToleranceResolvedIncidence {
    /// Cap occurrence in the captured canonical boundary traversal.
    pub cap: usize,
    /// Seam occurrence in that same traversal.
    pub seam: usize,
    /// Given vertex owner in the coherent source snapshot.
    pub vertex_owner: usize,
    /// Declared bottom/top endpoint class, an invocation-local address only.
    pub vertex_class: usize,
    /// Complete owned component/contact witnesses.
    pub proof: GivenVertexContactsReport,
}
/// Immutable full-operation evidence; no live geometry handles are retained.
/// Example: `report.certificate()` requires every ordered check of behavior four.
#[derive(Debug)]
pub struct ToleranceResolvedFaceReport {
    pub(in crate::face_validation::closed_extrusion) input: FaceInputSnapshot,
    pub(in crate::face_validation::closed_extrusion) policy: ToleranceResolvedFacePolicy,
    pub(in crate::face_validation::closed_extrusion) completion: ToleranceResolvedFaceCompletion,
    pub(in crate::face_validation::closed_extrusion) coverage: ToleranceResolvedFaceCoverage,
    pub(in crate::face_validation::closed_extrusion) work: ClosedFaceWork,
    pub(in crate::face_validation::closed_extrusion) incidence_work: u64,
    pub(in crate::face_validation::closed_extrusion) errors: Vec<ClosedFaceErrorBound>,
    pub(in crate::face_validation::closed_extrusion) support: Option<ToleranceResolvedSupportProof>,
    pub(in crate::face_validation::closed_extrusion) carrier: Option<ToleranceResolvedSupportProof>,
    pub(in crate::face_validation::closed_extrusion) incidences: Vec<ToleranceResolvedIncidence>,
    pub(in crate::face_validation::closed_extrusion) transverse_coordinate: Option<usize>,
    pub(in crate::face_validation::closed_extrusion) disjoint:
        Option<DisjointVertexNeighborhoodReport>,
}

pub(in crate::face_validation::closed_extrusion) type ResolvedResult<T> =
    Result<T, ToleranceResolvedFaceCompletion>;
