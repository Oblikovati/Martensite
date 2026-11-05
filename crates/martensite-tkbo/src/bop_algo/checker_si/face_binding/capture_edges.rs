// SPDX-License-Identifier: GPL-2.0-only
use super::{
    capture_curves::{self, unsupported},
    capture_vertices,
    types::*,
};
use martensite_tkbrep::{
    brep::{BRepTEdge, BRepTool},
    topods::{Edge, Face, Shape},
};

type StoredFaceRepresentation =
    martensite_tkernel::standard::Handle<dyn martensite_tkbrep::brep::BRepCurveRepresentation>;

pub(super) fn capture(
    edge: &Edge,
    face: &Face,
    wire: &Shape,
    wire_position: usize,
    edge_position: usize,
) -> Result<(OwnedEdgeUse, Vec<martensite_tkbrep::topods::Vertex>), BindingFailure> {
    let stored = stored_representation(edge, face)?;
    let query = BRepTool::curve_on_face(edge, face)?;
    if !query.is_stored {
        return Err(unsupported(
            "pcurve provenance",
            "generated or missing; expected stored representation",
        ));
    }
    let stored_branch = selected_branch(&stored, &query.pcurve)?;
    let curve = BRepTool::curve_with_location(edge)?;
    let (vertices, sources) = capture_vertices::capture(edge)?;
    Ok((
        OwnedEdgeUse {
            vertices,
            raw_tolerance: capture_vertices::raw_tolerance(edge)?,
            context: context(edge, face, wire, wire_position, edge_position)?,
            canonical_orientation: edge.orientation(),
            pcurve: capture_curves::plane(&query.pcurve)?,
            range: [query.first, query.last],
            curve: capture_curves::space(&curve.curve)?,
            curve_range: [curve.first, curve.last],
            placement: capture_curves::placement(curve.location.transformation())?,
            tolerance: BRepTool::edge_tolerance(edge)?,
            stored_branch,
            geometric_seam: geometric_seam(edge, face)?,
        },
        sources,
    ))
}
fn geometric_seam(edge: &Edge, face: &Face) -> Result<bool, BindingFailure> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    Ok(BRepTool::is_closed_on_surface(edge, &surface, &location)?)
}
fn stored_representation(
    edge: &Edge,
    face: &Face,
) -> Result<StoredFaceRepresentation, BindingFailure> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    let relative = location.predivided(edge.location())?;
    let curves = source_curves(edge)?;
    let matched = curves
        .into_iter()
        .filter(|r| {
            r.get()
                .is_some_and(|r| r.is_curve_on_surface_at(&surface, &relative))
        })
        .collect::<Vec<_>>();
    if matched.len() != 1 {
        return Err(BindingFailure::Count {
            role: "stored face representations",
            actual: matched.len(),
            expected: 1,
        });
    }
    Ok(matched[0].clone())
}
fn selected_branch(
    stored: &StoredFaceRepresentation,
    selected: &martensite_tkernel::standard::Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
) -> Result<u8, BindingFailure> {
    let rep = stored
        .get()
        .ok_or_else(|| unsupported("stored representation", "null"))?;
    let first = rep.pcurve()? == *selected;
    let second = rep.is_curve_on_closed_surface() && rep.pcurve2()? == *selected;
    match (first, second) {
        (true, false) => Ok(1),
        (false, true) => Ok(2),
        _ => Err(unsupported(
            "stored branch selection",
            format!("first={first}, second={second}; expected exactly one"),
        )),
    }
}
pub(super) fn relations(edges: &[Edge]) -> Vec<OccurrenceRelation> {
    edges
        .iter()
        .enumerate()
        .flat_map(|(left, a)| {
            edges
                .iter()
                .enumerate()
                .skip(left + 1)
                .map(move |(right, b)| OccurrenceRelation {
                    left,
                    right,
                    same: a.is_same(b),
                    location_equal: a.location() == b.location(),
                })
        })
        .collect()
}

fn source_curves(
    edge: &Edge,
) -> Result<
    Vec<martensite_tkernel::standard::Handle<dyn martensite_tkbrep::brep::BRepCurveRepresentation>>,
    BindingFailure,
> {
    let handle = edge.tshape().cast::<BRepTEdge>();
    let source = handle
        .get()
        .ok_or_else(|| unsupported("edge TShape", "not BRepTEdge"))?;
    Ok(source.curves())
}

pub(super) fn identity(shape: &Shape) -> Result<ShapeOccurrenceIdentity, BindingFailure> {
    let mut datums = Vec::new();
    let mut location = shape.location();
    while !location.is_identity() {
        datums.push((
            location.first_datum()?.object_address() as usize,
            location.first_power()?,
        ));
        location = location.next_location();
    }
    Ok(ShapeOccurrenceIdentity {
        tshape_token: shape.tshape_address(),
        location_datums: datums,
        orientation: shape.orientation(),
    })
}

fn context(
    edge: &Edge,
    face: &Face,
    wire: &Shape,
    wire_position: usize,
    edge_position: usize,
) -> Result<EdgeOccurrenceContext, BindingFailure> {
    Ok(EdgeOccurrenceContext {
        face: identity(face)?,
        wire: identity(wire)?,
        edge: identity(edge)?,
        wire_position,
        edge_position,
    })
}
