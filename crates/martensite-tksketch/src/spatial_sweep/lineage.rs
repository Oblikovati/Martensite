// SPDX-License-Identifier: GPL-2.0-only
//! Stable two-source faces and complete lower shape incidence in the retained shell.
use super::{
    SweepEdgeHistory, SweepFaceHistory, SweepFailure as Failure, SweepHistory, SweepImage,
    SweepOptions, SweepPathPart, SweepStage as Stage, SweepSupportEvidence, admission::Admission,
    geometry, native, poll,
};
use martensite_tkbrep::{
    topexp,
    topods::{self, Shape, Shell, Vertex},
    toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakePipeShell;
use martensite_tksketch3d::{PointId, wire::DetachedWire};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::collections::BTreeSet;

pub(super) struct Face {
    pub shape: topods::Face,
    pub history: SweepFaceHistory,
    pub profile: usize,
    pub station: usize,
    pub corner: bool,
}

pub(super) fn bind(
    pipe: &mut BRepOffsetAPIMakePipeShell,
    shell: &Shell,
    wires: &[DetachedWire; 2],
    terminals: [&Shape; 2],
    admitted: &Admission,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Vec<SweepImage>, SweepSupportEvidence), Failure> {
    let owned = Sources::read(pipe, wires, admitted, cancelled)?;
    let faces = owned.bind_faces(shell, wires, admitted, cancelled)?;
    let edges = owned.bind_edges(shell, &faces, wires, admitted, cancelled)?;
    let vertices = bind_vertices(shell, &edges, cancelled)?;
    let terminal_vertices = terminal_bindings(shell, terminals, &edges, &faces, admitted)?;
    let evidence = geometry::verify(admitted, &faces, &terminal_vertices, options, cancelled)?;
    let mut images = Vec::with_capacity(faces.len() + edges.len() + vertices.len());
    images.extend(faces.into_iter().map(|face| SweepImage {
        shape: face.shape.into_shape(),
        history: SweepHistory::Face(face.history),
    }));
    images.extend(edges.into_iter().map(|edge| SweepImage {
        shape: edge.shape,
        history: SweepHistory::Edge(edge.history),
    }));
    images.extend(vertices);
    Ok((images, evidence))
}

struct Sources {
    profile_faces: Vec<IndexedMapOfShape>,
    profile_rails: Vec<IndexedMapOfShape>,
    path_bands: Vec<IndexedMapOfShape>,
    path_stations: Vec<IndexedMapOfShape>,
}

impl Sources {
    fn read(
        pipe: &mut BRepOffsetAPIMakePipeShell,
        [path, profile]: &[DetachedWire; 2],
        admitted: &Admission,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        let tails = admitted
            .stations
            .iter()
            .map(|station| {
                path.points
                    .iter()
                    .find(|point| point.point == station.point)
                    .map(|point| point.vertex.as_ref())
                    .ok_or(Failure::Binding("every authored path station"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            profile_faces: capture(
                pipe,
                profile.edges.iter().map(|edge| edge.edge.as_ref()),
                cancelled,
            )?,
            profile_rails: capture(
                pipe,
                profile.points.iter().map(|point| point.vertex.as_ref()),
                cancelled,
            )?,
            path_bands: capture(
                pipe,
                path.edges.iter().map(|edge| edge.edge.as_ref()),
                cancelled,
            )?,
            path_stations: capture(pipe, tails, cancelled)?,
        })
    }

    fn bind_faces(
        &self,
        shell: &Shell,
        [path, profile]: &[DetachedWire; 2],
        admitted: &Admission,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Face>, Failure> {
        let native_faces = members(shell, ShapeEnum::Face)?;
        let mut faces = Vec::with_capacity(native_faces.extent());
        for i in 1..=native_faces.extent() {
            poll(cancelled, Stage::Binding)?;
            let shape = native(native_faces.find_key(i), Stage::Binding)?;
            let profile_line = owner(&self.profile_faces, shape)?
                .ok_or(Failure::Binding("one profile tape for each face"))?;
            let band = owner(&self.path_bands, shape)?;
            let corner = owner(&self.path_stations, shape)?;
            let (station, corner, part) = match (band, corner) {
                (Some(band), None) => (band, false, SweepPathPart::Band(path.edges[band].source)),
                (None, Some(station @ (1 | 2))) => (
                    station,
                    true,
                    SweepPathPart::Corner {
                        station: admitted.stations[station].point,
                        incoming: path.edges[station - 1].source,
                        outgoing: path.edges[station].source,
                    },
                ),
                _ => {
                    return Err(Failure::Binding(
                        "one spine band or interior station for each face",
                    ));
                }
            };
            let history = SweepFaceHistory {
                profile: profile.edges[profile_line].source,
                path: part,
            };
            faces.push(Face {
                shape: native(topods::face(shape), Stage::Binding)?,
                history,
                profile: profile_line,
                station,
                corner,
            });
        }
        // Sort by input occurrence order, then reject any native split without its own provenance.
        faces.sort_by_key(|face| (face.station, face.corner, face.profile));
        if faces
            .windows(2)
            .any(|pair| pair[0].history == pair[1].history)
            || faces.iter().filter(|face| !face.corner).count() != 3 * profile.edges.len()
            || faces.iter().filter(|face| face.corner).count() != 6
        {
            return Err(Failure::Binding(
                "all distinct band faces and the six rounded profile/station bindings",
            ));
        }
        Ok(faces)
    }

    fn bind_edges(
        &self,
        shell: &Shell,
        faces: &[Face],
        [_, profile]: &[DetachedWire; 2],
        admitted: &Admission,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Edge>, Failure> {
        let face_ids = shape_ids(faces.iter().map(|face| face.shape.as_ref()));
        let incidence = ancestors(shell, ShapeEnum::Edge, ShapeEnum::Face)?;
        let mut edges = Vec::with_capacity(incidence.extent());
        for index in 1..=incidence.extent() {
            poll(cancelled, Stage::Binding)?;
            let edge = native(incidence.find_key(index), Stage::Binding)?;
            let adjacent = adjacent_ids(&incidence, edge, &face_ids)?;
            if !(1..=2).contains(&adjacent.len()) {
                return Err(Failure::Binding(
                    "one or two labeled faces incident to each shell edge",
                ));
            }
            let original_point = owner(&self.profile_rails, edge)?.map(|i| profile.points[i].point);
            let station = owner(&self.path_stations, edge)?.map(|i| admitted.stations[i].point);
            let history = SweepEdgeHistory {
                profile_point: original_point,
                spine_station: station,
                faces: adjacent.iter().map(|i| faces[*i].history).collect(),
            };
            edges.push(Edge {
                shape: edge.clone(),
                adjacent,
                history,
            });
        }
        edges.sort_by(|a, b| a.key().cmp(&b.key()));
        if edges.windows(2).any(|pair| pair[0].key() == pair[1].key()) {
            return Err(Failure::Binding(
                "unique edge ancestry or explicit native split receipts",
            ));
        }
        Ok(edges)
    }
}

struct Edge {
    shape: Shape,
    adjacent: Vec<usize>,
    history: SweepEdgeHistory,
}

impl Edge {
    fn key(&self) -> (&[usize], Option<PointId>, Option<PointId>) {
        (
            &self.adjacent,
            self.history.profile_point,
            self.history.spine_station,
        )
    }
}

fn capture<'a>(
    pipe: &mut BRepOffsetAPIMakePipeShell,
    sources: impl IntoIterator<Item = &'a Shape>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<IndexedMapOfShape>, Failure> {
    sources
        .into_iter()
        .map(|source| {
            poll(cancelled, Stage::Binding)?;
            let images = native(pipe.generated(source), Stage::Binding)?;
            Ok(shape_ids(images.iter().filter(|shape| !shape.is_null())))
        })
        .collect()
}

fn owner(maps: &[IndexedMapOfShape], shape: &Shape) -> Result<Option<usize>, Failure> {
    let mut found = maps
        .iter()
        .enumerate()
        .filter(|(_, map)| map.contains(shape));
    let owner = found.next().map(|(index, _)| index);
    if found.next().is_some() {
        return Err(Failure::Binding(
            "one generating source within each authored namespace",
        ));
    }
    Ok(owner)
}

fn members(shape: &Shape, kind: ShapeEnum) -> Result<IndexedMapOfShape, Failure> {
    let mut map = IndexedMapOfShape::default();
    native(
        topexp::map_shapes_of_type(shape, kind, &mut map),
        Stage::Binding,
    )?;
    Ok(map)
}

fn ancestors(
    shape: &Shape,
    child: ShapeEnum,
    parent: ShapeEnum,
) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut map = IndexedDataMapOfShapeListOfShape::default();
    native(
        topexp::map_shapes_and_ancestors(shape, child, parent, &mut map),
        Stage::Binding,
    )?;
    Ok(map)
}

fn shape_ids<'a>(shapes: impl IntoIterator<Item = &'a Shape>) -> IndexedMapOfShape {
    let mut ids = IndexedMapOfShape::default();
    for shape in shapes {
        ids.add(shape.clone());
    }
    ids
}

fn adjacent_ids(
    map: &IndexedDataMapOfShapeListOfShape,
    shape: &Shape,
    ids: &IndexedMapOfShape,
) -> Result<Vec<usize>, Failure> {
    let adjacent = native(map.find_from_key(shape), Stage::Binding)?;
    adjacent
        .iter()
        .map(|shape| {
            ids.find_index(shape).checked_sub(1).ok_or(Failure::Binding(
                "all incident shapes in the completed result",
            ))
        })
        .collect::<Result<BTreeSet<_>, _>>()
        .map(|values| values.into_iter().collect())
}

fn bind_vertices(
    shell: &Shell,
    edges: &[Edge],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<SweepImage>, Failure> {
    let edge_ids = shape_ids(edges.iter().map(|edge| &edge.shape));
    let incidence = ancestors(shell, ShapeEnum::Vertex, ShapeEnum::Edge)?;
    let mut pending = Vec::with_capacity(incidence.extent());
    for index in 1..=incidence.extent() {
        poll(cancelled, Stage::Binding)?;
        let shape = native(incidence.find_key(index), Stage::Binding)?;
        let ids = adjacent_ids(&incidence, shape, &edge_ids)?;
        if ids.len() < 2 {
            return Err(Failure::Binding("labeled incidence at each native vertex"));
        }
        pending.push((ids, shape.clone()));
    }
    pending.sort_by(|a, b| a.0.cmp(&b.0));
    if pending.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(Failure::Binding(
            "distinct vertex incidence relative to authored edge roles",
        ));
    }
    Ok(pending
        .into_iter()
        .map(|(ids, shape)| SweepImage {
            history: SweepHistory::Vertex(
                ids.into_iter()
                    .map(|id| edges[id].history.clone())
                    .collect(),
            ),
            shape,
        })
        .collect())
}

pub(super) struct TerminalVertex {
    pub station: usize,
    pub profile_point: PointId,
    pub actual: Vertex,
}

fn terminal_bindings(
    shell: &Shell,
    terminals: [&Shape; 2],
    edges: &[Edge],
    faces: &[Face],
    admitted: &Admission,
) -> Result<Vec<TerminalVertex>, Failure> {
    let incidence = ancestors(shell, ShapeEnum::Vertex, ShapeEnum::Edge)?;
    let ids = shape_ids(edges.iter().map(|edge| &edge.shape));
    let mut matched = IndexedMapOfShape::default();
    let mut points = Vec::new();
    for (terminal, station) in terminals.into_iter().zip([0, 3]) {
        let wire = native(topods::wire(terminal), Stage::Binding)?;
        let boundary = members(&wire, ShapeEnum::Edge)?;
        if boundary.extent() != 4 || !native(wire.closed(), Stage::Binding)? {
            return Err(Failure::Binding("two closed four-edge terminal rings"));
        }
        for i in 1..=boundary.extent() {
            let shape = native(boundary.find_key(i), Stage::Binding)?;
            let id = ids
                .find_index(shape)
                .checked_sub(1)
                .ok_or(Failure::Binding("terminal edges on the shell"))?;
            let edge = &edges[id];
            if edge.adjacent.len() != 1
                || edge.history.spine_station != Some(admitted.stations[station].point)
                || faces[edge.adjacent[0]].corner
                || matched.contains(shape)
            {
                return Err(Failure::Binding("exact distinct terminal station edges"));
            }
            matched.add(shape.clone());
        }
        points.extend(terminal_points(&wire, station, &incidence, &ids, edges)?);
    }
    if edges.iter().filter(|edge| edge.adjacent.len() == 1).count() != matched.extent() {
        return Err(Failure::Binding(
            "the selected two rings are the whole sheet boundary",
        ));
    }
    Ok(points)
}

fn terminal_points(
    wire: &topods::Wire,
    station: usize,
    incidence: &IndexedDataMapOfShapeListOfShape,
    ids: &IndexedMapOfShape,
    edges: &[Edge],
) -> Result<Vec<TerminalVertex>, Failure> {
    let vertices = members(wire, ShapeEnum::Vertex)?;
    let mut owners = BTreeSet::new();
    let mut bound = Vec::new();
    for i in 1..=vertices.extent() {
        let shape = native(vertices.find_key(i), Stage::Binding)?;
        let candidates = adjacent_ids(incidence, shape, ids)?
            .into_iter()
            .filter_map(|index| edges[index].history.profile_point)
            .collect::<BTreeSet<_>>();
        let [profile_point] = candidates
            .into_iter()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| Failure::Binding("one profile rail at each native terminal vertex"))?;
        if !owners.insert(profile_point) {
            return Err(Failure::Binding(
                "a distinct authored point for each terminal vertex",
            ));
        }
        bound.push(TerminalVertex {
            station,
            profile_point,
            actual: native(topods::vertex(shape), Stage::Binding)?,
        });
    }
    if bound.len() != 4 {
        return Err(Failure::Binding("four exact points in each terminal ring"));
    }
    Ok(bound)
}
