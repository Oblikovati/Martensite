// SPDX-License-Identifier: GPL-2.0-only
//! Private prism and original-face Section ancestry.
use super::{
    Checked, CylinderProjectionBranch, ProjectionEndpointLineage, SurfaceProjectionCause as Cause,
    SurfaceProjectionEdge, SurfaceProjectionGeometry, SurfaceProjectionOptions,
    SurfaceProjectionScope as Scope, poll,
    storage::{self, SourceLine},
    target::TargetRead,
};
use crate::{bop_algo::ProgressCompletion, brep_algo_api::BRepAlgoAPISection};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::predicates::roots::metric_bounds::cylindrical_projection::{
    AxialProjectionWitness, NativeProjectionCurve, NativeProjectionEdge, ProjectionEdgeCertificate,
    ProjectionSweep, ProjectionTrimEndpoint,
};

use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

mod operands;
mod sweep;

pub(super) fn publish(
    source: &Edge,
    original: SourceLine,
    target: &Face,
    target_read: TargetRead,
    witness: &AxialProjectionWitness,
    options: SurfaceProjectionOptions,
    progress: &ProgressRange,
) -> Checked<SurfaceProjectionGeometry> {
    let sweep = witness.sweep()?;
    let private_source = operands::copy_source(source, &original)?;
    let (private_target, copied_target) =
        operands::copy_target(target, &target_read, options.limits.spatial, progress)?;
    let tool = sweep::build(&private_source, sweep, witness, progress)?;
    let section = build_section(&private_target, &tool, progress)?;
    let (candidates, common_seams) = candidates(
        &section,
        &private_target,
        &tool,
        &copied_target.seam_edge,
        progress,
    )?;
    let certificates = witness
        .certify_native_edges(sweep, &candidates, options.limits, || progress.user_break())?;
    let checked_edges = certificates.len();
    let edges = publish_edges(
        candidates,
        common_seams,
        certificates,
        &target_read,
        progress,
    )?;
    Ok(SurfaceProjectionGeometry {
        source_orientation: original.orientation,
        target_orientation: target_read.orientation,
        sweep,
        branches: witness.branch_plans()?,
        edges,
        checked_edges,
        target_trim_error: target_read.boundary_error,
        native_fuzzy_length: section.fuzzy_value(),
    })
}

fn build_section(
    target: &Face,
    tool: &Face,
    progress: &ProgressRange,
) -> Checked<BRepAlgoAPISection> {
    poll(progress)?;
    let mut section = BRepAlgoAPISection::from_shapes(target, tool, None, false)?;
    section.set_run_parallel(false);
    section.set_non_destructive(true);
    section.set_use_obb(false);
    section.set_fuzzy_value(0.);
    section.approximation(false);
    section.compute_pcurve_on1(true);
    section.compute_pcurve_on2(true);
    let status = section.build_with_progress(progress);
    match status {
        ProgressCompletion::Completed if section.is_done() && !section.has_errors() => Ok(section),
        ProgressCompletion::Completed => Err(Cause::SectionState {
            done: section.is_done(),
            reported_errors: section.has_errors(),
        }),
        ProgressCompletion::Cancelled { .. } => Err(Cause::Cancelled),
        other => Err(Cause::Section(Box::new(other))),
    }
}

fn candidates(
    section: &BRepAlgoAPISection,
    target: &Face,
    tool: &Face,
    seam: &Edge,
    progress: &ProgressRange,
) -> Checked<(Vec<NativeProjectionEdge>, Vec<bool>)> {
    let result = section.shape();
    storage::kind(result, ShapeEnum::Compound)?;
    let children = storage::direct(result, ShapeEnum::Edge, [1, 4])?;
    let mut candidates = Vec::with_capacity(children.len());
    let mut common_seam = Vec::with_capacity(children.len());
    for edge in children {
        poll(progress)?;
        require_ancestors(section, &edge, [target, tool])?;
        common_seam.push(
            section
                .builder()
                .and_then(|builder| builder.origins().seek(&edge))
                .is_some_and(|origins| origins.iter().any(|origin| origin.is_same(seam))),
        );
        candidates.push(read_edge(&topods::edge(&edge)?, target, tool)?);
    }
    Ok((candidates, common_seam))
}

fn require_ancestors(
    section: &BRepAlgoAPISection,
    edge: &Shape,
    [target, tool]: [&Face; 2],
) -> Checked<()> {
    if !section
        .has_ancestor_face_on1(edge)?
        .is_some_and(|shape| shape.is_same(target))
    {
        return Err(Scope::Ancestry(1).into());
    }
    if !section
        .has_ancestor_face_on2(edge)?
        .is_some_and(|shape| shape.is_same(tool))
    {
        return Err(Scope::Ancestry(2).into());
    }
    Ok(())
}

fn read_edge(edge: &Edge, target: &Face, tool: &Face) -> Checked<NativeProjectionEdge> {
    storage::reversed(edge.orientation())?;
    let forward = Edge::unchecked(edge.oriented(Orientation::Forward));
    storage::require_spatial(&forward)?;
    let space = BRepTool::curve_with_location(&forward)?;
    let range = [space.first, space.last];
    let (target, second_target) = chart_curves(&forward, target, range)?;
    let (prism, second_prism) = chart_curves(&forward, tool, range)?;
    if second_prism.is_some() {
        return Err(Scope::Representation.into());
    }
    let witnesses = storage::vertices(&forward)?;
    let vertices = storage::vertex_coefficients(&witnesses)?;
    Ok(NativeProjectionEdge {
        range,
        spatial: storage::curve3d(&space.curve, range)?,
        placement: storage::placement(&space.location)?,
        target,
        second_target,
        prism,
        vertices,
    })
}

fn chart_curves(
    edge: &Edge,
    face: &Face,
    range: [f64; 2],
) -> Checked<(NativeProjectionCurve<2>, Option<NativeProjectionCurve<2>>)> {
    let (stored, seam) = storage::pcurve(edge, face)?;
    if [stored.first, stored.last] != range {
        return Err(Scope::Representation.into());
    }
    let primary = storage::curve2d(&stored.pcurve, range)?;
    if !seam {
        return Ok((primary, None));
    }
    let reversed = Edge::unchecked(edge.oriented(Orientation::Reversed));
    let (second, _) = storage::pcurve(&reversed, face)?;
    if [second.first, second.last] != range {
        return Err(Scope::Representation.into());
    }
    Ok((primary, Some(storage::curve2d(&second.pcurve, range)?)))
}

fn publish_edges(
    candidates: Vec<NativeProjectionEdge>,
    seam_ancestry: Vec<bool>,
    certificates: Vec<(usize, ProjectionEdgeCertificate)>,
    target: &TargetRead,
    progress: &ProgressRange,
) -> Checked<Vec<SurfaceProjectionEdge>> {
    let mut shapes: Vec<_> = candidates.into_iter().map(Some).collect();
    let mut result = Vec::new();
    for (index, certificate) in certificates {
        poll(progress)?;
        let common = seam_ancestry[index];
        if (certificate.branch == CylinderProjectionBranch::PositiveU) != common {
            return Err(Scope::Ancestry(1).into());
        }
        let native = shapes[index].take().ok_or(Scope::Topology)?;
        if !certificate.selected.is_empty() {
            let [a, b] = certificate.origins;
            let endpoints = [lineage(a, target)?, lineage(b, target)?];
            result.push(SurfaceProjectionEdge {
                native,
                certificate,
                seam: common.then_some(target.seam),
                endpoints,
            });
        }
    }
    poll(progress)?;
    Ok(result)
}

fn lineage(
    origin: ProjectionTrimEndpoint,
    target: &TargetRead,
) -> Checked<ProjectionEndpointLineage> {
    match origin {
        ProjectionTrimEndpoint::SourceFirst => Ok(ProjectionEndpointLineage::SourceFirst),
        ProjectionTrimEndpoint::SourceLast => Ok(ProjectionEndpointLineage::SourceLast),
        ProjectionTrimEndpoint::HoleLower | ProjectionTrimEndpoint::HoleUpper => {
            let rims = target.hole_rims.ok_or(Scope::Topology)?;
            let index = usize::from(origin == ProjectionTrimEndpoint::HoleUpper);
            Ok(ProjectionEndpointLineage::Hole(rims[index]))
        }
    }
}
