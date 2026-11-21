//! Exact incidence ties each analytic face to its expected authored boundaries.
use super::{
    failure::*,
    history::*,
    request::{SurfaceCurveId, SurfaceVertexId},
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn validate_face(
    corners: [SurfaceVertexId; 4],
    boundaries: [SurfaceCurveId; 4],
    key: HistoryKey,
    face: &Shape,
    bindings: &Bindings,
) -> Checked<()> {
    let actual = face_boundaries(face, key)?;
    for edge in face_edges(corners, boundaries, key)? {
        require_edge(&actual, edge, bindings)?;
    }
    Ok(())
}
fn face_boundaries(face: &Shape, key: HistoryKey) -> Checked<IndexedMapOfShape> {
    let mut actual = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut actual)
        .map_err(|error| native("face trim index", error))?;
    if actual.extent() != 4 {
        return Err(incomplete(
            "four face boundaries",
            format!("{key:?}: {} edges", actual.extent()),
        ));
    }
    Ok(actual)
}
fn face_edges(
    corners: [SurfaceVertexId; 4],
    boundaries: [SurfaceCurveId; 4],
    key: HistoryKey,
) -> Checked<[HistoryKey; 4]> {
    if key.role == HistoryRole::SideFace {
        return side_edges(corners, boundaries, key);
    }
    let role = if key.role == HistoryRole::FirstDepthFace {
        HistoryRole::FirstDepthEdge
    } else {
        HistoryRole::LastDepthEdge
    };
    Ok(boundaries.map(|id| named(HistorySource::Curve(id), role)))
}

fn require_edge(actual: &IndexedMapOfShape, key: HistoryKey, bindings: &Bindings) -> Checked<()> {
    let shape = bindings
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .ok_or_else(|| incomplete("face boundary binding", format!("missing {key:?}")))?;
    if actual.find_index(&shape.1) == 0 {
        return Err(incomplete(
            "face semantic boundary",
            format!("missing {key:?}"),
        ));
    }
    Ok(())
}
fn named(source: HistorySource, role: HistoryRole) -> HistoryKey {
    HistoryKey { source, role }
}
fn side_edges(
    corners: [SurfaceVertexId; 4],
    boundaries: [SurfaceCurveId; 4],
    key: HistoryKey,
) -> Checked<[HistoryKey; 4]> {
    let index = boundaries
        .iter()
        .position(|id| key.source == HistorySource::Curve(*id))
        .ok_or_else(|| incomplete("side authored boundary", format!("{key:?}")))?;
    let first = HistorySource::Vertex(corners[index]);
    let last = HistorySource::Vertex(corners[(index + 1) % 4]);
    Ok([
        named(key.source, HistoryRole::FirstDepthEdge),
        named(key.source, HistoryRole::LastDepthEdge),
        named(first, HistoryRole::NormalEdge),
        named(last, HistoryRole::NormalEdge),
    ])
}
