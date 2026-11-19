// SPDX-License-Identifier: GPL-2.0-only
//! Plane graph region proof. Generalizes the loop proof from
//! disjoint authored loops to any plane graph of Bézier sources joined at caller vertex
//! ids: loops may touch at a vertex and adjacent faces may share edges.
//!
//! 1. Embedding: the complete census pair matrix has exactly the vertex incidences as
//!    contacts, at exact native endpoints (`joins::certify_contacts`). Sources are injective
//!    and meet only at shared vertices, so the sources form an embedded plane graph.
//! 2. Rotation: the cyclic order of the darts leaving each vertex, from exact orientation
//!    signs of their first distinct control legs. Equal directions at a vertex of degree
//!    three or more are refused (`TangentBranches`).
//! 3. Faces: the orbits of the rotation system; per component exactly one orbit has a
//!    negative certified Green integral (the outer walk), the others are bounded faces, and
//!    their number satisfies Euler's relation. A bounded face that revisits a vertex is
//!    refused (`PinchedFace`).
//! 4. Nesting: components are disjoint, so a point of one decides which bounded faces of
//!    another contain it; containers must form a strict chain. A component's outer walk is
//!    a hole of its innermost container and must be simple (`PinchedHole`).
//! 5. Area: each face's certified area, outer face minus holes.
mod components;
mod darts;
mod orientation;
mod types;
pub(crate) use types::*;

mod area;
use super::binding::bind_segments;
use super::measure::{FaceUses, certified_face_area};
use super::program_trace::ProgramTrace;
use super::types::*;
use martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions;

/// Certify every bounded face of the plane graph; `inspect_plane_graph(&request, || false)`.
pub(crate) fn inspect_plane_graph(
    request: &GraphRegionRequest<'_, '_>,
    cancelled: impl FnMut() -> bool,
) -> LoopRegionCheck<GraphRegionCertificate> {
    inspect_plane_graph_with_access(
        request,
        cancelled,
        crate::sketch_region_certification::SourceAccess::Live,
    )
}
pub(crate) fn inspect_plane_graph_with_access(
    request: &GraphRegionRequest<'_, '_>,
    cancelled: impl FnMut() -> bool,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> LoopRegionCheck<GraphRegionCertificate> {
    inspect_graph_core(request, cancelled, None, &mut None, access)
}
pub(crate) fn inspect_plane_graph_program(
    request: &GraphRegionRequest<'_, '_>,
    cancelled: impl FnMut() -> bool,
    options: IntegralProgramOptions,
) -> Result<GraphRegionCertificate, Box<ProgramGraphFailure>> {
    inspect_plane_graph_program_with_access(
        request,
        cancelled,
        options,
        crate::sketch_region_certification::SourceAccess::Live,
    )
}
pub(crate) fn inspect_plane_graph_program_with_access(
    request: &GraphRegionRequest<'_, '_>,
    cancelled: impl FnMut() -> bool,
    options: IntegralProgramOptions,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> Result<GraphRegionCertificate, Box<ProgramGraphFailure>> {
    let mut program = None;
    inspect_graph_core(request, cancelled, Some(options), &mut program, access).map_err(|failure| {
        Box::new(ProgramGraphFailure {
            failure,
            options,
            consumed: program.as_ref().map_or(0, ProgramTrace::consumed),
        })
    })
}
fn inspect_graph_core(
    request: &GraphRegionRequest<'_, '_>,
    mut cancelled: impl FnMut() -> bool,
    options: Option<IntegralProgramOptions>,
    program: &mut Option<ProgramTrace>,
    access: crate::sketch_region_certification::SourceAccess<'_>,
) -> LoopRegionCheck<GraphRegionCertificate> {
    poll(&mut cancelled, LoopRegionStage::Admission)?;
    let binding = bind_segments(request.bridge, request.sources, request.options.version)?;
    darts::require_ends(request, &binding)?;
    poll(&mut cancelled, LoopRegionStage::Joins)?;
    let expected = darts::incidence_contacts(request, &binding)?;
    let (joins, carrier_pairs, pair_domains) = super::joins::certify_contacts_with_access(
        &request.bridge.census,
        &binding,
        expected,
        access,
    )?;
    *program = options.map(|options| ProgramTrace::new(options, &binding));
    finish_graph(
        request,
        &binding,
        (joins.len(), carrier_pairs, pair_domains),
        &mut cancelled,
        program,
    )
}
fn finish_graph(
    request: &GraphRegionRequest<'_, '_>,
    binding: &super::binding::SegmentBinding<'_>,
    counts: (usize, usize, usize),
    cancelled: &mut impl FnMut() -> bool,
    program: &mut Option<ProgramTrace>,
) -> LoopRegionCheck<GraphRegionCertificate> {
    let (graph, faces) = graph_faces(request, binding, cancelled, program.as_mut())?;
    poll(cancelled, LoopRegionStage::Nesting)?;
    let components = components::nesting(request, binding, &graph, &faces, cancelled)?;
    poll(cancelled, LoopRegionStage::Regions)?;
    let mut certificate = graph_certificate(request, &faces, components, counts);
    certify_faces(
        request,
        binding,
        &faces,
        &mut certificate,
        cancelled,
        program,
    )?;
    certificate.programs = program
        .take()
        .map(|trace| std::sync::Arc::new(trace.finish()));
    Ok(certificate)
}

fn graph_certificate(
    request: &GraphRegionRequest<'_, '_>,
    faces: &components::Faces,
    components: Vec<GraphComponentProof>,
    counts: (usize, usize, usize),
) -> GraphRegionCertificate {
    let (contacts, carrier_pairs, pair_domains) = counts;
    GraphRegionCertificate {
        programs: None,
        faces: Vec::with_capacity(faces.bounded.len()),
        components,
        contacts,
        carrier_pairs,
        pair_domains,
        work: crate::sketch_region_certification::CensusWorkReceipt::capture(
            &request.bridge.census,
        ),
    }
}

/// Steps 2-3: rotation, orbits, components, certified orbit signs and the face split.
fn graph_faces(
    request: &GraphRegionRequest<'_, '_>,
    binding: &super::binding::SegmentBinding<'_>,
    cancelled: &mut impl FnMut() -> bool,
    program: Option<&mut ProgramTrace>,
) -> LoopRegionCheck<(components::Graph, components::Faces)> {
    poll(cancelled, LoopRegionStage::Faces)?;
    let rotation = darts::rotation(request, binding)?;
    let graph = components::components(request, darts::orbits(request, &rotation));
    let signs = if let Some(program) = program {
        orientation::signs_with_program(request, binding, &graph, cancelled, Some(program))?
    } else {
        orientation::signs(request, binding, &graph, cancelled)?
    };
    let faces = components::classify(request, &graph, &signs)?;
    Ok((graph, faces))
}

fn face_proof(
    request: &GraphRegionRequest<'_, '_>,
    binding: &super::binding::SegmentBinding<'_>,
    certificate: &GraphRegionCertificate,
    index: usize,
    face: &components::BoundedFace,
    cancelled: &mut impl FnMut() -> bool,
    program: Option<&mut ProgramTrace>,
) -> LoopRegionCheck<GraphFaceProof> {
    let holes = contained_components(certificate, index);
    let hole_uses: Vec<&[AuthoredLoopUse]> = holes
        .iter()
        .map(|k| certificate.components[*k].outer.as_slice())
        .collect();
    let outer: Vec<AuthoredLoopUse> = face.orbit.iter().map(|d| d.authored(binding)).collect();
    let uses = FaceUses {
        name: index as u64 + 1,
        outer: &outer,
        holes: &hole_uses,
    };
    let area = selected_area(request, binding, uses, program, cancelled)?;
    Ok(GraphFaceProof {
        uses: outer,
        component: face.component,
        holes,
        area,
    })
}

fn poll(cancelled: &mut impl FnMut() -> bool, stage: LoopRegionStage) -> LoopRegionCheck<()> {
    if cancelled() {
        return refuse(stage, LoopRegionCause::Cancelled);
    }
    Ok(())
}

fn certify_faces(
    request: &GraphRegionRequest<'_, '_>,
    binding: &super::binding::SegmentBinding<'_>,
    faces: &components::Faces,
    certificate: &mut GraphRegionCertificate,
    cancelled: &mut impl FnMut() -> bool,
    program: &mut Option<ProgramTrace>,
) -> LoopRegionCheck<()> {
    for (index, face) in faces.bounded.iter().enumerate() {
        let proof = face_proof(
            request,
            binding,
            certificate,
            index,
            face,
            cancelled,
            program.as_mut(),
        )?;
        certificate.faces.push(proof);
    }
    Ok(())
}

fn selected_area(
    request: &GraphRegionRequest<'_, '_>,
    binding: &super::binding::SegmentBinding<'_>,
    uses: FaceUses<'_>,
    program: Option<&mut ProgramTrace>,
    cancelled: &mut impl FnMut() -> bool,
) -> LoopRegionCheck<area::GraphArea> {
    if let Some(program) = program {
        return super::measure::certified_program_face_area(binding, uses, program, cancelled)
            .map(area::GraphArea::Program);
    }
    certified_face_area(request.options.area, binding, uses, cancelled).map(area::GraphArea::Legacy)
}

fn contained_components(certificate: &GraphRegionCertificate, index: usize) -> Vec<usize> {
    (0..certificate.components.len())
        .filter(|k| certificate.components[*k].container == Some(index))
        .collect()
}
