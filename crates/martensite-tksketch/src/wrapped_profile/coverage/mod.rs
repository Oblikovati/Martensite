//! Exact coverage of the frozen single-region extrusion graph (#418).
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::collections::BTreeMap;
mod edges;
mod error;
mod expected;
mod faces;
mod members;
mod registry;
mod representations;
use error::{Checked, count, fail, native};
pub(super) use error::{CoverageCause, CoverageFailure};
use faces::FaceEvidence;
use registry::Registry;
#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BoundUse {
    pub key: BindingKey,
    pub orientation: Orientation,
}
impl BoundUse {
    fn new(key: BindingKey, orientation: Orientation) -> Self {
        Self { key, orientation }
    }
    fn forward(key: BindingKey) -> Self {
        Self::new(key, Orientation::Forward)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CoverageEvidence {
    pub binding_count: usize,
    pub faces: usize,
    pub wires: usize,
    pub edges: usize,
    pub vertices: usize,
    pub edge_uses: usize,
    pub euler_characteristic: i64,
}

pub(super) fn certify(
    request: &Request,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<CoverageEvidence> {
    let registry = Registry::build(request, bindings)?;
    let owner = expected::region(request, HistoryRole::Solid);
    if !root.is_equal(registry.shape(owner)?) {
        return Err(fail(owner, CoverageCause::RootBinding));
    }
    let expected_faces = expected::faces(request);
    let shell = shell(owner, root)?;
    members::uses(owner, &shell, ShapeEnum::Face, &expected_faces, &registry)?;
    let faces = check_faces(request, &registry)?;
    let edge_faces = edges::incidences(&faces)?;
    edges::endpoints(request, &registry)?;
    representations::check(&registry, &edge_faces)?;
    evidence(&registry, &faces, &edge_faces)
}

fn shell(owner: BindingKey, root: &Shape) -> Checked<Shape> {
    registry::canonical(owner, root)?;
    registry::kind_check(owner, root, ShapeEnum::Solid)?;
    let shells = members::children(owner, root, ShapeEnum::Shell, 1)?;
    count(owner, "solid shells", shells.len(), 1)?;
    let shell = shells.into_iter().next().ok_or_else(|| {
        fail(
            owner,
            CoverageCause::Count {
                role: "solid shells",
                actual: 0,
                expected: 1,
            },
        )
    })?;
    registry::canonical(owner, &shell)?;
    members::closed(owner, &shell)?;
    Ok(shell)
}
fn check_faces(request: &Request, registry: &Registry<'_>) -> Checked<Vec<FaceEvidence>> {
    let mut result = cap_faces(request, registry)?;
    for piece in request.loops.iter().flatten() {
        let owner = expected::span(piece.span, HistoryRole::SideFace);
        result.push(faces::check(
            owner,
            &[expected::side(piece)],
            Orientation::Forward,
            registry,
        )?);
    }
    Ok(result)
}
const CAP_USES: &[(HistoryRole, HistoryRole, Orientation)] = &[
    (
        HistoryRole::FirstDepthFace,
        HistoryRole::FirstDepthEdge,
        Orientation::Reversed,
    ),
    (
        HistoryRole::LastDepthFace,
        HistoryRole::LastDepthEdge,
        Orientation::Forward,
    ),
];

fn cap_faces(request: &Request, registry: &Registry<'_>) -> Checked<Vec<FaceEvidence>> {
    let mut result = Vec::new();
    for &(face, edge, orientation) in CAP_USES {
        result.push(faces::check(
            expected::region(request, face),
            &expected::cap(request, edge),
            orientation,
            registry,
        )?);
    }
    Ok(result)
}

fn evidence(
    registry: &Registry<'_>,
    faces: &[FaceEvidence],
    edges: &edges::EdgeFaces,
) -> Checked<CoverageEvidence> {
    let wires = faces.iter().map(|face| face.wire_count).sum();
    let vertices = registry.count_kind(ShapeEnum::Vertex)?;
    let face_euler: i64 = faces.iter().map(|face| 2 - face.wire_count as i64).sum();
    Ok(CoverageEvidence {
        binding_count: registry.len(),
        faces: faces.len(),
        wires,
        edges: edges.len(),
        vertices,
        edge_uses: faces.iter().map(|face| face.edges.len()).sum(),
        euler_characteristic: vertices as i64 - edges.len() as i64 + face_euler,
    })
}
