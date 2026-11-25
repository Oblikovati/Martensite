// SPDX-License-Identifier: GPL-2.0-only
//! Solid closure, exact native cap ownership and final-volume publication.
mod cap;

use super::{
    Prepared, SweepFailure as Failure, SweepHistory, SweepOptions, SweepStage as Stage,
    SweepSupportEvidence, Validation, include_wire_trace, lineage, native, poll, prepare,
    result::copy_failure, trim_proof, validate,
};
use crate::native_shape_copy::ShapeCopy;
use martensite_tkbo::{
    body_check::{ResolutionRejected, ValidationReport},
    profile_extrusion::{AchievedTolerance, Finding, Measured},
};
use martensite_tkbrep::{
    topexp,
    topods::{self, Shape, Solid},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSweep;
use martensite_tksketch3d::{
    PointId,
    wire::{DirectedLineUse, WireRecipe},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Requested construction, geometry and outward-volume limits for the solid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolidSweepOptions {
    /// The ordinary round-corner construction and its retained surface limits.
    pub sweep: SweepOptions,
    /// Maximum absolute uncertainty of the certified volume, in cubic model units.
    pub volume_error_limit: f64,
}

/// Authored lineage on the final closed body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolidSweepHistory {
    /// Member retained from the lateral sweep.
    Lateral(SweepHistory),
    /// A native closing face at an original path station.
    Cap {
        /// Opening or closing authored spine point.
        station: PointId,
        /// Original directed profile sides in their authored order.
        profile: [DirectedLineUse; 4],
    },
}

/// An actual topological member and its complete ancestry.
#[derive(Debug)]
pub struct SolidSweepImage {
    /// Actual member in this detached native body.
    pub shape: Shape,
    /// Binding to authored sources.
    pub history: SolidSweepHistory,
}

/// Independent native solid and its ancestry images.
#[derive(Debug)]
pub struct DetachedSolidSweep {
    /// Fresh native body.
    pub solid: Shape,
    /// Faces, edges and vertices in stable authored and incidence order.
    pub images: Vec<SolidSweepImage>,
}

/// Certified round-corner solid with private native state.
#[derive(Debug)]
pub struct SolidSweepResult {
    solid: Solid,
    images: Vec<SolidSweepImage>,
    options: SolidSweepOptions,
    validation: Validation,
    supports: SweepSupportEvidence,
    volume: Measured,
    closure: f64,
    estimate: f64,
}

/// Form and close the bounded rounded sweep on three orthogonal legs.
/// Every cap, carrier and trim must support an outward physical volume enclosure.
/// Native building and final-body checking finish before the next cancellation poll.
/// Example: `sweep_rounded_solid(path, section, options, || false)?`.
pub fn sweep_rounded_solid(
    path: &WireRecipe,
    section: &WireRecipe,
    options: SolidSweepOptions,
    mut cancelled: impl FnMut() -> bool,
) -> Result<SolidSweepResult, Failure> {
    admit(options)?;
    let Prepared {
        admission,
        wires,
        mut pipe,
        shell,
        terminals: [first, last],
    } = prepare(path, section, options.sweep, &mut cancelled)?;
    let mut pending = lineage::snapshot(
        &mut pipe,
        &shell,
        &wires,
        [&first, &last],
        &admission,
        &mut cancelled,
    )?;
    poll(&mut cancelled, Stage::Construction)?;
    let closed = pipe.make_solid();
    poll(&mut cancelled, Stage::Construction)?;
    if !native(closed, Stage::Construction)? {
        return Err(Failure::SolidClosure);
    }
    let body = native(
        topods::solid(native(pipe.shape(), Stage::Construction)?),
        Stage::Construction,
    )?;
    let validation = validate(&body, options.sweep, &mut cancelled)?;
    pending.retain_faces(&body, &mut cancelled)?;
    let (lateral, mut supports) = pending.complete(&admission, options.sweep, &mut cancelled)?;
    include_wire_trace(&mut supports, &wires);
    let cap_sources = [
        native(pipe.first_shape(), Stage::Binding)?,
        native(pipe.last_shape(), Stage::Binding)?,
    ];
    let (images, cap_bound) = cap::bind(
        &body,
        cap_sources,
        lateral,
        &wires[1],
        &admission,
        options.sweep,
        &mut cancelled,
    )?;
    supports.achieved_trace_bound = supports.achieved_trace_bound.max(cap_bound);
    let volume = trim_proof::certify(&images, admission.stations[0].xyz, options, &mut cancelled)?;
    supports.achieved_trace_bound = supports.achieved_trace_bound.max(volume.trace);
    poll(&mut cancelled, Stage::Publication)?;
    Ok(SolidSweepResult {
        solid: body,
        images,
        validation,
        volume: volume.measured,
        closure: volume.closure,
        options,
        supports,
        estimate: pipe.error_on_surface(),
    })
}

fn admit(options: SolidSweepOptions) -> Result<(), Failure> {
    let value = options.volume_error_limit;
    if !value.is_finite() || value <= 0. {
        return Err(Failure::Resolution(ResolutionRejected {
            field: "volume_error_limit",
            value,
        }));
    }
    Ok(())
}

impl SolidSweepResult {
    /// Requested solid limits. Example: `result.options().volume_error_limit`.
    pub fn options(&self) -> SolidSweepOptions {
        self.options
    }

    /// Ordered validation on the actual closed body. Example: `result.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.validation.report
    }

    /// Retained side carrier and terminal receipt. Example: `result.support_evidence().bands`.
    pub fn support_evidence(&self) -> SweepSupportEvidence {
        self.supports
    }

    /// Native surface approximation estimate. Example: `result.surface_error_estimate()`.
    pub fn surface_error_estimate(&self) -> f64 {
        self.estimate
    }

    /// Largest stored native tolerances. Example: `result.achieved_tolerance().max()`.
    pub fn achieved_tolerance(&self) -> AchievedTolerance {
        self.validation.achieved
    }

    /// Any report-limit findings. Example: `result.findings().is_empty()`.
    pub fn findings(&self) -> &[Finding] {
        &self.validation.findings
    }

    /// Outward enclosure of actual physical volume. Example: `result.volume().error_bound`.
    pub fn volume(&self) -> Measured {
        self.volume
    }

    /// Certified contribution of the common-edge closure strips and vertex fans
    /// to the outward volume uncertainty, in cubic model units.
    /// Example: `result.closure_volume_error_bound()`.
    pub fn closure_volume_error_bound(&self) -> f64 {
        self.closure
    }

    /// Stable authored and retained native history. Example: `result.history().len()`.
    pub fn history(&self) -> impl ExactSizeIterator<Item = &SolidSweepHistory> {
        self.images.iter().map(|image| &image.history)
    }

    /// Deep copy the body and bind every exported member. Example: `result.export_brep(|| false)?`.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedSolidSweep, Failure> {
        poll(&mut cancelled, Stage::Export)?;
        let mut copy = ShapeCopy::new(&self.solid).map_err(copy_failure)?;
        let images = self
            .images
            .iter()
            .map(|original| {
                poll(&mut cancelled, Stage::Export)?;
                let shape = match &original.history {
                    SolidSweepHistory::Cap { .. }
                    | SolidSweepHistory::Lateral(SweepHistory::Face(_)) => {
                        copy.bind_face_occurrence(&original.shape)
                    }
                    _ => copy.bind(&original.shape),
                }
                .map_err(copy_failure)?;
                Ok(SolidSweepImage {
                    shape,
                    history: original.history.clone(),
                })
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        poll(&mut cancelled, Stage::Export)?;
        Ok(DetachedSolidSweep {
            solid: copy.into_root(),
            images,
        })
    }
}

fn members(shape: &Shape, kind: ShapeEnum) -> Result<IndexedMapOfShape, Failure> {
    let mut ids = IndexedMapOfShape::default();
    native(
        topexp::map_shapes_of_type(shape, kind, &mut ids),
        Stage::Binding,
    )?;
    Ok(ids)
}
