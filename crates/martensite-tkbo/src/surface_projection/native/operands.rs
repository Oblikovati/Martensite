// SPDX-License-Identifier: GPL-2.0-only
//! Separate native operand ownership and check original-to-private bindings.
use super::{Checked, Scope, SourceLine, TargetRead, poll, storage};
use crate::surface_projection::{ProjectionBoundaryBinding, target};
use martensite_tkbrep::topods::{self, Edge, Face};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::predicates::roots::metric_bounds::cylindrical_projection::ProjectionCylinder;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use std::collections::BTreeMap;

pub(super) fn copy_source(source: &Edge, original: &SourceLine) -> Checked<Edge> {
    let forward = Edge::unchecked(source.oriented(Orientation::Forward));
    let mut copier = BRepBuilderAPICopy::with_shape(&forward, true, false)?;
    let copied = topods::edge(copier.shape()?)?;
    let observed = storage::source_line(&copied)?;
    if source.tshape() == copied.tshape()
        || observed.owner == original.owner
        || observed.line.range != original.line.range
        || observed.line.line != original.line.line
        || observed.line.placement != original.line.placement
    {
        return Err(Scope::PrivateCopy.into());
    }
    let original_vertices = storage::vertices(&forward)?;
    let private_vertices = storage::vertices(&copied)?;
    if original_vertices
        .iter()
        .zip(&private_vertices)
        .any(|(a, b)| a.tshape() == b.tshape())
    {
        return Err(Scope::PrivateCopy.into());
    }
    Ok(copied)
}

pub(super) fn copy_target(
    source: &Face,
    original: &TargetRead,
    allowance: f64,
    progress: &ProgressRange,
) -> Checked<(Face, TargetRead)> {
    poll(progress)?;
    let forward = Face::unchecked(source.oriented(Orientation::Forward));
    let mut copier = BRepBuilderAPICopy::with_shape(&forward, true, false)?;
    let copied = topods::face(copier.shape()?)?;
    if source.tshape() == copied.tshape() {
        return Err(Scope::PrivateCopy.into());
    }
    let copied = Face::unchecked(copied.oriented(source.orientation()));
    let bound = mapped_boundaries(&copier, original, progress)?;
    let bindings: Vec<_> = bound
        .iter()
        .map(|(&id, edge)| ProjectionBoundaryBinding { id, edge })
        .collect();
    let observation = target::capture(&copied, &bindings, allowance, progress)?;
    if !same_target(original, &observation) {
        return Err(Scope::PrivateCopy.into());
    }
    Ok((copied, observation))
}

fn mapped_boundaries(
    copier: &BRepBuilderAPICopy,
    original: &TargetRead,
    progress: &ProgressRange,
) -> Checked<BTreeMap<crate::surface_projection::ProjectionBoundaryId, Edge>> {
    let mut bound = BTreeMap::new();
    for edge in &original.observations {
        poll(progress)?;
        if let std::collections::btree_map::Entry::Vacant(entry) = bound.entry(edge.source.id) {
            entry.insert(topods::edge(&copier.modified_shape(&edge.edge)?)?);
        }
    }
    Ok(bound)
}

fn same_target(original: &TargetRead, copy: &TargetRead) -> bool {
    original.support != copy.support
        && original.orientation == copy.orientation
        && same_cylinder(original.cylinder, copy.cylinder)
        && original.seam == copy.seam
        && original.hole_rims == copy.hole_rims
        && original.observations.len() == copy.observations.len()
        && original
            .observations
            .iter()
            .zip(&copy.observations)
            .all(|(a, b)| {
                a.source == b.source
                    && a.uv == b.uv
                    && a.seam == b.seam
                    && a.edge.tshape() != b.edge.tshape()
                    && a.spatial != b.spatial
                    && a.pcurve.pcurve != b.pcurve.pcurve
                    && a.vertices
                        .iter()
                        .zip(&b.vertices)
                        .all(|(a, b)| a.tshape() != b.tshape())
            })
}

fn same_cylinder(a: ProjectionCylinder, b: ProjectionCylinder) -> bool {
    let [a_frame, b_frame] = [a.cylinder.position(), b.cylinder.position()];
    a.placement == b.placement
        && a.axial == b.axial
        && a.hole.map(|h| (h.angle, h.axial)) == b.hole.map(|h| (h.angle, h.axial))
        && a.cylinder.radius() == b.cylinder.radius()
        && a_frame.location() == b_frame.location()
        && a_frame.x_direction() == b_frame.x_direction()
        && a_frame.y_direction() == b_frame.y_direction()
        && a_frame.direction() == b_frame.direction()
}
