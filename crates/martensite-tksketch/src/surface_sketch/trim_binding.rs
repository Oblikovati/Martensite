//! Exact incidence ties each analytic face to its expected authored boundaries.
use super::{admission::Admitted, failure::*, history::*};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn validate_face(
    admitted: &Admitted,
    key: HistoryKey,
    face: &Shape,
    bindings: &Bindings,
) -> Checked<()> {
    let mut actual = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut actual)
        .map_err(|error| native("face trim index", error))?;
    if actual.extent() != 4 {
        return Err(incomplete(
            "four face boundaries",
            format!("{key:?}: {} edges", actual.extent()),
        ));
    }
    for edge in face_edges(admitted, key)? {
        require_edge(&actual, edge, bindings)?;
    }
    Ok(())
}
fn face_edges(admitted: &Admitted, key: HistoryKey) -> Checked<[HistoryKey; 4]> {
    if key.role == HistoryRole::SideFace {
        return side_edges(admitted, key);
    }
    let role = if key.role == HistoryRole::FirstDepthFace {
        HistoryRole::FirstDepthEdge
    } else {
        HistoryRole::LastDepthEdge
    };
    Ok(admitted
        .rectangle
        .boundaries
        .map(|id| named(HistorySource::Curve(id), role)))
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
fn side_edges(admitted: &Admitted, key: HistoryKey) -> Checked<[HistoryKey; 4]> {
    let index = admitted
        .rectangle
        .boundaries
        .iter()
        .position(|id| key.source == HistorySource::Curve(*id))
        .ok_or_else(|| incomplete("side authored boundary", format!("{key:?}")))?;
    let first = HistorySource::Vertex(admitted.rectangle.corners[index]);
    let last = HistorySource::Vertex(admitted.rectangle.corners[(index + 1) % 4]);
    Ok([
        named(key.source, HistoryRole::FirstDepthEdge),
        named(key.source, HistoryRole::LastDepthEdge),
        named(first, HistoryRole::NormalEdge),
        named(last, HistoryRole::NormalEdge),
    ])
}
