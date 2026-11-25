// SPDX-License-Identifier: GPL-2.0-only
//! Protected native result and an exact fresh topology copy for each export.
use super::{LoftFailure, LoftOptions, LoftSectionId, LoftStage, binding::Evidence, poll};
use crate::native_shape_copy::{CopyFailure, ShapeCopy};
use martensite_tkbo::{
    body_check::ValidationReport,
    profile_extrusion::{AchievedTolerance, Finding, Measured, Resolution},
};
use martensite_tkbrep::topods::{Shape, Solid};
use martensite_tksketch3d::{PointId, wire::DirectedLineUse};
use std::cmp::Ordering;

/// One line identity and direction within its selected loft section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoftLine {
    /// Selected source section.
    pub section: LoftSectionId,
    /// The authored directed occurrence.
    pub directed: DirectedLineUse,
}

/// One point identity within its selected loft section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoftPoint {
    /// Selected source section.
    pub section: LoftSectionId,
    /// The authored point.
    pub point: PointId,
}

/// Complete native role and original parents of a ruled loft image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoftHistory {
    /// A cap derived from the entire selected section.
    Cap(LoftSectionId),
    /// A section edge derived from one authored line.
    SectionEdge(LoftLine),
    /// A section vertex derived from one authored point.
    SectionVertex(LoftPoint),
    /// A side face generated between corresponding directed lines in section order.
    Side([LoftLine; 2]),
    /// A longitudinal edge generated between corresponding section points.
    Rail([LoftPoint; 2]),
}

/// One associated native shape inside the detached solid.
#[derive(Debug)]
pub struct LoftImage {
    /// Complete authored lineage.
    pub history: LoftHistory,
    /// Native topology in this export only.
    pub shape: Shape,
}

/// One fresh consumer-owned native solid and its exact image bindings.
pub struct DetachedLoft {
    /// Fresh solid root.
    pub solid: Shape,
    /// Every face, edge and vertex under the root with its authored lineage.
    pub images: Vec<LoftImage>,
}

/// Complete ordered evidence and privately retained native result.
#[derive(Debug)]
pub struct LoftResult {
    solid: Solid,
    validation: ValidationReport,
    station_sign: Ordering,
    options: LoftOptions,
    evidence: Evidence,
}

impl LoftResult {
    pub(super) fn new(
        solid: Solid,
        validation: ValidationReport,
        station_sign: Ordering,
        options: LoftOptions,
        evidence: Evidence,
    ) -> Self {
        Self {
            solid,
            validation,
            station_sign,
            options,
            evidence,
        }
    }

    /// Four ordered body checks. Example: loft.validation().is_valid().
    pub fn validation(&self) -> &ValidationReport {
        &self.validation
    }

    /// Exact station sign relative to the first section's ordered plane. Example: loft.station_sign().
    pub fn station_sign(&self) -> Ordering {
        self.station_sign
    }

    /// Certified enclosure integrated from the retained planar trims. Example: loft.volume().
    pub fn volume(&self) -> Measured {
        self.evidence.volume
    }

    /// Upper bound on the retained boundary error against the authored sections and rails.
    /// Example: loft.source_trace_bound() <= loft.source_trace_limit().
    pub fn source_trace_bound(&self) -> f64 {
        self.evidence.trace_bound
    }

    /// Requested spatial trace limit. Example: loft.source_trace_limit().
    pub fn source_trace_limit(&self) -> f64 {
        self.options.source_trace_limit
    }

    /// Requested native resolution. Example: loft.resolution().weld.
    pub fn resolution(&self) -> Resolution {
        self.options.resolution
    }

    /// Largest retained native tolerances. Example: loft.achieved_tolerance().max().
    pub fn achieved_tolerance(&self) -> AchievedTolerance {
        self.evidence.achieved
    }

    /// Nonfatal report-cap findings. Example: loft.findings().
    pub fn findings(&self) -> &[Finding] {
        &self.evidence.findings
    }

    /// Authored image history in explicit section/correspondence order. Example: loft.history().len().
    pub fn history(&self) -> impl ExactSizeIterator<Item = LoftHistory> + '_ {
        self.evidence.images.iter().map(|image| image.history)
    }

    /// Copy the body and all semantic members as one detached export.
    /// Example: loft.export_brep(|| false)?.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedLoft, LoftFailure> {
        poll(&mut cancelled, LoftStage::Export)?;
        let mut copy = ShapeCopy::new(&self.solid).map_err(copy_failure)?;
        let images = self
            .evidence
            .images
            .iter()
            .map(|source| {
                poll(&mut cancelled, LoftStage::Export)?;
                let shape = copy.bind(&source.shape).map_err(copy_failure)?;
                Ok(LoftImage {
                    history: source.history,
                    shape,
                })
            })
            .collect::<Result<_, LoftFailure>>()?;
        poll(&mut cancelled, LoftStage::Export)?;
        Ok(DetachedLoft {
            solid: copy.into_root(),
            images,
        })
    }
}

fn copy_failure(failure: CopyFailure) -> LoftFailure {
    match failure {
        CopyFailure::SharedRoot => LoftFailure::Binding("an independent solid copy"),
        CopyFailure::MissingMember => {
            LoftFailure::Binding("a distinct member under the copied solid")
        }
        CopyFailure::Native(source) => LoftFailure::Native {
            stage: LoftStage::Export,
            source: Box::new(source),
        },
    }
}
