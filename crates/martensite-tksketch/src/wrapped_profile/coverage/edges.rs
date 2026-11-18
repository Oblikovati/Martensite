//! Each declared edge has two opposite face uses and exactly its two endpoint uses.
use super::*;
pub(super) type EdgeFaces = BTreeMap<BindingKey, Vec<BoundUse>>;
pub(super) fn incidences(faces: &[FaceEvidence]) -> Checked<EdgeFaces> {
    let mut result: EdgeFaces = BTreeMap::new();
    for face in faces {
        accumulate_faces(face, &mut result);
    }
    for (edge, uses) in &result {
        opposite(*edge, uses)?;
    }
    Ok(result)
}
fn accumulate_faces(face: &FaceEvidence, target: &mut EdgeFaces) {
    for edge in &face.edges {
        let orientation = martensite_tkg3d::top_abs::compose(face.orientation, edge.orientation);
        target
            .entry(edge.key)
            .or_default()
            .push(BoundUse::new(face.owner, orientation));
    }
}
fn opposite(owner: BindingKey, uses: &[BoundUse]) -> Checked<()> {
    let valid = uses.len() == 2
        && uses[0].key != uses[1].key
        && ((uses[0].orientation == Orientation::Forward
            && uses[1].orientation == Orientation::Reversed)
            || (uses[0].orientation == Orientation::Reversed
                && uses[1].orientation == Orientation::Forward));
    if !valid {
        return Err(fail(
            owner,
            CoverageCause::EdgeFaces {
                actual: uses.to_vec(),
            },
        ));
    }
    Ok(())
}
pub(super) fn endpoints(request: &Request, registry: &Registry<'_>) -> Checked<()> {
    for piece in request.loops.iter().flatten() {
        piece_endpoints(piece, registry)?;
    }
    Ok(())
}
fn piece_endpoints(piece: &AuthoredPiece, registry: &Registry<'_>) -> Checked<()> {
    depth_endpoints(piece, registry)?;
    let key = expected::corner(piece.start, HistoryRole::NormalEdge);
    members::uses(
        key,
        registry.shape(key)?,
        ShapeEnum::Vertex,
        &expected::normal_ends(piece.start),
        registry,
    )?;
    leaves(piece.start, registry)
}
fn depth_endpoints(piece: &AuthoredPiece, registry: &Registry<'_>) -> Checked<()> {
    for (edge, vertex) in [
        (HistoryRole::FirstDepthEdge, HistoryRole::FirstDepthVertex),
        (HistoryRole::LastDepthEdge, HistoryRole::LastDepthVertex),
    ] {
        let key = expected::span(piece.span, edge);
        members::uses(
            key,
            registry.shape(key)?,
            ShapeEnum::Vertex,
            &expected::ends(piece, vertex),
            registry,
        )?;
    }
    Ok(())
}

fn leaves(source: CornerSource, registry: &Registry<'_>) -> Checked<()> {
    for role in [HistoryRole::FirstDepthVertex, HistoryRole::LastDepthVertex] {
        let key = expected::corner(source, role);
        members::children(key, registry.shape(key)?, ShapeEnum::Shape, 0)?;
    }
    Ok(())
}
