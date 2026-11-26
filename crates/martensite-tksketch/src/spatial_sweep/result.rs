// SPDX-License-Identifier: GPL-2.0-only
use super::{SweepFailure, SweepOptions, SweepStage, Validation, poll};
use crate::native_shape_copy::{CopyFailure, ShapeCopy};
use martensite_tkbo::{
    body_check::ValidationReport,
    profile_extrusion::{AchievedTolerance, Finding},
};
use martensite_tkbrep::topods::{Shape, Shell};
use martensite_tksketch3d::{PointId, wire::DirectedLineUse};

/// A native band or a rounded filling at an authored station.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepPathPart {
    /// The directed generating path segment.
    Band(DirectedLineUse),
    /// A filling between the two directed segments at this authored vertex.
    Corner {
        /// Spine vertex at the turn.
        station: PointId,
        /// Directed leg entering the corner.
        incoming: DirectedLineUse,
        /// Directed leg leaving the corner.
        outgoing: DirectedLineUse,
    },
}

/// Both original sources of a retained sweep face.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SweepFaceHistory {
    /// The original directed cross-section line.
    pub profile: DirectedLineUse,
    /// Native path band or rounded filling station.
    pub path: SweepPathPart,
}

/// Direct builder history and actual incident faces of an edge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SweepEdgeHistory {
    /// Original profile point for a reported rail, if present.
    pub profile_point: Option<PointId>,
    /// Original path station for a reported section edge, if present.
    pub spine_station: Option<PointId>,
    /// Actual incident face roles in authored profile and path order.
    pub faces: Vec<SweepFaceHistory>,
}

/// Retained lineage of a published sheet member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SweepHistory {
    /// Native face from both source operands.
    Face(SweepFaceHistory),
    /// Native rail, section edge or intersection of labeled face boundaries.
    Edge(SweepEdgeHistory),
    /// Native vertex, derived from its exact incident edge occurrences.
    Vertex(Vec<SweepEdgeHistory>),
}

/// One completely named native member of a detached export.
#[derive(Debug)]
pub struct SweepImage {
    /// Actual topology within this export.
    pub shape: Shape,
    /// Complete recorded ancestry.
    pub history: SweepHistory,
}

/// One detached open native sheet and all its named faces, edges and vertices.
#[derive(Debug)]
pub struct DetachedSweep {
    /// Independent native shell.
    pub shell: Shape,
    /// Actual scoped members of this shell.
    pub images: Vec<SweepImage>,
}

/// Geometric checks in addition to the body's ordinary ordered report.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SweepSupportEvidence {
    /// Number of decoded side planes bound to source cross-section lines and spine bands.
    pub bands: usize,
    /// Number of decoded planar corner fillings.
    pub planar_corners: usize,
    /// Number of decoded revolved corner fillings.
    pub revolved_corners: usize,
    /// Number of terminal vertices matched by exact authored lineage and placement.
    pub terminal_vertices: usize,
    /// Requested maximum spatial trace error in model units.
    pub trace_limit: f64,
    /// Largest achieved spatial L1 upper bound across the certified input wire
    /// images, source-bound face spans, retained trim and vertex joins, and
    /// transported terminals. The solid also includes the two cap boundaries.
    pub achieved_trace_bound: f64,
}

/// Completed checked open sheet, with native input/output proof and private topology.
#[derive(Debug)]
pub struct SweepResult {
    shell: Shell,
    options: SweepOptions,
    validation: Validation,
    images: Vec<SweepImage>,
    supports: SweepSupportEvidence,
    estimate: f64,
}

impl SweepResult {
    pub(super) fn new(
        shell: Shell,
        options: SweepOptions,
        validation: Validation,
        images: Vec<SweepImage>,
        supports: SweepSupportEvidence,
        estimate: f64,
    ) -> Self {
        Self {
            shell,
            options,
            validation,
            images,
            supports,
            estimate,
        }
    }

    /// Ordinary four ordered native checks with their actual modes. Example: sweep.validation().
    pub fn validation(&self) -> &ValidationReport {
        &self.validation.report
    }

    /// Decoded carrier and terminal receipt. Example: sweep.support_evidence().
    pub fn support_evidence(&self) -> SweepSupportEvidence {
        self.supports
    }

    /// Native surface approximation estimate. Example: sweep.surface_error_estimate().
    pub fn surface_error_estimate(&self) -> f64 {
        self.estimate
    }

    /// Requested operation settings. Example: sweep.options().
    pub fn options(&self) -> SweepOptions {
        self.options
    }

    /// Largest retained stored body tolerances. Example: sweep.achieved_tolerance().max().
    pub fn achieved_tolerance(&self) -> AchievedTolerance {
        self.validation.achieved
    }

    /// Report-cap findings. Example: sweep.findings().
    pub fn findings(&self) -> &[Finding] {
        &self.validation.findings
    }

    /// All retained member roles in stable authored/derived order. Example: sweep.history().len().
    pub fn history(&self) -> impl ExactSizeIterator<Item = &SweepHistory> {
        self.images.iter().map(|image| &image.history)
    }

    /// Copy the sheet and every history member. Example: sweep.export_brep(|| false)?.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedSweep, SweepFailure> {
        poll(&mut cancelled, SweepStage::Export)?;
        let mut copy = ShapeCopy::new(&self.shell).map_err(copy_failure)?;
        let images = self
            .images
            .iter()
            .map(|source| {
                poll(&mut cancelled, SweepStage::Export)?;
                let shape = if matches!(&source.history, SweepHistory::Face(_)) {
                    copy.bind_face_occurrence(&source.shape)
                } else {
                    copy.bind(&source.shape)
                }
                .map_err(copy_failure)?;
                Ok(SweepImage {
                    history: source.history.clone(),
                    shape,
                })
            })
            .collect::<Result<Vec<_>, SweepFailure>>()?;
        poll(&mut cancelled, SweepStage::Export)?;
        Ok(DetachedSweep {
            shell: copy.into_root(),
            images,
        })
    }
}

pub(super) fn copy_failure(failure: CopyFailure) -> SweepFailure {
    match failure {
        CopyFailure::SharedRoot => SweepFailure::Binding("a fresh copied sheet"),
        CopyFailure::MissingMember => {
            SweepFailure::Binding("one distinct copied member under the detached sheet")
        }
        CopyFailure::Native(source) => SweepFailure::Native {
            stage: SweepStage::Export,
            source: Box::new(source),
        },
    }
}
