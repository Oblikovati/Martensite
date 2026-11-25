// SPDX-License-Identifier: GPL-2.0-only
//! Qualification of the single native Section invocation and its actual operands.
use super::{
    Checked, PlanarSectionCause as Cause,
    geometry::{self, RawPcurve},
    poll,
    topology::{self, BoundaryObservation, ObservedFace},
};
use crate::{bop_algo::ProgressCompletion, brep_algo_api::BRepAlgoAPISection};
use martensite_tkbrep::topods::{self, Edge, Face};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    gp::Plane3,
    predicates::roots::metric_bounds::{
        PlanarSectionWitness, SectionLineSource, SectionUvLine, SectionVertexSource,
    },
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
};

pub(super) struct NativeCandidate {
    pub space: SectionLineSource,
    pub vertices: [SectionVertexSource; 2],
    pub pcurves: [Option<SectionUvLine>; 2],
    pub native_fuzzy_length: f64,
}

pub(super) fn run(
    source: &Face,
    observed: &ObservedFace,
    witness: &PlanarSectionWitness,
    work_plane: Plane3,
    progress: &ProgressRange,
) -> Checked<NativeCandidate> {
    poll(progress)?;
    let source = private_face(source, observed, witness, progress)?;
    let tool = BRepBuilderAPIMakeFace::from_plane(work_plane)?.face()?;
    if !geometry::same_chart(
        geometry::face_chart(&tool)?.value,
        geometry::unplaced_chart(work_plane),
    ) {
        return Err(Cause::PrivateCopy);
    }
    poll(progress)?;
    let section = build(&source, &tool, progress)?;
    let edge = section_edge(&section, [&source, &tool])?;
    poll(progress)?;
    capture_candidate(&edge, [&source, &tool], section.fuzzy_value())
}

fn private_face(
    source: &Face,
    observed: &ObservedFace,
    witness: &PlanarSectionWitness,
    progress: &ProgressRange,
) -> Checked<Face> {
    let original = Face::unchecked(source.oriented(Orientation::Forward));
    let mut copy = BRepBuilderAPICopy::with_shape(&original, true, false)?;
    let forward = topods::face(copy.shape()?)?;
    let support = geometry::face_chart(&forward)?;
    if forward.tshape() == source.tshape()
        || support.owner == observed.support.owner
        || !geometry::same_chart(support.value, observed.support.value)
    {
        return Err(Cause::PrivateCopy);
    }
    verify_private_edges(&copy, observed, witness, &forward, progress)?;
    // The generic copy publishes a forward root. Section receives the original face use.
    Ok(Face::unchecked(forward.oriented(source.orientation())))
}

fn verify_private_edges(
    copy: &BRepBuilderAPICopy,
    observed: &ObservedFace,
    witness: &PlanarSectionWitness,
    forward: &Face,
    progress: &ProgressRange,
) -> Checked<()> {
    let members = topology::face_wire_edges(forward)?;
    if members.len() != observed.edges.len() {
        return Err(Cause::PrivateCopy);
    }
    for (index, (original, border)) in observed.edges.iter().zip(&observed.border).enumerate() {
        poll(progress)?;
        let mapped = copy.modified_shape(&original.edge)?;
        let member = members
            .iter()
            .find(|edge| edge.is_same(&mapped))
            .ok_or(Cause::PrivateCopy)?;
        if member.orientation() != original.edge.orientation() {
            return Err(Cause::PrivateCopy);
        }
        let edge = topods::edge(member)?;
        let vertices = topology::directed_vertices(&edge)?;
        for (input, private) in original.vertices.iter().zip(&vertices) {
            if copy.modified_shape(input)?.tshape() != private.tshape() {
                return Err(Cause::PrivateCopy);
            }
        }
        let (private, coefficients) = topology::read_edge(edge, vertices, forward)?;
        if !disjoint_edge(original, &private) {
            return Err(Cause::PrivateCopy);
        }
        witness
            .verify_copy_border(index, border, &coefficients)
            .map_err(|cause| observed.trim_failure(cause))?;
    }
    Ok(())
}

fn disjoint_edge(source: &BoundaryObservation, private: &BoundaryObservation) -> bool {
    source.edge.tshape() != private.edge.tshape()
        && source
            .vertices
            .iter()
            .zip(&private.vertices)
            .all(|(public, private)| public.tshape() != private.tshape())
        && source.space.owner != private.space.owner
        && disjoint_pcurves(source.pcurve.as_ref(), private.pcurve.as_ref())
}

fn disjoint_pcurves(source: Option<&RawPcurve>, private: Option<&RawPcurve>) -> bool {
    match (source, private) {
        (None, _) => true,
        (Some(source), Some(private)) => source.owner != private.owner,
        _ => false,
    }
}

fn build(source: &Face, tool: &Face, progress: &ProgressRange) -> Checked<BRepAlgoAPISection> {
    let mut section = BRepAlgoAPISection::from_shapes(source, tool, None, false)?;
    // The legacy Section constructor inherits the process parallel default.
    // This bounded query chooses its own schedule before building either operand.
    section.set_run_parallel(false);
    section.set_non_destructive(true);
    section.set_use_obb(false);
    section.set_fuzzy_value(0.);
    section.approximation(false);
    section.compute_pcurve_on1(true);
    section.compute_pcurve_on2(true);
    let completion = section.build_with_progress(progress);
    match completion {
        ProgressCompletion::Completed if section.is_done() && !section.has_errors() => Ok(section),
        ProgressCompletion::Completed => Err(Cause::NativeState {
            done: section.is_done(),
            reported_errors: section.has_errors(),
        }),
        ProgressCompletion::Cancelled { .. } => Err(Cause::Cancelled),
        other => Err(Cause::NativeBuild(Box::new(other))),
    }
}

fn section_edge(section: &BRepAlgoAPISection, [source, tool]: [&Face; 2]) -> Checked<Edge> {
    let result = section.shape();
    topology::require_kind(result, ShapeEnum::Compound)?;
    let actual = result.nb_children() as usize;
    if actual != 1 {
        return Err(Cause::NativeResultCount {
            actual,
            expected: 1,
        });
    }
    let mut children = topology::children(result, ShapeEnum::Edge, [1, 1])?;
    let raw = children.pop().ok_or(Cause::NativeResultCount {
        actual: 0,
        expected: 1,
    })?;
    let first = section.has_ancestor_face_on1(&raw)?;
    if !first.is_some_and(|shape| shape.is_same(source)) {
        return Err(Cause::NativeAncestor { operand: 1 });
    }
    let second = section.has_ancestor_face_on2(&raw)?;
    if !second.is_some_and(|shape| shape.is_same(tool)) {
        return Err(Cause::NativeAncestor { operand: 2 });
    }
    topology::reversed(&raw)?;
    Ok(Edge::unchecked(raw.oriented(Orientation::Forward)))
}

fn capture_candidate(
    edge: &Edge,
    [source, tool]: [&Face; 2],
    native_fuzzy_length: f64,
) -> Checked<NativeCandidate> {
    let [start, end] = topology::directed_vertices(edge)?;
    let [source_pcurve, tool_pcurve] = [
        geometry::face_pcurve(edge, source)?,
        geometry::face_pcurve(edge, tool)?,
    ];
    Ok(NativeCandidate {
        space: geometry::space_line(edge)?.value,
        vertices: [
            geometry::vertex_source(&start)?,
            geometry::vertex_source(&end)?,
        ],
        native_fuzzy_length,
        pcurves: [
            source_pcurve.map(|curve| curve.value),
            tool_pcurve.map(|curve| curve.value),
        ],
    })
}
