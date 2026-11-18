//! Exact coverage of the frozen single-region extrusion graph (#418).
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::collections::BTreeMap;
mod edges;
mod error;
mod expected;
mod faces;
mod family;
pub(super) use family::{FamilyCoverage, MemberView, certify_developed_family};
mod members;
mod occurrences;
pub(super) use occurrences::Occurrences;
use operation::topology::DevelopedMapHandedness;
mod registry;
mod representations;
use error::{Checked, count, fail, native};
pub(super) use error::{CoverageCause, CoverageFailure, CoverageOwner};
use faces::FaceEvidence;
use registry::Registry;
#[cfg(test)]
mod handedness_tests;
#[cfg(test)]
mod occurrence_index_tests;
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
    certify_developed(request, root, bindings)
}

pub(super) fn certify_developed(
    request: &DevelopedRegionRequest,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<CoverageEvidence> {
    inspect(request, root, bindings, DevelopedMapHandedness::Preserving).map(|(report, _)| report)
}
pub(super) fn certify_developed_oriented(
    request: &DevelopedRegionRequest,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
    handedness: DevelopedMapHandedness,
) -> Checked<(CoverageEvidence, Occurrences)> {
    let (report, faces) = inspect(request, root, bindings, handedness)?;
    Ok((report, Occurrences::new(faces)?))
}
fn inspect(
    request: &DevelopedRegionRequest,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
    handedness: DevelopedMapHandedness,
) -> Checked<(CoverageEvidence, Vec<FaceEvidence>)> {
    let registry = Registry::build(request, bindings)?;
    let faces = root_faces(request, root, &registry, handedness)?;
    let edge_faces = edges::incidences(&faces)?;
    edges::endpoints(request, &registry)?;
    representations::check(&registry, &edge_faces)?;
    let report = evidence(&registry, &faces, &edge_faces)?;
    Ok((report, faces))
}

fn root_faces(
    request: &DevelopedRegionRequest,
    root: &Shape,
    registry: &Registry<'_>,
    handedness: DevelopedMapHandedness,
) -> Checked<Vec<FaceEvidence>> {
    let owner = expected::region(request, HistoryRole::Solid);
    if !root.is_equal(registry.shape(owner)?) {
        return Err(fail(owner, CoverageCause::RootBinding));
    }
    let expected_faces = expected::faces(request, handedness);
    let shell = shell(owner, root)?;
    let mut uses = members::uses(owner, &shell, ShapeEnum::Face, &expected_faces, registry)?;
    occurrences::order_uses(&mut uses)?;
    check_faces(request, registry, handedness, &uses)
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
fn check_faces(
    request: &DevelopedRegionRequest,
    registry: &Registry<'_>,
    handedness: DevelopedMapHandedness,
    uses: &[BoundUse],
) -> Checked<Vec<FaceEvidence>> {
    let mut result = cap_faces(request, registry, handedness, uses)?;
    for piece in request.loops.iter().flatten() {
        let owner = expected::span(piece.span, HistoryRole::SideFace);
        result.push(faces::check(
            owner,
            &[expected::side(piece)],
            occurrences::face_orientation(owner, uses)?,
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

fn cap_faces(
    request: &DevelopedRegionRequest,
    registry: &Registry<'_>,
    handedness: DevelopedMapHandedness,
    uses: &[BoundUse],
) -> Checked<Vec<FaceEvidence>> {
    let mut result = Vec::new();
    for &(face, edge, _) in CAP_USES {
        result.push(faces::check(
            expected::region(request, face),
            &expected::cap(request, edge, handedness),
            occurrences::face_orientation(expected::region(request, face), uses)?,
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
