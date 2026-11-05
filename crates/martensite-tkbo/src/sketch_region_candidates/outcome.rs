// SPDX-License-Identifier: GPL-2.0-only
use crate::sketch_region::{ArrangementRelation, OriginalMemberIdentity};
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::roots::bezier_line::BezierWireError;
#[derive(Clone)]
pub(crate) struct CandidateSegment {
    pub occurrence: u64,
    pub edge: Shape,
    pub parents: Vec<OriginalMemberIdentity>,
    pub controls: Vec<[f64; 2]>,
    pub weights: Option<Vec<f64>>,
    pub interval: [f64; 2],
    pub original_interval: [f64; 2],
    pub reversed: bool,
}
#[derive(Clone)]
pub(crate) struct CandidateWire {
    pub segments: Vec<CandidateSegment>,
}
pub(crate) struct RegionCandidate {
    pub face: Face,
    pub wires: Vec<CandidateWire>,
    pub relations: Vec<ArrangementRelation>,
}
pub(crate) struct CandidateBatch {
    pub plane: martensite_tkmath::gp::Plane3,
    pub candidates: Vec<RegionCandidate>,
    pub input_deleted: Vec<OriginalMemberIdentity>,
    pub unrepresented: Vec<ArrangementRelation>,
    pub mapped_report: std::sync::Arc<martensite_tkernel::message::Report>,
}
pub(crate) enum CandidateCause {
    Mapped(Failure),
    MappedReport(std::sync::Arc<martensite_tkernel::message::Report>),
    Incomplete(CandidateGap),
    Cancelled(CandidatePhase),
    Numerical(BezierWireError),
    CapturedNumerical(martensite_tkmath::predicates::roots::bezier_line::CapturedBezierWireFailure),
    CapturedOperation(Box<super::captured_store::CandidateCapturedFailure>),
}
pub(crate) enum CandidateGap {
    WindingOrientationOverflow {
        winding: i64,
    },
    UnsupportedFaceOrientation {
        orientation: martensite_tkg3d::top_abs::Orientation,
    },
    UnclassifiedOccurrence {
        member: Shape,
    },
    WorkBudget {
        requested: u16,
        used: u32,
    },
    UnattributedMember {
        member: Shape,
    },
    UnsupportedCurve {
        occurrence: u64,
        dynamic_type: String,
    },
    MissingParents {
        occurrence: u64,
    },
    IncompleteWire {
        omitted: Vec<Shape>,
        unexpected: Vec<Shape>,
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CandidatePhase {
    Copy,
    Generation,
    Capture,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MissingCertificate {
    Construction,
    SharedIncidenceReuse,
    CandidateHistoryCoverage,
    SearchCompleteness,
    SelfIntersection,
    Tolerance,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SeedSelection {
    Empty,
    Boundary { occurrences: Vec<u64> },
    Unique { candidate: usize },
    Ambiguous { candidates: Vec<usize> },
}
pub(crate) enum CandidateClassification {
    Incomplete {
        selection: SeedSelection,
        missing: Vec<MissingCertificate>,
        bisections: u32,
        requested: martensite_tkmath::predicates::roots::RootIsolationOptions,
        remaining: martensite_tkmath::predicates::roots::RootIsolationOptions,
        achieved_error: Option<f64>,
        captured: Box<super::captured_store::CandidateCaptureReceipt>,
    },
}
