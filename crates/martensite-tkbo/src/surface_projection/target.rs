// SPDX-License-Identifier: GPL-2.0-only
//! Original directed trim, periodic seam, and rectangular hole evidence.
use super::{
    Checked, ProjectionBoundaryBinding, ProjectionBoundaryUse, SurfaceProjectionScope as Scope,
    poll, storage,
};
use martensite_tkbrep::{
    brep::{BRepTFace, BRepTool, EdgePCurve},
    topods::{self, Edge, Face, Shape, Vertex},
};
use martensite_tkernel::{message::ProgressRange, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCurve, GeomCylindricalSurface, GeomSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Axis3, Cylinder3, Line2},
    predicates::roots::metric_bounds::{
        cylindrical_projection::{ProjectionCylinder, ProjectionHole, check_projection_meridian},
        exact_affine_coordinate,
    },
};
use std::f64::consts::TAU;

mod rectangle;
use rectangle::{Rectangle, Side};

pub(super) struct TargetRead {
    pub cylinder: ProjectionCylinder,
    pub orientation: Orientation,
    pub seam: [ProjectionBoundaryUse; 2],
    pub hole_rims: Option<[ProjectionBoundaryUse; 2]>,
    pub boundary_error: f64,
    pub support: Handle<dyn GeomSurface>,
    pub seam_edge: Edge,
    pub observations: Vec<BoundaryObservation>,
}

pub(super) struct BoundaryObservation {
    pub source: ProjectionBoundaryUse,
    pub edge: Edge,
    pub vertices: [Vertex; 2],
    pub uv: [[f64; 2]; 2],
    pub pcurve: EdgePCurve,
    pub spatial: Handle<dyn GeomCurve>,
    pub seam: bool,
}

pub(super) fn capture(
    selected_face: &Face,
    bindings: &[ProjectionBoundaryBinding<'_>],
    allowance: f64,
    progress: &ProgressRange,
) -> Checked<TargetRead> {
    poll(progress)?;
    storage::kind(selected_face, ShapeEnum::Face)?;
    let orientation = selected_face.orientation();
    storage::reversed(orientation)?;
    storage::placement(selected_face.location())?;
    let face = Face::unchecked(selected_face.oriented(Orientation::Forward));
    let native = face.tshape().downcast::<BRepTFace>();
    let stored = native.get().ok_or(Scope::Representation)?;
    if !stored.tolerance().is_finite() || stored.tolerance() < 0. {
        return Err(Scope::Representation.into());
    }
    storage::placement(&stored.location())?;
    let (support, location) = BRepTool::surface_with_location(&face)?;
    let cylinder = support
        .downcast_ref::<GeomCylindricalSurface>()
        .ok_or(Scope::Representation)?
        .cylinder()?;
    let placement = storage::placement(&location)?;
    let (outer, hole) = rectangles(&face, bindings, progress)?;
    finish(
        orientation,
        support,
        ProjectionCylinder {
            cylinder,
            placement,
            axial: outer.axial,
            hole: hole.as_ref().map(Rectangle::as_hole),
        },
        outer,
        hole,
        allowance,
        progress,
    )
}

fn rectangles(
    face: &Face,
    bindings: &[ProjectionBoundaryBinding<'_>],
    progress: &ProgressRange,
) -> Checked<(Rectangle, Option<Rectangle>)> {
    let direct = storage::direct(face, ShapeEnum::Wire, [1, 2])?;
    let mut outer = None;
    let mut hole = None;
    for wire in direct {
        poll(progress)?;
        let ring = read_wire(&wire, face, bindings, progress)?;
        let rectangle = Rectangle::capture(ring)?;
        let slot = if rectangle.angular == [0., TAU] {
            &mut outer
        } else {
            &mut hole
        };
        if slot.replace(rectangle).is_some() {
            return Err(Scope::Topology.into());
        }
    }
    let outer = outer.ok_or(Scope::Topology)?;
    require_bindings(&outer, hole.as_ref(), bindings)?;
    Ok((outer, hole))
}

fn require_bindings(
    outer: &Rectangle,
    hole: Option<&Rectangle>,
    bindings: &[ProjectionBoundaryBinding<'_>],
) -> Checked<()> {
    let uses: Vec<_> = outer
        .uses
        .iter()
        .chain(hole.into_iter().flat_map(|ring| &ring.uses))
        .collect();
    if bindings.len() != uses.len() - 1 {
        return Err(Scope::BoundaryIdentity.into());
    }
    for (index, binding) in bindings.iter().enumerate() {
        if !uses.iter().any(|side| side.edge.is_same(binding.edge))
            || bindings[..index]
                .iter()
                .any(|prior| prior.id == binding.id || prior.edge.is_same(binding.edge))
        {
            return Err(Scope::BoundaryIdentity.into());
        }
    }
    Ok(())
}

fn read_wire(
    wire: &Shape,
    face: &Face,
    bindings: &[ProjectionBoundaryBinding<'_>],
    progress: &ProgressRange,
) -> Checked<Vec<BoundaryObservation>> {
    storage::reversed(wire.orientation())?;
    if !wire.closed()? {
        return Err(Scope::Topology.into());
    }
    let mut observed = Vec::with_capacity(4);
    for member in storage::direct(wire, ShapeEnum::Edge, [4, 4])? {
        poll(progress)?;
        observed.push(read_boundary(topods::edge(&member)?, face, bindings)?);
    }
    Ok(observed)
}

fn read_boundary(
    edge: Edge,
    face: &Face,
    bindings: &[ProjectionBoundaryBinding<'_>],
) -> Checked<BoundaryObservation> {
    let binding = bindings
        .iter()
        .find(|binding| edge.is_same(binding.edge))
        .ok_or(Scope::BoundaryIdentity)?;
    let orientation = edge.orientation();
    let reversed = storage::reversed(orientation)?;
    let vertices = storage::vertices(&edge)?;
    let (pcurve, seam) = storage::pcurve(&edge, face)?;
    let mut uv = line_uv(&pcurve)?;
    if reversed {
        uv.swap(0, 1);
    }
    storage::require_spatial(&edge)?;
    let spatial = BRepTool::curve_with_location(&edge)?.curve;
    Ok(BoundaryObservation {
        source: ProjectionBoundaryUse {
            id: binding.id,
            orientation,
        },
        edge,
        vertices,
        uv,
        pcurve,
        spatial,
        seam,
    })
}

pub(super) fn line_uv(pcurve: &EdgePCurve) -> Checked<[[f64; 2]; 2]> {
    let range = [pcurve.first, pcurve.last];
    let line = storage::planar_line(&pcurve.pcurve, range).ok_or(Scope::Representation)?;
    if !range.into_iter().all(f64::is_finite) || range[0] >= range[1] {
        return Err(Scope::Representation.into());
    }
    Ok([
        pcurve_endpoint(line, range[0])?,
        pcurve_endpoint(line, range[1])?,
    ])
}

fn pcurve_endpoint(line: Line2, parameter: f64) -> Checked<[f64; 2]> {
    let origin = line.location();
    let direction = line.direction();
    let coordinate = |a, b| -> Checked<f64> {
        exact_affine_coordinate(a, b, parameter)
            .map_err(|_| Scope::Representation)?
            .ok_or_else(|| Scope::Representation.into())
    };
    Ok([
        coordinate(origin.x(), direction.x())?,
        coordinate(origin.y(), direction.y())?,
    ])
}

fn finish(
    orientation: Orientation,
    support: Handle<dyn GeomSurface>,
    cylinder: ProjectionCylinder,
    outer: Rectangle,
    hole: Option<Rectangle>,
    allowance: f64,
    progress: &ProgressRange,
) -> Checked<TargetRead> {
    let [lower, upper] = [
        &outer.uses[Side::LowerU as usize],
        &outer.uses[Side::UpperU as usize],
    ];
    if !outer.forward
        || !lower.edge.is_same(&upper.edge)
        || lower.source.orientation == upper.source.orientation
        || !lower.seam
        || !upper.seam
    {
        return Err(Scope::Topology.into());
    }
    let seam = [lower.source, upper.source];
    let seam_edge = lower.edge.clone();
    let mut boundary_error = verify_rectangle(cylinder, &outer, allowance, progress)?;
    let hole_rims = verify_hole(
        cylinder,
        hole.as_ref(),
        allowance,
        &mut boundary_error,
        progress,
    )?;
    let observations = outer
        .uses
        .into_iter()
        .chain(hole.into_iter().flat_map(|ring| ring.uses))
        .collect();
    Ok(TargetRead {
        cylinder,
        orientation,
        seam,
        hole_rims,
        boundary_error,
        support,
        seam_edge,
        observations,
    })
}

fn verify_hole(
    cylinder: ProjectionCylinder,
    hole: Option<&Rectangle>,
    allowance: f64,
    upper: &mut f64,
    progress: &ProgressRange,
) -> Checked<Option<[ProjectionBoundaryUse; 2]>> {
    let Some(hole) = hole else {
        return Ok(None);
    };
    if hole.forward
        || hole.uses.iter().any(|edge| edge.seam)
        || hole.angular[0] <= 0.
        || hole.angular[1] >= TAU
        || hole.axial[0] <= cylinder.axial[0]
        || hole.axial[1] >= cylinder.axial[1]
    {
        return Err(Scope::Topology.into());
    }
    *upper = upper.max(verify_rectangle(cylinder, hole, allowance, progress)?);
    Ok(Some([
        hole.uses[Side::LowerV as usize].source,
        hole.uses[Side::UpperV as usize].source,
    ]))
}

fn verify_rectangle(
    cylinder: ProjectionCylinder,
    ring: &Rectangle,
    allowance: f64,
    progress: &ProgressRange,
) -> Checked<f64> {
    let mut upper: f64 = 0.;
    for (index, side) in ring.uses.iter().enumerate() {
        poll(progress)?;
        if index < 2 {
            upper = upper.max(verify_meridian(cylinder, side, ring.axial, allowance)?);
        } else {
            verify_circle(cylinder.cylinder, cylinder.placement, side, ring.angular)?;
        }
    }
    Ok(upper)
}

fn verify_meridian(
    cylinder: ProjectionCylinder,
    side: &BoundaryObservation,
    range: [f64; 2],
    cap: f64,
) -> Checked<f64> {
    let (line, _) = storage::original_line(&side.edge)?;
    if line.range != range
        || line_uv(&side.pcurve)? != [[side.uv[0][0], range[0]], [side.uv[0][0], range[1]]]
    {
        return Err(Scope::Representation.into());
    }
    let edge = Edge::unchecked(side.edge.oriented(Orientation::Forward));
    let vertices = storage::vertex_coefficients(&storage::vertices(&edge)?)?;
    Ok(check_projection_meridian(
        cylinder,
        line,
        side.uv[0][0],
        vertices,
        cap,
    )?)
}

fn verify_circle(
    cylinder: Cylinder3,
    placement: [[f64; 4]; 3],
    side: &BoundaryObservation,
    angles: [f64; 2],
) -> Checked<()> {
    let actual = BRepTool::curve_with_location(&side.edge)?;
    let circle = actual
        .curve
        .downcast_ref::<GeomCircle>()
        .ok_or(Scope::Representation)?
        .circ()?;
    let frame = cylinder.position();
    let circle_frame = circle.position();
    Axis3::from_stored_coefficients(
        circle_frame.location(),
        *circle_frame.direction().coordinates(),
        *circle_frame.x_direction().coordinates(),
        *circle_frame.y_direction().coordinates(),
    )?;
    let v = side.uv[0][1];
    if circle.radius() != cylinder.radius()
        || circle.position().x_direction() != frame.x_direction()
        || circle.position().y_direction() != frame.y_direction()
        || !local_axial_point(frame.location(), frame.direction(), v, circle.location())?
        || storage::placement(&actual.location)? != placement
        || [actual.first, actual.last] != angles
        || line_uv(&side.pcurve)? != [[angles[0], v], [angles[1], v]]
        || side.seam
    {
        return Err(Scope::Representation.into());
    }
    Ok(())
}

fn local_axial_point(
    origin: martensite_tkmath::gp::Point3,
    direction: martensite_tkmath::gp::Direction3,
    v: f64,
    actual: martensite_tkmath::gp::Point3,
) -> Checked<bool> {
    for ((&origin, &slope), &observed) in origin
        .coordinates()
        .iter()
        .zip(direction.coordinates())
        .zip(actual.coordinates())
    {
        if exact_affine_coordinate(origin, slope, v).map_err(|_| Scope::Representation)?
            != Some(observed)
        {
            return Ok(false);
        }
    }
    Ok(true)
}
