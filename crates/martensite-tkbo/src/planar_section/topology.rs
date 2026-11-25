// SPDX-License-Identifier: GPL-2.0-only
//! Bind the original face and wire occurrences before trusting any geometric order.
use super::{
    Checked, PlanarSectionCause as Cause, PlanarSectionError, PlanarSectionFace,
    SectionBoundaryBinding, SectionEndpointOrigin, SectionTrimUse, geometry, poll,
};
use geometry::{RawChart, RawLine, RawPcurve};
use martensite_tkbrep::{
    brep::face_input_capture::{
        CapturedFaceCell, FaceCaptureBehavior, FaceCaptureLimits, capture_face_input_with_behavior,
    },
    topods::{self, Edge, Face, Shape, ShapeIterator, Vertex},
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::predicates::roots::metric_bounds::{
    PLANAR_SECTION_MAX_EDGES, PlanarSectionAnchor, SectionBorder,
};

pub(super) struct ObservedFace {
    pub uses: Vec<SectionTrimUse>,
    pub border: Vec<SectionBorder>,
    pub reversed: bool,
    pub captured_cells: usize,
    pub support: RawChart,
    pub edges: Vec<BoundaryObservation>,
}

pub(super) struct BoundaryObservation {
    pub edge: Edge,
    pub vertices: [Vertex; 2],
    pub space: RawLine,
    pub pcurve: Option<RawPcurve>,
}

struct LocatedUse {
    edge: Edge,
    vertices: [Vertex; 2],
    binding: SectionTrimUse,
}

pub(super) fn observe(
    source: PlanarSectionFace<'_>,
    progress: &ProgressRange,
) -> Checked<ObservedFace> {
    require_kind(source.face, ShapeEnum::Face)?;
    let reversed = reversed(source.face)?;
    let face = Face::unchecked(source.face.oriented(Orientation::Forward));
    let members = source_members(&face, source.boundary, progress)?;
    let captured = capture_face_input_with_behavior(
        &face,
        FaceCaptureLimits::bounded(),
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
        || progress.user_break(),
    )?;
    require_source_tolerances(captured.cells(), progress)?;
    let support = geometry::face_chart(&face)?;
    read_original(
        support,
        &face,
        members,
        reversed,
        captured.cells().len(),
        progress,
    )
}

fn require_source_tolerances(
    cells: &[(usize, CapturedFaceCell)],
    progress: &ProgressRange,
) -> Checked<()> {
    for (owner, cell) in cells {
        poll(progress)?;
        match cell {
            CapturedFaceCell::Face { tolerance, .. }
            | CapturedFaceCell::Edge { tolerance, .. }
            | CapturedFaceCell::Vertex { tolerance, .. }
                if *tolerance < 0. =>
            {
                return Err(Cause::SourceTolerance {
                    owner: *owner,
                    actual: *tolerance,
                });
            }
            _ => {}
        }
    }
    Ok(())
}

fn read_original(
    support: RawChart,
    face: &Face,
    members: Vec<LocatedUse>,
    reversed: bool,
    captured_cells: usize,
    progress: &ProgressRange,
) -> Checked<ObservedFace> {
    let mut uses = Vec::with_capacity(members.len());
    let mut border = Vec::with_capacity(members.len());
    let mut edges = Vec::with_capacity(members.len());
    for LocatedUse {
        edge,
        vertices,
        binding,
    } in members
    {
        poll(progress)?;
        let (observed, input) =
            read_edge(edge, vertices, face).map_err(|cause| Cause::BoundarySource {
                boundary: binding,
                cause: Box::new(cause),
            })?;
        uses.push(binding);
        border.push(input);
        edges.push(observed);
    }
    Ok(ObservedFace {
        uses,
        border,
        reversed,
        captured_cells,
        support,
        edges,
    })
}

pub(super) fn read_edge(
    edge: Edge,
    vertices: [Vertex; 2],
    face: &Face,
) -> Checked<(BoundaryObservation, SectionBorder)> {
    let space = geometry::space_line(&edge)?;
    let pcurve = geometry::face_pcurve(&edge, face)?;
    let input = SectionBorder {
        space: space.value,
        vertices: [
            geometry::vertex_source(&vertices[0])?,
            geometry::vertex_source(&vertices[1])?,
        ],
        reversed: reversed(&edge)?,
        pcurve: pcurve.as_ref().map(|stored| stored.value),
    };
    Ok((
        BoundaryObservation {
            edge,
            vertices,
            space,
            pcurve,
        },
        input,
    ))
}

fn source_members(
    face: &Face,
    input: &[SectionBoundaryBinding<'_>],
    progress: &ProgressRange,
) -> Checked<Vec<LocatedUse>> {
    let edges = face_wire_edges(face)?;
    if input.len() != edges.len() {
        return Err(Cause::BoundaryBindingCount {
            supplied: input.len(),
            expected: edges.len(),
        });
    }
    for binding in input {
        poll(progress)?;
        if !edges.iter().any(|edge| edge.is_same(binding.edge)) {
            return Err(Cause::BoundaryBinding(binding.id));
        }
    }
    let mut pending = Vec::with_capacity(edges.len());
    for edge in edges {
        poll(progress)?;
        pending.push(bind_use(edge, input, &pending)?);
    }
    directed_cycle(pending, progress)
}

pub(super) fn face_wire_edges(face: &Face) -> Checked<Vec<Shape>> {
    let mut wires = children(face, ShapeEnum::Wire, [1, 1])?;
    let wire = wires.pop().ok_or(Cause::OpenOrBranchedBoundary)?;
    reversed(&wire)?;
    children(&wire, ShapeEnum::Edge, [3, PLANAR_SECTION_MAX_EDGES])
}

fn bind_use(
    edge: Shape,
    input: &[SectionBoundaryBinding<'_>],
    pending: &[LocatedUse],
) -> Checked<LocatedUse> {
    let mut matches = input.iter().filter(|selected| edge.is_same(selected.edge));
    let selected = matches.next().ok_or(Cause::OpenOrBranchedBoundary)?;
    if matches.next().is_some() || pending.iter().any(|item| item.binding.id == selected.id) {
        return Err(Cause::BoundaryBinding(selected.id));
    }
    reversed(&edge)?;
    let orientation = edge.orientation();
    let edge = Edge::unchecked(edge);
    let vertices = directed_vertices(&edge)?;
    Ok(LocatedUse {
        edge,
        vertices,
        binding: SectionTrimUse {
            id: selected.id,
            orientation,
        },
    })
}

fn directed_cycle(
    mut pending: Vec<LocatedUse>,
    progress: &ProgressRange,
) -> Checked<Vec<LocatedUse>> {
    pending.sort_unstable_by_key(|edge| std::cmp::Reverse(edge.binding.id));
    let first = pending.pop().ok_or(Cause::OpenOrBranchedBoundary)?;
    let mut cycle = Vec::with_capacity(pending.len() + 1);
    cycle.push(first);
    while !pending.is_empty() {
        poll(progress)?;
        let [_, end] = &cycle.last().ok_or(Cause::OpenOrBranchedBoundary)?.vertices;
        let next = unique_successor(end, &pending).ok_or(Cause::OpenOrBranchedBoundary)?;
        cycle.push(pending.swap_remove(next));
    }
    let first = &cycle[0].vertices[0];
    let last = &cycle[cycle.len() - 1].vertices[1];
    if !last.is_same(first) {
        return Err(Cause::OpenOrBranchedBoundary);
    }
    Ok(cycle)
}

fn unique_successor(vertex: &Vertex, candidates: &[LocatedUse]) -> Option<usize> {
    let mut matches = candidates
        .iter()
        .enumerate()
        .filter(|(_, item)| vertex.is_same(&item.vertices[0]));
    let (index, _) = matches.next()?;
    if matches.next().is_some() {
        None
    } else {
        Some(index)
    }
}

pub(super) fn directed_vertices(edge: &Edge) -> Checked<[Vertex; 2]> {
    let children = children(edge, ShapeEnum::Vertex, [2, 2])?;
    let [a, b]: [Shape; 2] = children
        .try_into()
        .map_err(|_| Cause::OpenOrBranchedBoundary)?;
    let [a_reversed, b_reversed] = [reversed(&a)?, reversed(&b)?];
    if a_reversed == b_reversed || a.is_same(&b) {
        return Err(Cause::OpenOrBranchedBoundary);
    }
    let (a, b) = if a_reversed { (b, a) } else { (a, b) };
    Ok([topods::vertex(&a)?, topods::vertex(&b)?])
}

pub(super) fn children(shape: &Shape, kind: ShapeEnum, allowed: [usize; 2]) -> Checked<Vec<Shape>> {
    let actual = shape.nb_children() as usize;
    if !(allowed[0]..=allowed[1]).contains(&actual) {
        return Err(Cause::TopologyCount {
            child: kind,
            actual,
            allowed,
        });
    }
    let mut iterator = ShapeIterator::new(shape, true, true);
    let mut result = Vec::with_capacity(actual);
    while iterator.more() {
        let item = iterator.value()?;
        require_kind(item, kind)?;
        result.push(item.clone());
        iterator.next();
    }
    Ok(result)
}

pub(super) fn require_kind(shape: &Shape, expected: ShapeEnum) -> Checked<()> {
    let actual = shape.shape_type()?;
    if actual != expected {
        return Err(Cause::ShapeKind { actual, expected });
    }
    Ok(())
}

pub(super) fn reversed(shape: &Shape) -> Checked<bool> {
    match shape.orientation() {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        other => Err(Cause::Orientation(other)),
    }
}

impl ObservedFace {
    pub fn trim_failure(&self, cause: PlanarSectionError) -> Cause {
        let PlanarSectionError::InconsistentBorder(index) = cause else {
            return cause.into();
        };
        Cause::BoundarySource {
            boundary: self.uses[index],
            cause: Box::new(cause.into()),
        }
    }

    pub fn origin(&self, anchor: PlanarSectionAnchor) -> SectionEndpointOrigin {
        match anchor {
            PlanarSectionAnchor::Edge { index, parameter } => SectionEndpointOrigin::Edge {
                boundary: self.uses[index],
                parameter,
            },
            PlanarSectionAnchor::Vertex(index) => self.vertex_origin(index),
        }
    }

    pub fn vertex_origin(&self, index: usize) -> SectionEndpointOrigin {
        let incoming = (index + self.uses.len() - 1) % self.uses.len();
        SectionEndpointOrigin::Vertex {
            incoming: self.uses[incoming],
            outgoing: self.uses[index],
        }
    }
}
