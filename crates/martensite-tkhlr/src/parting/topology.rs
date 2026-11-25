// SPDX-License-Identifier: GPL-2.0-only
//! Resolve caller IDs against accumulated native topology, never against query output order.
use super::{
    PartingBindingProblem as Binding, PartingEdgeBinding, PartingEdgeId, PartingEntity,
    PartingFaceId, PartingFailureKind, PartingOptions, PartingSource,
    PartingTopologyProblem as Topology,
    failure::{Checked, native},
    observe_stop,
};
use martensite_tkbrep::topods::{self, Face, Shape, ShapeIterator};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::collections::{BTreeMap, HashMap, HashSet};

pub(super) struct SourceFace {
    pub id: PartingFaceId,
    pub face: Face,
    pub shell: usize,
}

#[derive(Clone, Copy)]
pub(super) struct EdgeIncidence {
    pub face: usize,
    pub orientation: Orientation,
}

pub(super) struct BoundEdge<'a> {
    pub binding: PartingEdgeBinding<'a>,
    pub uses: [EdgeIncidence; 2],
}

pub(super) struct SourceTopology<'a> {
    pub faces: Vec<SourceFace>,
    pub shells: Vec<Vec<usize>>,
    pub edges: Vec<BoundEdge<'a>>,
}

pub(super) fn capture_topology<'a>(
    source: PartingSource<'a>,
    options: PartingOptions<'_>,
) -> Checked<SourceTopology<'a>> {
    let (faces, shells) = bind_faces(source, options)?;
    let edges = bind_edges(source, options, &faces)?;
    check_connectivity(&shells, &edges, faces.len(), options)?;
    check_exclusions(source, options)?;
    Ok(SourceTopology {
        faces,
        shells,
        edges,
    })
}

fn bind_faces(
    source: PartingSource<'_>,
    options: PartingOptions<'_>,
) -> Checked<(Vec<SourceFace>, Vec<Vec<usize>>)> {
    let solid = PartingEntity::Solid;
    if source.solid.is_null() || source.solid.nb_children() == 0 {
        return Err(topology(solid, Topology::ChildCount { actual: 0 }));
    }
    require_sense(source.solid.orientation(), solid)?;
    let mut bindings = face_bindings(source, options)?;
    let mut faces = Vec::new();
    let mut shells = Vec::new();
    for shell in ShapeIterator::new(source.solid, true, true) {
        observe_stop(options)?;
        require_kind(&shell, ShapeEnum::Shell, solid)?;
        shells.push(bind_shell(
            &shell,
            shells.len(),
            &mut bindings,
            &mut faces,
            options,
        )?);
    }
    if !bindings.is_empty() {
        return Err(binding_count(
            ShapeEnum::Face,
            source.faces.len(),
            faces.len(),
        ));
    }
    Ok((faces, shells))
}

fn face_bindings(
    source: PartingSource<'_>,
    options: PartingOptions<'_>,
) -> Checked<HashMap<Shape, PartingFaceId>> {
    let mut bindings = HashMap::with_capacity(source.faces.len());
    let mut ids = HashSet::new();
    let mut located = HashSet::new();
    for binding in source.faces {
        observe_stop(options)?;
        let entity = PartingEntity::Face(binding.id);
        check_binding_sense(binding.occurrence.orientation(), entity)?;
        if !ids.insert(binding.id) {
            return Err(invalid_binding(entity, Binding::DuplicateId));
        }
        if !located.insert(key(binding.occurrence)) {
            return Err(invalid_binding(entity, Binding::DuplicateOccurrence));
        }
        bindings.insert(binding.occurrence.to_shape(), binding.id);
    }
    Ok(bindings)
}

fn bind_shell(
    shell: &Shape,
    shell_index: usize,
    bindings: &mut HashMap<Shape, PartingFaceId>,
    faces: &mut Vec<SourceFace>,
    options: PartingOptions<'_>,
) -> Checked<Vec<usize>> {
    let solid = PartingEntity::Solid;
    require_sense(shell.orientation(), solid)?;
    let mut members = Vec::new();
    for native_face in ShapeIterator::new(shell, true, true) {
        observe_stop(options)?;
        require_kind(&native_face, ShapeEnum::Face, solid)?;
        let id = take_face_binding(bindings, &native_face, faces)?;
        let face = topods::face(&native_face).map_err(native(PartingEntity::Face(id)))?;
        members.push(faces.len());
        faces.push(SourceFace {
            id,
            face,
            shell: shell_index,
        });
    }
    if members.is_empty() {
        return Err(topology(solid, Topology::ShellConnectivity));
    }
    Ok(members)
}

fn take_face_binding(
    bindings: &mut HashMap<Shape, PartingFaceId>,
    native_face: &Shape,
    prior: &[SourceFace],
) -> Checked<PartingFaceId> {
    if let Some(bound) = bindings.remove(native_face) {
        return Ok(bound);
    }
    if let Some(repeated) = prior.iter().find(|face| face.face.is_same(native_face)) {
        return Err(topology(
            PartingEntity::Face(repeated.id),
            Topology::RepeatedFace,
        ));
    }
    let supplied = bindings
        .iter()
        .filter(|(face, _)| face.is_same(native_face))
        .map(|(_, id)| *id)
        .min();
    Err(invalid_binding(
        supplied.map_or(PartingEntity::Solid, PartingEntity::Face),
        Binding::ForeignOccurrence,
    ))
}

fn bind_edges<'a>(
    source: PartingSource<'a>,
    options: PartingOptions<'_>,
    faces: &[SourceFace],
) -> Checked<Vec<BoundEdge<'a>>> {
    let (by_occurrence, mut edges) = edge_bindings(source, options)?;
    for (face_index, face) in faces.iter().enumerate() {
        add_face_uses(face_index, face, &by_occurrence, &mut edges, options)?;
    }
    edges
        .into_values()
        .map(|(binding, uses)| checked_edge(binding, uses, faces))
        .collect()
}

type EdgeBindings<'a> = BTreeMap<PartingEdgeId, (PartingEdgeBinding<'a>, Vec<EdgeIncidence>)>;

fn edge_bindings<'a>(
    source: PartingSource<'a>,
    options: PartingOptions<'_>,
) -> Checked<(HashMap<Shape, PartingEdgeId>, EdgeBindings<'a>)> {
    let mut by_occurrence = HashMap::with_capacity(source.edges.len());
    let mut by_id = BTreeMap::new();
    for binding in source.edges {
        observe_stop(options)?;
        let entity = PartingEntity::Edge(binding.id);
        check_binding_sense(binding.occurrence.orientation(), entity)?;
        if by_id.insert(binding.id, (*binding, Vec::new())).is_some() {
            return Err(invalid_binding(entity, Binding::DuplicateId));
        }
        if by_occurrence
            .insert(key(binding.occurrence), binding.id)
            .is_some()
        {
            return Err(invalid_binding(entity, Binding::DuplicateOccurrence));
        }
    }
    Ok((by_occurrence, by_id))
}

fn add_face_uses(
    index: usize,
    source_face: &SourceFace,
    ids: &HashMap<Shape, PartingEdgeId>,
    edges: &mut EdgeBindings<'_>,
    options: PartingOptions<'_>,
) -> Checked<()> {
    let entity = PartingEntity::Face(source_face.id);
    if source_face.face.nb_children() == 0 {
        return Err(topology(entity, Topology::ChildCount { actual: 0 }));
    }
    for wire in ShapeIterator::new(&source_face.face, true, true) {
        observe_stop(options)?;
        require_kind(&wire, ShapeEnum::Wire, entity)?;
        require_sense(wire.orientation(), entity)?;
        add_wire_uses(&wire, index, entity, ids, edges, options)?;
    }
    Ok(())
}

fn add_wire_uses(
    wire: &Shape,
    face: usize,
    entity: PartingEntity,
    ids: &HashMap<Shape, PartingEdgeId>,
    edges: &mut EdgeBindings<'_>,
    options: PartingOptions<'_>,
) -> Checked<()> {
    if wire.nb_children() == 0 {
        return Err(topology(entity, Topology::ChildCount { actual: 0 }));
    }
    for edge in ShapeIterator::new(wire, true, true) {
        observe_stop(options)?;
        require_kind(&edge, ShapeEnum::Edge, entity)?;
        let id = ids
            .get(&key(&edge))
            .ok_or_else(|| invalid_binding(entity, Binding::ForeignOccurrence))?;
        let entity = PartingEntity::Edge(*id);
        let orientation = edge.orientation();
        require_sense(orientation, entity)?;
        let (_, uses) = edges
            .get_mut(id)
            .ok_or_else(|| invalid_binding(entity, Binding::ForeignOccurrence))?;
        uses.push(EdgeIncidence { face, orientation });
    }
    Ok(())
}

fn checked_edge<'a>(
    binding: PartingEdgeBinding<'a>,
    uses: Vec<EdgeIncidence>,
    faces: &[SourceFace],
) -> Checked<BoundEdge<'a>> {
    let entity = PartingEntity::Edge(binding.id);
    let [first, second] = uses.as_slice() else {
        return Err(topology(entity, Topology::EdgeUses(uses.len())));
    };
    if first.orientation == second.orientation
        || faces[first.face].shell != faces[second.face].shell
    {
        return Err(topology(entity, Topology::EdgeIncidence));
    }
    let mut uses = [*first, *second];
    uses.sort_by_key(|member| faces[member.face].id);
    Ok(BoundEdge { binding, uses })
}

fn check_connectivity(
    shells: &[Vec<usize>],
    edges: &[BoundEdge<'_>],
    count: usize,
    options: PartingOptions<'_>,
) -> Checked<()> {
    let mut neighbors = vec![Vec::new(); count];
    for edge in edges {
        let [a, b] = edge.uses.map(|member| member.face);
        neighbors[a].push(b);
        neighbors[b].push(a);
    }
    let mut seen = vec![false; count];
    for shell in shells {
        let mut pending = vec![shell[0]];
        while let Some(face) = pending.pop() {
            observe_stop(options)?;
            if std::mem::replace(&mut seen[face], true) {
                continue;
            }
            pending.extend(
                neighbors[face]
                    .iter()
                    .copied()
                    .filter(|&other| !seen[other]),
            );
        }
        if shell.iter().any(|&member| !seen[member]) {
            return Err(topology(PartingEntity::Solid, Topology::ShellConnectivity));
        }
    }
    Ok(())
}

fn check_exclusions(source: PartingSource<'_>, options: PartingOptions<'_>) -> Checked<()> {
    for id in options.exclude_faces {
        if source.faces.iter().all(|binding| binding.id != *id) {
            return Err(invalid_binding(
                PartingEntity::Face(*id),
                Binding::UnknownExclusion,
            ));
        }
    }
    Ok(())
}

fn key(shape: &Shape) -> Shape {
    shape.oriented(Orientation::Forward)
}

fn valid_sense(orientation: Orientation) -> bool {
    matches!(orientation, Orientation::Forward | Orientation::Reversed)
}

fn check_binding_sense(orientation: Orientation, entity: PartingEntity) -> Checked<()> {
    if valid_sense(orientation) {
        return Ok(());
    }
    Err(invalid_binding(entity, Binding::InvalidSense(orientation)))
}

fn require_sense(orientation: Orientation, entity: PartingEntity) -> Checked<()> {
    if valid_sense(orientation) {
        return Ok(());
    }
    Err(topology(entity, Topology::Orientation(orientation)))
}

fn require_kind(shape: &Shape, expected: ShapeEnum, entity: PartingEntity) -> Checked<()> {
    let actual = shape.shape_type().map_err(native(entity))?;
    if actual == expected {
        return Ok(());
    }
    Err(topology(entity, Topology::ChildKind { actual, expected }))
}

fn binding_count(kind: ShapeEnum, supplied: usize, expected: usize) -> PartingFailureKind {
    PartingFailureKind::BindingCount {
        kind,
        supplied,
        expected,
    }
}

fn invalid_binding(entity: PartingEntity, problem: Binding) -> PartingFailureKind {
    PartingFailureKind::Binding { entity, problem }
}

pub(super) fn topology(entity: PartingEntity, problem: Topology) -> PartingFailureKind {
    PartingFailureKind::Topology { entity, problem }
}
