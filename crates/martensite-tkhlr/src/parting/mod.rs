// SPDX-License-Identifier: GPL-2.0-only
//! Parting curves of a closed solid under a parallel pull direction.
//!
//! Native shared edges and trimmed regular cylindrical generators keep their source
//! revision and face or edge occurrence bindings. The result owns its analytic
//! carriers. The first behavior accepts planar faces and complete circular cylindrical
//! walls, with strict orthogonal cavity proofs for multi-shell solids.
//! Shared edges require a stored 3D line or circle. Cylindrical generators use the
//! existing binary64 Contap construction and the stored axial trim interval.
mod cylinder;
mod failure;
mod geometry;
mod shells;
mod topology;
mod types;

pub use failure::*;
pub use types::*;

use self::{
    failure::{Checked, native},
    geometry::{FaceGeometry, FaceSign, check_placement, direction, edge_curve},
    topology::{BoundEdge, capture_topology},
};
use martensite_tkmath::{geom_abs::SurfaceType, gp::Direction3};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape},
    brep_check::BRepCheckAnalyzer,
};
use std::{collections::HashSet, sync::atomic::Ordering};

/// Select the parting curves of one stable, located source revision.
///
/// `parting::query(source, Direction3::Z, PartingOptions::default())` includes all
/// contributing faces. A completed empty result has an empty `curves()` slice.
/// Any cancellation or unresolved required proof returns an error for the whole query.
/// Keep the native source stable until the call returns.
pub fn query(
    source: PartingSource<'_>,
    pull: Direction3,
    options: PartingOptions<'_>,
) -> Result<PartingResult, PartingFailure> {
    evaluate(source, pull, options).map_err(|kind| PartingFailure {
        source: source.scope,
        kind,
    })
}

fn evaluate(
    source: PartingSource<'_>,
    pull: Direction3,
    options: PartingOptions<'_>,
) -> Checked<PartingResult> {
    observe_stop(options)?;
    check_pull(pull)?;
    check_placement(
        source.solid.location().transformation(),
        PartingEntity::Solid,
    )?;
    let topology = capture_topology(source, options)?;
    let geometry = read_faces(&topology, options)?;
    let roles = shells::classify_shells(&topology, &geometry, options)?;
    let faces = evaluate_faces(&topology, &geometry, &roles, pull, options)?;
    validate_native(source, options)?;
    let curves = collect_curves(topology.edges, faces, options)?;
    observe_stop(options)?;
    Ok(PartingResult {
        source: source.scope,
        pull,
        curves,
    })
}

fn check_pull(pull: Direction3) -> Checked<()> {
    let components = direction(pull);
    let entity = PartingEntity::Solid;
    if let Some((axis, value)) = components
        .into_iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        let cause = martensite_tkmath::predicates::NonFiniteCoordinate {
            point: 0,
            axis,
            value,
        };
        return Err(PartingFailureKind::Coordinate { entity, cause });
    }
    if components != [0.; 3] {
        return Ok(());
    }
    Err(PartingFailureKind::Pull(components))
}

fn read_faces(
    topology: &topology::SourceTopology<'_>,
    options: PartingOptions<'_>,
) -> Checked<Vec<FaceGeometry>> {
    topology
        .faces
        .iter()
        .map(|face| {
            observe_stop(options)?;
            FaceGeometry::read(face)
        })
        .collect()
}

fn validate_native(source: PartingSource<'_>, options: PartingOptions<'_>) -> Checked<()> {
    observe_stop(options)?;
    let entity = PartingEntity::Solid;
    // The native checker wraps shells in temporary solids, which freezes those
    // shell handles. Keep those writes on operation-local topology.
    let mut copy =
        BRepBuilderAPICopy::with_shape(source.solid, false, false).map_err(native(entity))?;
    observe_stop(options)?;
    let local = copy.shape().map_err(native(entity))?;
    let analyzer = BRepCheckAnalyzer::new(local, true, false, true).map_err(native(entity))?;
    let valid = analyzer.is_valid().map_err(native(entity))?;
    observe_stop(options)?;
    if valid {
        return Ok(());
    }
    Err(topology::topology(
        entity,
        PartingTopologyProblem::NativeCheck,
    ))
}

struct EvaluatedFace {
    trace: PartingFaceUse,
    sign: FaceSign,
    excluded: bool,
    smooth: Vec<PartingCurve>,
}

fn evaluate_faces(
    topology: &topology::SourceTopology<'_>,
    geometry: &[FaceGeometry],
    roles: &[PartingShell],
    pull: Direction3,
    options: PartingOptions<'_>,
) -> Checked<Vec<EvaluatedFace>> {
    let explicit: HashSet<_> = options.exclude_faces.iter().copied().collect();
    topology
        .faces
        .iter()
        .zip(geometry)
        .map(|(face, geometry)| {
            observe_stop(options)?;
            let shell = roles[face.shell];
            let trace = PartingFaceUse {
                id: face.id,
                orientation: face.face.orientation(),
                shell,
            };
            let excluded = explicit.contains(&face.id)
                || (options.exclude_straight && geometry.kind == SurfaceType::Plane)
                || (options.exclude_internal && shell == PartingShell::Cavity);
            if excluded {
                return Ok(EvaluatedFace {
                    trace,
                    sign: FaceSign::Zero,
                    excluded,
                    smooth: Vec::new(),
                });
            }
            let (sign, smooth) = geometry.evaluate(&face.face, trace, pull)?;
            Ok(EvaluatedFace {
                trace,
                sign,
                excluded,
                smooth,
            })
        })
        .collect()
}

fn collect_curves(
    edges: Vec<BoundEdge<'_>>,
    mut faces: Vec<EvaluatedFace>,
    options: PartingOptions<'_>,
) -> Checked<Vec<PartingCurve>> {
    let mut output = Vec::new();
    for edge in edges {
        observe_stop(options)?;
        if let Some(origin) = shared_origin(&edge, &faces) {
            output.push(edge_curve(
                edge.binding.occurrence,
                edge.binding.id,
                origin,
            )?);
        }
    }
    faces.sort_by_key(|face| face.trace.id);
    for face in faces {
        observe_stop(options)?;
        output.extend(face.smooth);
    }
    Ok(output)
}

fn shared_origin(edge: &BoundEdge<'_>, faces: &[EvaluatedFace]) -> Option<PartingOrigin> {
    let [a, b] = edge.uses.map(|member| member.face);
    if a == b || faces[a].excluded || faces[b].excluded {
        return None;
    }
    let kind = shared_kind(faces[a].sign, faces[b].sign)?;
    let faces = edge.uses.map(|member| PartingEdgeFaceUse {
        face: faces[member.face].trace,
        edge_orientation: member.orientation,
    });
    Some(PartingOrigin::Shared {
        edge: edge.binding.id,
        orientation: edge.binding.occurrence.orientation(),
        faces,
        kind,
    })
}

fn shared_kind(a: FaceSign, b: FaceSign) -> Option<PartingEdgeKind> {
    use FaceSign::{Negative, Positive, Zero};
    match (a, b) {
        (Positive, Negative) | (Negative, Positive) => Some(PartingEdgeKind::Opposite),
        (Zero, Positive | Negative) | (Positive | Negative, Zero) => {
            Some(PartingEdgeKind::ZeroBandRim)
        }
        _ => None,
    }
}

fn observe_stop(options: PartingOptions<'_>) -> Checked<()> {
    if options
        .cancelled
        .is_some_and(|flag| flag.load(Ordering::Relaxed))
    {
        return Err(PartingFailureKind::Cancelled);
    }
    Ok(())
}
