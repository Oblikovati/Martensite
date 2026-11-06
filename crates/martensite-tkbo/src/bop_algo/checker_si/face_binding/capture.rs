// SPDX-License-Identifier: GPL-2.0-only
//! Canonical Forward-face charts retain original occurrence orientation separately.
use super::{
    capture_curves::{placement, spline_values, unsupported},
    capture_edges, capture_vertices,
    types::*,
};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools,
    topods::{self, Face, ShapeIterator},
};
use martensite_tkg3d::{
    geom::{GeomSurfaceOfLinearExtrusion, GeomSweptSurface},
    top_abs::{Orientation, ShapeEnum},
};

/// Copy actual support/trim inputs without asserting an atomic concurrent capture.
/// Example: `capture_face(&face)` retains independent arrays and actual identity relations.
pub(crate) fn capture_face(face: &Face) -> Result<OwnedFacePacket, BindingFailure> {
    let (basis, direction, support_placement) = capture_support(face)?;
    let bounds = brep_tools::uv_bounds(face)?;
    let (wire_count, edges, relations, vertex_relations) = capture_incidence(face)?;
    Ok(OwnedFacePacket {
        raw_face_tolerance: capture_vertices::raw_tolerance(face)?,
        vertex_relations,
        invocation: invocation_identity(face)?,
        basis,
        direction,
        support_placement,
        uv_bounds: [bounds.0, bounds.1, bounds.2, bounds.3],
        face_orientation: face.orientation(),
        face_tolerance: BRepTool::face_tolerance(face)?,
        wire_count,
        edges,
        relations,
    })
}
#[allow(clippy::type_complexity)] // Test-only capture rows.
fn capture_incidence(
    face: &Face,
) -> Result<
    (
        usize,
        Vec<OwnedEdgeUse>,
        Vec<OccurrenceRelation>,
        Vec<OccurrenceRelation>,
    ),
    BindingFailure,
> {
    let canonical = topods::face(&face.oriented(Orientation::Forward))?;
    let mut sources = Vec::new();
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut wire_count = 0;
    for (wire_position, wire) in ShapeIterator::new(&canonical, true, true).enumerate() {
        if wire.shape_type()? != ShapeEnum::Wire {
            return Err(unsupported(
                "face child",
                format!("{:?}; expected Wire", wire.shape_type()?),
            ));
        }
        wire_count += 1;
        append_edges(
            &canonical,
            &wire,
            wire_position,
            &mut edges,
            &mut sources,
            &mut vertices,
        )?;
    }
    let relations = capture_edges::relations(&sources);
    Ok((
        wire_count,
        edges,
        relations,
        capture_vertices::relations(&vertices),
    ))
}

fn invocation_identity(face: &Face) -> Result<FaceInvocationIdentity, BindingFailure> {
    let identity = capture_edges::identity(face)?;
    Ok(FaceInvocationIdentity {
        tshape_token: identity.tshape_token,
        location_datums: identity.location_datums,
    })
}

fn capture_support(
    face: &Face,
) -> Result<(OwnedSpline, [f64; 3], AffinePlacement), BindingFailure> {
    let (support, location) = BRepTool::surface_with_location(face)?;
    let handle = require_extrusion(&support)?;
    let source = handle.get().expect("checked extrusion handle");
    let basis = source
        .basis_curve()
        .cast::<martensite_tkg3d::geom::GeomBSplineCurve>();
    let basis = spline_values(
        basis
            .get()
            .ok_or_else(|| unsupported("extrusion basis", "not built-in B-spline"))?,
    )?;
    let direction = source.direction();
    Ok((
        basis,
        [direction.x(), direction.y(), direction.z()],
        placement(location.transformation())?,
    ))
}

fn require_extrusion(
    support: &martensite_tkernel::standard::Handle<dyn martensite_tkg3d::geom::GeomSurface>,
) -> Result<martensite_tkernel::standard::Handle<GeomSurfaceOfLinearExtrusion>, BindingFailure> {
    let handle = support.cast::<GeomSurfaceOfLinearExtrusion>();
    let source = handle.get().ok_or_else(|| {
        unsupported(
            "surface",
            format!("{:?}; expected extrusion", support.dynamic_type()),
        )
    })?;
    if source.has_eval_representation() {
        return Err(unsupported("surface evaluator", "delegated override"));
    }
    Ok(handle)
}

fn append_edges(
    canonical: &Face,
    wire: &martensite_tkbrep::topods::Shape,
    wire_position: usize,
    edges: &mut Vec<OwnedEdgeUse>,
    sources: &mut Vec<martensite_tkbrep::topods::Edge>,
    vertices: &mut Vec<martensite_tkbrep::topods::Vertex>,
) -> Result<(), BindingFailure> {
    for (edge_position, edge) in ShapeIterator::new(wire, true, true).enumerate() {
        let edge = topods::edge(&edge)?;
        let (owned, captured) =
            capture_edges::capture(&edge, canonical, wire, wire_position, edge_position)?;
        edges.push(owned);
        vertices.extend(captured);
        sources.push(edge);
    }
    Ok(())
}
