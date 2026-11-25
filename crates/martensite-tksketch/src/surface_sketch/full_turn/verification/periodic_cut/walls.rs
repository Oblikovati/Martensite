// SPDX-License-Identifier: GPL-2.0-only
//! Verify each aperture wall against its unsplit or split authored boundary graph.
use super::*;

pub(super) fn check(proof: &Verification<'_>) -> Checked<()> {
    let mut walls = Vec::new();
    let mut regular = Vec::new();
    for (key, image) in &proof.direct {
        if !aperture_source(proof, key.source) {
            continue;
        }
        if key.role == HistoryRole::SideFace {
            walls.push((*key, image.clone()));
        } else {
            regular.push((*key, image.clone()));
        }
    }
    semantic::validate(&proof.prepared.aperture, &regular)?;
    trim::verify_aperture_walls(&proof.prepared.stock, &walls)?;
    for (index, &curve) in proof.hole.boundaries.iter().enumerate() {
        wall(proof, index, curve)?;
    }
    Ok(())
}

fn aperture_source(proof: &Verification<'_>, source: HistorySource) -> bool {
    match source {
        HistorySource::Region(_) => false,
        HistorySource::Curve(id) => proof.hole.boundaries.contains(&id),
        HistorySource::Vertex(id) => proof.hole.corners.contains(&id),
    }
}

fn wall(proof: &Verification<'_>, index: usize, curve: SurfaceCurveId) -> Checked<()> {
    let key = HistoryKey {
        source: HistorySource::Curve(curve),
        role: HistoryRole::SideFace,
    };
    let face = proof.resolve(PeriodicImageKey::Retained(key))?;
    semantic::check_face_carrier(&proof.prepared.aperture, key, face)?;
    one_wire(face)?;
    let expected = aperture_wall_edges(proof, index)?;
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut edges)
        .map_err(|e| native("periodic wall boundary", e))?;
    if edges.extent() == expected.len() && expected.iter().all(|image| edges.find_index(image) != 0)
    {
        return Ok(());
    }
    Err(incomplete(
        "all and only the authored aperture wall edges",
        format!(
            "{key:?}: {} native edges, expected {}",
            edges.extent(),
            expected.len()
        ),
    ))
}

fn aperture_wall_edges<'a>(
    proof: &'a Verification<'_>,
    index: usize,
) -> Checked<Vec<&'a topods::Shape>> {
    let mut edges = proof.aperture_edges(index, PeriodicDepth::First)?;
    edges.extend(proof.aperture_edges(index, PeriodicDepth::Last)?);
    for corner in [index, (index + 1) % 4] {
        edges.push(proof.retained(
            HistorySource::Vertex(proof.hole.corners[corner]),
            HistoryRole::NormalEdge,
        )?);
    }
    Ok(edges)
}

pub(super) fn one_wire(face: &topods::Shape) -> Checked<()> {
    let mut wires = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Wire, &mut wires)
        .map_err(|e| native("periodic face wires", e))?;
    if wires.extent() != 1 {
        return Err(incomplete(
            "one connected seam-cut wire",
            format!("{} wires", wires.extent()),
        ));
    }
    Ok(())
}
