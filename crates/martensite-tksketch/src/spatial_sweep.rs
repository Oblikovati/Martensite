// SPDX-License-Identifier: GPL-2.0-only
//! Bounded rounded sheets and solids through the native PipeShell builder.
mod admission;
mod failure;
mod geometry;
mod lineage;
mod result;
mod solid;
mod trim_proof;

pub use failure::{SweepFailure, SweepInput, SweepStage};
pub use result::{
    DetachedSweep, SweepEdgeHistory, SweepFaceHistory, SweepHistory, SweepImage, SweepPathPart,
    SweepResult, SweepSupportEvidence,
};

pub use solid::{
    DetachedSolidSweep, SolidSweepHistory, SolidSweepImage, SolidSweepOptions, SolidSweepResult,
    sweep_rounded_solid,
};

use martensite_tkbo::{
    body_check::validate_body,
    profile_extrusion::{AchievedTolerance, Finding, Resolution, ToleranceCap},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape, Shell},
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakePipeShell;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSweep;
use martensite_tksketch3d::wire::{DetachedWire, WireFailure, WireRecipe};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPITransitionMode,
};

/// Native modeling and explicit independent Sweep allowances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SweepOptions {
    /// Requested geometric, angular and stored-tolerance settings.
    pub resolution: Resolution,
    /// Native boundary approximation tolerance.
    pub boundary_tolerance: f64,
    /// Allowed spatial error of the actual trims and terminals against their source spans.
    pub source_trace_limit: f64,
    /// Largest accepted native approximation estimate.
    pub approximation_limit: f64,
}

/// Sweep a centered closed rectangular line section along three orthogonal open legs.
/// Returns a sheet with two boundary loops. Native build and validation may finish before
/// cooperative cancellation is observed. Example: sweep_rounded_shell(path, section, options, || false)?.
pub fn sweep_rounded_shell(
    path: &WireRecipe,
    section: &WireRecipe,
    options: SweepOptions,
    mut cancelled: impl FnMut() -> bool,
) -> Result<SweepResult, SweepFailure> {
    let Prepared {
        admission,
        wires,
        mut pipe,
        shell,
        terminals: [first, last],
    } = prepare(path, section, options, &mut cancelled)?;
    let validation = validate(&shell, options, &mut cancelled)?;
    let pending = lineage::snapshot(
        &mut pipe,
        &shell,
        &wires,
        [&first, &last],
        &admission,
        &mut cancelled,
    )?;
    let (images, mut evidence) = pending.complete(&admission, options, &mut cancelled)?;
    include_wire_trace(&mut evidence, &wires);
    poll(&mut cancelled, SweepStage::Publication)?;
    Ok(SweepResult::new(
        shell,
        options,
        validation,
        images,
        evidence,
        pipe.error_on_surface(),
    ))
}

struct Prepared {
    admission: admission::Admission,
    wires: [DetachedWire; 2],
    pipe: BRepOffsetAPIMakePipeShell,
    shell: Shell,
    terminals: [Shape; 2],
}

fn prepare(
    path: &WireRecipe,
    section: &WireRecipe,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Prepared, SweepFailure> {
    let admission = admission::admit(path, section, options, cancelled)?;
    let wires = [
        materialize(path, SweepInput::Spine, options, cancelled)?,
        materialize(section, SweepInput::Profile, options, cancelled)?,
    ];
    let mut pipe = build_pipe(&wires, admission.stations[0].point, options, cancelled)?;
    let first = native(pipe.first_shape(), SweepStage::Binding)?;
    let last = native(pipe.last_shape(), SweepStage::Binding)?;
    let shell = native(
        topods::shell(native(pipe.shape(), SweepStage::Construction)?),
        SweepStage::Construction,
    )?;
    Ok(Prepared {
        admission,
        wires,
        pipe,
        shell,
        terminals: [first, last],
    })
}

fn include_wire_trace(evidence: &mut SweepSupportEvidence, wires: &[DetachedWire; 2]) {
    for wire in wires {
        evidence.achieved_trace_bound = evidence
            .achieved_trace_bound
            .max(wire.validation.curve_error_bound);
    }
}

fn materialize(
    source: &WireRecipe,
    owner: SweepInput,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<DetachedWire, SweepFailure> {
    poll(cancelled, SweepStage::Materialize)?;
    let wire = source
        .materialize(&mut *cancelled)
        .map_err(|cause| match cause {
            WireFailure::Cancelled(_) => SweepFailure::Cancelled(SweepStage::Materialize),
            source => SweepFailure::Wire { owner, source },
        })?;
    let bound = wire.validation.curve_error_bound;
    if !bound.is_finite() || bound > options.source_trace_limit {
        return Err(SweepFailure::InputTrace {
            owner,
            bound,
            limit: options.source_trace_limit,
        });
    }
    Ok(wire)
}

fn build_pipe(
    [path, profile]: &[DetachedWire; 2],
    start: martensite_tksketch3d::PointId,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BRepOffsetAPIMakePipeShell, SweepFailure> {
    poll(cancelled, SweepStage::Construction)?;
    let start = path
        .points
        .iter()
        .find(|point| point.point == start)
        .ok_or(SweepFailure::Binding("the selected opening spine vertex"))?;
    let mut pipe = native(
        BRepOffsetAPIMakePipeShell::new(&path.wire),
        SweepStage::Construction,
    )?;
    pipe.set_transition_mode(BRepBuilderAPITransitionMode::RoundCorner);
    pipe.set_tolerance(
        options.resolution.sew,
        options.boundary_tolerance,
        options.resolution.angular,
    );
    native(
        pipe.add_at(&profile.wire, &start.vertex, false, false),
        SweepStage::Construction,
    )?;
    let built = pipe.build(&ProgressRange::default());
    poll(cancelled, SweepStage::Construction)?;
    native(built, SweepStage::Construction)?;
    if !pipe.is_done() {
        return Err(SweepFailure::Construction(pipe.get_status()));
    }
    let estimate = pipe.error_on_surface();
    if !estimate.is_finite() || estimate < 0. || estimate > options.approximation_limit {
        return Err(SweepFailure::SurfaceEstimate {
            estimate,
            limit: options.approximation_limit,
        });
    }
    Ok(pipe)
}

#[derive(Debug)]
struct Validation {
    report: martensite_tkbo::body_check::ValidationReport,
    achieved: AchievedTolerance,
    findings: Vec<Finding>,
}

fn validate(
    shell: &Shape,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Validation, SweepFailure> {
    poll(cancelled, SweepStage::Validation)?;
    let mut native_resolution = options.resolution;
    if matches!(native_resolution.cap, ToleranceCap::Report(_)) {
        native_resolution.cap = ToleranceCap::NoLimit;
    }
    let report = validate_body(shell, &native_resolution).map_err(SweepFailure::Resolution)?;
    poll(cancelled, SweepStage::Validation)?;
    if !report.is_valid() {
        return Err(SweepFailure::Validation(Box::new(report)));
    }
    let value = |kind| native(BRepTool::max_tolerance(shell, kind), SweepStage::Validation);
    let achieved = AchievedTolerance {
        face: value(ShapeEnum::Face)?,
        edge: value(ShapeEnum::Edge)?,
        vertex: value(ShapeEnum::Vertex)?,
    };
    let findings = match options.resolution.cap {
        ToleranceCap::Report(limit) if achieved.max() > limit => {
            vec![Finding::ToleranceAboveReportLimit {
                achieved: achieved.max(),
                limit,
            }]
        }
        _ => Vec::new(),
    };
    Ok(Validation {
        report,
        achieved,
        findings,
    })
}

fn native<T>(
    result: Result<T, martensite_tkernel::standard::Failure>,
    stage: SweepStage,
) -> Result<T, SweepFailure> {
    result.map_err(|source| SweepFailure::Native {
        stage,
        source: Box::new(source),
    })
}

fn poll(cancelled: &mut impl FnMut() -> bool, stage: SweepStage) -> Result<(), SweepFailure> {
    if cancelled() {
        return Err(SweepFailure::Cancelled(stage));
    }
    Ok(())
}
