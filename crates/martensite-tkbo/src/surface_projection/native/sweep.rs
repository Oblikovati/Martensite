// SPDX-License-Identifier: GPL-2.0-only
//! Actual canonized finite sweep with a retained planar UV chart.
use super::{
    Checked, ProjectionPrismChart, ProjectionSweep, Scope, materialize_planar_pcurve, poll, storage,
};
use crate::surface_projection::target::line_uv;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Face, Shape},
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::{geom::GeomPlane, top_abs::ShapeEnum};
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_sweep::BRepSweepPrism;

pub(super) fn build(
    source: &Edge,
    sweep: ProjectionSweep,
    progress: &ProgressRange,
) -> Checked<(Face, ProjectionPrismChart)> {
    poll(progress)?;
    let generator = start_edge(source, sweep)?;
    let vector = Vector3::from(sweep.direction) * sweep.length;
    let mut prism = BRepSweepPrism::new(&generator, vector, true, true)?;
    let raw = prism.shape()?;
    if !raw.is_same(&prism.shape_of(&generator)?) {
        return Err(Scope::Ancestry(2).into());
    }
    let face = topods::face(&raw)?;
    let chart = capture(&face)?;
    poll(progress)?;
    Ok((face, chart))
}

fn start_edge(source: &Edge, sweep: ProjectionSweep) -> Checked<Shape> {
    if sweep.start == 0. {
        return Ok(source.clone().into_shape());
    }
    let shift = Vector3::from(sweep.direction) * sweep.start;
    let location = Location::new(Transform3::translation_vector(shift));
    Ok(source.moved(&location, false)?)
}

fn capture(face: &Face) -> Checked<ProjectionPrismChart> {
    let (surface, location) = BRepTool::surface_with_location(face)?;
    let frame = surface
        .downcast_ref::<GeomPlane>()
        .ok_or(Scope::Representation)?
        .pln()
        .position();
    let axes = [frame.x_direction(), frame.y_direction(), frame.direction()];
    Ok(ProjectionPrismChart {
        origin: *frame.location().coordinates(),
        axes: axes.map(|axis| *axis.coordinates()),
        placement: storage::placement(&location)?,
        trim: rectangle(face)?,
    })
}

fn rectangle(face: &Face) -> Checked<[[f64; 2]; 2]> {
    let mut wires = storage::direct(face, ShapeEnum::Wire, [1, 1])?;
    let wire = wires.pop().ok_or(Scope::Topology)?;
    let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    let mut sides = Vec::with_capacity(4);
    for raw in storage::direct(&wire, ShapeEnum::Edge, [4, 4])? {
        let edge = topods::edge(&raw)?;
        storage::require_spatial(&edge)?;
        materialize_planar_pcurve(&edge, face)?;
        let (pcurve, seam) = storage::pcurve(&edge, face)?;
        if seam {
            return Err(Scope::Topology.into());
        }
        let coordinates = line_uv(&pcurve)?;
        union_bounds(&mut bounds, coordinates);
        sides.push(coordinates);
    }
    let mut sides_seen = [false; 4];
    for [a, b] in sides {
        let index = side([a, b], bounds).ok_or(Scope::Topology)?;
        if std::mem::replace(&mut sides_seen[index], true) {
            return Err(Scope::Topology.into());
        }
    }
    Ok(bounds)
}

fn union_bounds(bounds: &mut [[f64; 2]; 2], coordinates: [[f64; 2]; 2]) {
    for point in coordinates {
        for (axis, value) in point.into_iter().enumerate() {
            bounds[axis][0] = bounds[axis][0].min(value);
            bounds[axis][1] = bounds[axis][1].max(value);
        }
    }
}

fn side([a, b]: [[f64; 2]; 2], bounds: [[f64; 2]; 2]) -> Option<usize> {
    (0..2).find_map(|axis| {
        let along = 1 - axis;
        if a[axis] != b[axis] || [a[along].min(b[along]), a[along].max(b[along])] != bounds[along] {
            return None;
        }
        bounds[axis]
            .iter()
            .position(|value| *value == a[axis])
            .map(|index| axis * 2 + index)
    })
}
