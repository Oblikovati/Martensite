// SPDX-License-Identifier: GPL-2.0-only
//! Verify each aperture wall against its unsplit or split authored boundary graph.
use super::*;

pub(super) fn check(proof: &Verification<'_>) -> Checked<()> {
    for hole in &proof.prepared.holes {
        check_aperture(proof, hole)?;
    }
    Ok(())
}

fn check_aperture(proof: &Verification<'_>, hole: &ApprovedHole) -> Checked<()> {
    let regular = proof
        .direct
        .iter()
        .filter(|(key, _)| {
            key.role != HistoryRole::SideFace && aperture_source(&hole.source, key.source)
        })
        .cloned()
        .collect();
    hole.admitted.check_boundary(&regular)?;
    for (index, &curve) in hole.source.boundaries.iter().enumerate() {
        wall(proof, hole, index, curve)?;
    }
    Ok(())
}

fn aperture_source(hole: &Contour, source: HistorySource) -> bool {
    match source {
        HistorySource::Region(_) => false,
        HistorySource::Curve(id) => hole.boundaries.contains(&id),
        HistorySource::Vertex(id) => hole.corners.contains(&id),
    }
}

fn wall(
    proof: &Verification<'_>,
    hole: &ApprovedHole,
    index: usize,
    curve: SurfaceCurveId,
) -> Checked<()> {
    let key = HistoryKey {
        source: HistorySource::Curve(curve),
        role: HistoryRole::SideFace,
    };
    let face = proof.resolve(PeriodicImageKey::Retained(key))?;
    hole.admitted.check_wall(key, face)?;
    let stock = &proof.prepared.stock;
    let carrier = if !stock.cylinder && index.is_multiple_of(2) {
        SurfaceType::Cone
    } else {
        SurfaceType::Plane
    };
    trim::verify_aperture_wall(stock, key, face, carrier)?;
    one_wire(face)?;
    let expected = aperture_wall_edges(proof, hole, index)?;
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
    hole: &ApprovedHole,
    index: usize,
) -> Checked<Vec<&'a topods::Shape>> {
    let mut edges = proof.aperture_edges(hole, index, PeriodicDepth::First)?;
    edges.extend(proof.aperture_edges(hole, index, PeriodicDepth::Last)?);
    for corner in [index, (index + 1) % 4] {
        edges.push(proof.retained(
            HistorySource::Vertex(hole.source.corners[corner]),
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
