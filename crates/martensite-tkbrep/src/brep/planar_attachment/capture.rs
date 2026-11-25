// SPDX-License-Identifier: GPL-2.0-only
//! Decode only the coherent source values, with no second reads of live geometry.
use super::{FaceAttachmentError as Error, PlanarSupport, TrimScope, numeric_failure, poll};
use crate::{
    brep::{
        face_input_capture::{CapturedFaceCell as Cell, CapturedOccurrence, FaceInputSnapshot},
        tedge::DEGENERATED_MASK,
    },
    topods::bit_layout::{BIT_CLOSED, BITS_SHAPE_TYPE_MASK},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    gp::{Axis2d, Axis3, Line3},
    predicates::roots::metric_bounds::planar_attachment::{
        CertifiedPlanarAttachment, PLANAR_ATTACHMENT_MAX_LOOP_EDGES, PlanarAttachmentEdge,
    },
};
use std::collections::BTreeMap;

type Checked<T> = Result<T, Error>;

pub(in crate::brep) struct Captured<'a>(BTreeMap<usize, Vec<&'a Cell>>);

struct DirectedEdge {
    topology: [usize; 2],
    input: PlanarAttachmentEdge,
}

pub(super) fn prove(
    snapshot: &FaceInputSnapshot,
    scope: TrimScope,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<(CertifiedPlanarAttachment, PlanarSupport)> {
    let root = snapshot.root();
    reversed(root)?;
    let values = Captured::index(snapshot);
    let (surface, frame) = values.support(root.owner)?;
    let wires = values.children(root.owner, ShapeEnum::Face)?;
    let (outer, hole) = match (scope, wires) {
        (TrimScope::OptionalHole, [outer]) => (outer, None),
        (_, [a, b]) => (a, Some(b)),
        _ => return Err(Error::Topology(root.owner)),
    };
    let a = values.wire(outer, surface, cancelled)?;
    let b = hole
        .map(|hole| values.wire(hole, surface, cancelled))
        .transpose()?;
    for (_, _, datum) in root.location.steps() {
        poll(cancelled)?;
        datum
            .checked_unit_placement_rows()
            .map_err(Error::Placement)?;
    }
    let position = root.location.effective_transform();
    let placement = position
        .checked_unit_placement_rows()
        .map_err(Error::Placement)?;
    let proof = match b {
        Some(b) => CertifiedPlanarAttachment::capture(frame, placement, [&a, &b], cancelled),
        None => CertifiedPlanarAttachment::capture_outer(frame, placement, &a, cancelled),
    }
    .map_err(numeric_failure)?;
    Ok((proof, PlanarSupport { frame, position }))
}

impl<'a> Captured<'a> {
    pub(in crate::brep) fn index(snapshot: &'a FaceInputSnapshot) -> Self {
        let mut entries = BTreeMap::<_, Vec<_>>::new();
        for (owner, cell) in snapshot.cells() {
            entries.entry(*owner).or_default().push(cell);
        }
        Self(entries)
    }

    pub(in crate::brep) fn cells(&self, owner: usize) -> Checked<&[&'a Cell]> {
        self.0
            .get(&owner)
            .map(Vec::as_slice)
            .ok_or(Error::Topology(owner))
    }

    fn support(&self, owner: usize) -> Checked<(usize, Axis3)> {
        let surface = self.surface(owner)?;
        let [Cell::Plane(frame)] = self.cells(surface)? else {
            return Err(Error::Representation(surface));
        };
        Ok((surface, *frame))
    }

    pub(in crate::brep) fn surface(&self, owner: usize) -> Checked<usize> {
        let (surface, location, tolerance) = self
            .cells(owner)?
            .iter()
            .find_map(|cell| match cell {
                Cell::Face {
                    surface,
                    location,
                    tolerance,
                    ..
                } => Some((*surface, location, *tolerance)),
                _ => None,
            })
            .ok_or(Error::Representation(owner))?;
        if !location.steps().is_empty() || tolerance < 0. {
            return Err(Error::Representation(owner));
        }
        Ok(surface)
    }

    pub(in crate::brep) fn children(
        &self,
        owner: usize,
        kind: ShapeEnum,
    ) -> Checked<&'a [CapturedOccurrence]> {
        for cell in self.cells(owner)? {
            if let Cell::Topology { state, children } = cell {
                let valid_kind = usize::from(*state & BITS_SHAPE_TYPE_MASK) == kind as usize;
                let closed = kind != ShapeEnum::Wire || *state & BIT_CLOSED != 0;
                return (valid_kind && closed)
                    .then_some(children)
                    .map(Vec::as_slice)
                    .ok_or(Error::Topology(owner));
            }
        }
        Err(Error::Topology(owner))
    }

    fn wire(
        &self,
        wire: &CapturedOccurrence,
        surface: usize,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<Vec<PlanarAttachmentEdge>> {
        local(wire)?;
        let reverse = reversed(wire)?;
        let edges = self.children(wire.owner, ShapeEnum::Wire)?;
        if !(3..=PLANAR_ATTACHMENT_MAX_LOOP_EDGES).contains(&edges.len()) {
            return Err(Error::Topology(wire.owner));
        }
        let mut directed = Vec::with_capacity(edges.len());
        for edge in edges {
            poll(cancelled)?;
            if edges
                .iter()
                .filter(|candidate| candidate.owner == edge.owner)
                .count()
                != 1
            {
                return Err(Error::Topology(edge.owner));
            }
            directed.push(self.edge(edge, surface, reverse)?);
        }
        join_cycle(directed, wire.owner, cancelled)
    }

    fn edge(
        &self,
        edge: &CapturedOccurrence,
        surface: usize,
        wire_reverse: bool,
    ) -> Checked<DirectedEdge> {
        local(edge)?;
        let reverse = wire_reverse ^ reversed(edge)?;
        let endpoints = self.endpoints(edge.owner, reverse)?;
        let reprs = self.edge_representations(edge.owner)?;
        let (line, range, pcurve) = self.curves(reprs, surface, edge.owner)?;
        let [a, b] = endpoints;
        let vertices = [self.vertex(a)?, self.vertex(b)?];
        Ok(DirectedEdge {
            topology: endpoints,
            input: PlanarAttachmentEdge {
                line,
                range,
                vertices,
                reversed: reverse,
                pcurve,
            },
        })
    }

    fn endpoints(&self, edge: usize, reverse: bool) -> Checked<[usize; 2]> {
        let [a, b] = self.directed_vertices(edge, reverse)?;
        if a == b {
            return Err(Error::Topology(edge));
        }
        Ok([a, b])
    }

    pub(in crate::brep) fn directed_vertices(
        &self,
        edge: usize,
        reverse: bool,
    ) -> Checked<[usize; 2]> {
        let [a, b] = self.children(edge, ShapeEnum::Edge)? else {
            return Err(Error::Topology(edge));
        };
        let [ar, br] = [reversed(a)?, reversed(b)?];
        local(a)?;
        local(b)?;
        if ar == br {
            return Err(Error::Topology(edge));
        }
        Ok(if ar ^ reverse {
            [b.owner, a.owner]
        } else {
            [a.owner, b.owner]
        })
    }

    pub(in crate::brep) fn vertex(&self, owner: usize) -> Checked<[f64; 3]> {
        if !self.children(owner, ShapeEnum::Vertex)?.is_empty() {
            return Err(Error::Topology(owner));
        }
        self.cells(owner)?
            .iter()
            .find_map(|cell| match cell {
                Cell::Vertex { point, tolerance } if *tolerance >= 0. => Some(*point),
                _ => None,
            })
            .ok_or(Error::Representation(owner))
    }

    pub(in crate::brep) fn edge_representations(&self, owner: usize) -> Checked<&'a [usize]> {
        self.cells(owner)?
            .iter()
            .find_map(|cell| match cell {
                Cell::Edge {
                    tolerance,
                    flags,
                    representations,
                } if *tolerance >= 0. && *flags & DEGENERATED_MASK == 0 => {
                    Some(representations.as_slice())
                }
                _ => None,
            })
            .ok_or(Error::Representation(owner))
    }

    fn curves(&self, representations: &[usize], surface: usize, edge: usize) -> Checked<Curves> {
        let mut spatial = None;
        let mut parametric = None;
        for &owner in representations {
            self.curve(owner, surface, &mut spatial, &mut parametric)?;
        }
        let (line, range) = spatial.ok_or(Error::Representation(edge))?;
        Ok((line, range, parametric))
    }

    fn curve(
        &self,
        owner: usize,
        support: usize,
        spatial: &mut Option<(Line3, [f64; 2])>,
        parametric: &mut Option<(Axis2d, [f64; 2])>,
    ) -> Checked<()> {
        let cells = self.cells(owner)?;
        for cell in cells {
            match cell {
                Cell::Spatial { curve } => once(spatial, self.spatial(*curve, owner)?, owner)?,
                Cell::AbsentSpatial => return Err(Error::Representation(owner)),
                Cell::Parametric { surface, curve, .. } if *surface == support => {
                    if cells.iter().any(|cell| matches!(cell, Cell::Seam { .. })) {
                        return Err(Error::Representation(owner));
                    }
                    once(parametric, self.parametric(*curve, owner)?, owner)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn spatial(&self, curve: usize, instance: usize) -> Checked<(Line3, [f64; 2])> {
        let [Cell::Line(axis)] = self.cells(curve)? else {
            return Err(Error::Representation(curve));
        };
        Ok((Line3::from_axis(*axis), self.range(instance)?))
    }

    pub(in crate::brep) fn parametric(
        &self,
        curve: usize,
        instance: usize,
    ) -> Checked<(Axis2d, [f64; 2])> {
        let [Cell::PlanarLine(axis)] = self.cells(curve)? else {
            return Err(Error::Representation(curve));
        };
        Ok((*axis, self.range(instance)?))
    }

    pub(in crate::brep) fn range(&self, representation: usize) -> Checked<[f64; 2]> {
        let cells = self.cells(representation)?;
        let allowed = cells
            .iter()
            .any(|cell| matches!(cell, Cell::Location(location) if location.steps().is_empty()));
        if !allowed {
            return Err(Error::Representation(representation));
        }
        cells
            .iter()
            .find_map(|cell| match cell {
                Cell::Range(range) => Some(*range),
                _ => None,
            })
            .ok_or(Error::Representation(representation))
    }
}

type Curves = (Line3, [f64; 2], Option<(Axis2d, [f64; 2])>);

fn once<T>(slot: &mut Option<T>, value: T, owner: usize) -> Checked<()> {
    if slot.replace(value).is_some() {
        return Err(Error::Representation(owner));
    }
    Ok(())
}

fn join_cycle(
    mut pending: Vec<DirectedEdge>,
    wire: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Vec<PlanarAttachmentEdge>> {
    let first = pending.remove(0);
    let start = first.topology[0];
    let mut end = first.topology[1];
    let mut ordered = Vec::with_capacity(pending.len() + 1);
    ordered.push(first.input);
    while !pending.is_empty() {
        poll(cancelled)?;
        let next = unique_successor(&pending, end).ok_or(Error::Topology(wire))?;
        let edge = pending.swap_remove(next);
        end = edge.topology[1];
        ordered.push(edge.input);
    }
    (start == end)
        .then_some(ordered)
        .ok_or(Error::Topology(wire))
}

fn unique_successor(pending: &[DirectedEdge], vertex: usize) -> Option<usize> {
    let mut matches = pending
        .iter()
        .enumerate()
        .filter(|(_, edge)| edge.topology[0] == vertex);
    let (index, _) = matches.next()?;
    matches.next().is_none().then_some(index)
}

pub(in crate::brep) fn reversed(occurrence: &CapturedOccurrence) -> Checked<bool> {
    match occurrence.orientation {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        _ => Err(Error::Topology(occurrence.owner)),
    }
}

pub(in crate::brep) fn local(occurrence: &CapturedOccurrence) -> Checked<()> {
    occurrence
        .location
        .steps()
        .is_empty()
        .then_some(())
        .ok_or(Error::Topology(occurrence.owner))
}
