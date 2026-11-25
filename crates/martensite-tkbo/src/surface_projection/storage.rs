// SPDX-License-Identifier: GPL-2.0-only
//! Bounded literal readers for original and native Section geometry.
use super::{Checked, SurfaceProjectionScope as Scope};
use martensite_tkbrep::{
    brep::{BRepCurveRepresentation, BRepTEdge, BRepTVertex, BRepTool, EdgePCurve},
    topods::{self, Edge, Face, Shape, ShapeIterator, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomCurve, GeomLine, GeomTrimmedCurve},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Direction3, Line2, Line3},
    predicates::roots::metric_bounds::cylindrical_projection::{
        AxialProjectionLine, MAX_PROJECTION_CURVE_POINTS, NativeProjectionCurve,
        ProjectionPlacement, ProjectionVertex, check_projection_line_vertices,
    },
    toploc::Location,
};

const LOCATION_STEPS: usize = 64;
const REPRESENTATION_LIMIT: usize = 16;

pub(super) struct SourceLine {
    pub line: AxialProjectionLine,
    pub owner: Handle<dyn GeomCurve>,
    pub orientation: Orientation,
}

pub(super) fn source_line(edge: &Edge) -> Checked<SourceLine> {
    kind(edge, ShapeEnum::Edge)?;
    let orientation = edge.orientation();
    reversed(orientation)?;
    let (line, owner) = original_line(edge)?;
    let edge = Edge::unchecked(edge.oriented(Orientation::Forward));
    let directed = vertices(&edge)?;
    if directed[0].is_same(&directed[1]) {
        return Err(Scope::Topology.into());
    }
    let vertices = vertex_coefficients(&directed)?;
    check_projection_line_vertices(line, vertices)?;
    Ok(SourceLine {
        line,
        owner,
        orientation,
    })
}

pub(super) fn original_line(edge: &Edge) -> Checked<(AxialProjectionLine, Handle<dyn GeomCurve>)> {
    placement(edge.location())?;
    require_spatial(edge)?;
    if BRepTool::degenerated(edge)? {
        return Err(Scope::Representation.into());
    }
    let read = BRepTool::curve_with_location(edge)?;
    let range = [read.first, read.last];
    let line = spatial_line(&read.curve, range).ok_or(Scope::Representation)?;
    Direction3::from_stored_coefficients(*line.direction().coordinates())?;
    Ok((
        AxialProjectionLine {
            line,
            placement: placement(&read.location)?,
            range,
        },
        read.curve,
    ))
}

pub(super) fn spatial_line(curve: &Handle<dyn GeomCurve>, range: [f64; 2]) -> Option<Line3> {
    if let Some(line) = curve.downcast_ref::<GeomLine>() {
        return Some(line.lin());
    }
    let trim = curve.downcast_ref::<GeomTrimmedCurve>()?;
    if !contains([trim.first_parameter(), trim.last_parameter()], range) {
        return None;
    }
    Some(trim.basis_curve().downcast_ref::<GeomLine>()?.lin())
}

pub(super) fn planar_line(curve: &Handle<dyn Geom2dCurve>, range: [f64; 2]) -> Option<Line2> {
    if let Some(line) = curve.downcast_ref::<Geom2dLine>() {
        return Some(line.lin2d());
    }
    let trim = curve.downcast_ref::<Geom2dTrimmedCurve>()?;
    if !contains([trim.first_parameter(), trim.last_parameter()], range) {
        return None;
    }
    Some(trim.basis_curve().downcast_ref::<Geom2dLine>()?.lin2d())
}

fn contains(outer: [f64; 2], inner: [f64; 2]) -> bool {
    outer.into_iter().all(f64::is_finite) && outer[0] <= inner[0] && inner[1] <= outer[1]
}

pub(super) fn pcurve(edge: &Edge, face: &Face) -> Checked<(EdgePCurve, bool)> {
    let (support, location) = BRepTool::surface_with_location(face)?;
    let relative = location.predivided(edge.location())?;
    let records = records(edge)?;
    let mut matches = records
        .iter()
        .filter_map(Handle::get)
        .filter(|record| record.is_curve_on_surface_at(&support, &relative));
    let matched = matches.next().ok_or(Scope::Representation)?;
    if matches.next().is_some() {
        return Err(Scope::Representation.into());
    }
    let proof = BRepTool::curve_on_face(edge, face)?;
    if !proof.is_stored || proof.pcurve.is_null() {
        return Err(Scope::Representation.into());
    }
    Ok((proof, matched.is_curve_on_closed_surface()))
}

pub(super) fn require_spatial(edge: &Edge) -> Checked<()> {
    let actual = records(edge)?
        .iter()
        .filter_map(Handle::get)
        .filter(|repr| repr.is_curve3d())
        .count();
    if actual != 1 {
        return Err(Scope::Representation.into());
    }
    Ok(())
}

fn records(edge: &Edge) -> Checked<Vec<Handle<dyn BRepCurveRepresentation>>> {
    let native = edge.tshape().downcast::<BRepTEdge>();
    let stored = native.get().ok_or(Scope::Representation)?;
    if !stored.tolerance().is_finite() || stored.tolerance() < 0. {
        return Err(Scope::Representation.into());
    }
    let records = stored
        .curves_within_limit(REPRESENTATION_LIMIT)
        .map_err(|count| Scope::CurveRepresentations {
            count,
            maximum: REPRESENTATION_LIMIT,
        })?;
    for record in &records {
        let repr = record.get().ok_or(Scope::Representation)?;
        placement(&repr.location())?;
    }
    Ok(records)
}

pub(super) fn vertex_coefficients(vertices: &[Vertex; 2]) -> Checked<[ProjectionVertex; 2]> {
    let read = |vertex: &Vertex| -> Checked<_> {
        let owner = vertex.tshape().downcast::<BRepTVertex>();
        let vertex_geometry = owner.get().ok_or(Scope::Representation)?;
        if !vertex_geometry.tolerance().is_finite() || vertex_geometry.tolerance() < 0. {
            return Err(Scope::Representation.into());
        }
        Ok(ProjectionVertex {
            point: *vertex_geometry.pnt().coordinates(),
            placement: placement(vertex.location())?,
        })
    };
    Ok([read(&vertices[0])?, read(&vertices[1])?])
}

pub(super) fn vertices(edge: &Edge) -> Checked<[Vertex; 2]> {
    let children = direct(edge, ShapeEnum::Vertex, [2, 2])?;
    let [a, b]: [Shape; 2] = children.try_into().map_err(|_| Scope::Topology)?;
    let [a_reversed, b_reversed] = [reversed(a.orientation())?, reversed(b.orientation())?];
    if a_reversed == b_reversed {
        return Err(Scope::Topology.into());
    }
    let (a, b) = if a_reversed { (b, a) } else { (a, b) };
    Ok([topods::vertex(&a)?, topods::vertex(&b)?])
}

pub(super) fn direct(shape: &Shape, child: ShapeEnum, [lo, hi]: [usize; 2]) -> Checked<Vec<Shape>> {
    if !(lo..=hi).contains(&(shape.nb_children() as usize)) {
        return Err(Scope::Topology.into());
    }
    // Original child datums also matter when effective composition erases a shortcut.
    for raw in ShapeIterator::new(shape, false, false) {
        placement(raw.location())?;
    }
    ShapeIterator::new(shape, true, true)
        .map(|item| {
            kind(&item, child)?;
            Ok(item)
        })
        .collect()
}

pub(super) fn kind(shape: &Shape, expected: ShapeEnum) -> Checked<()> {
    let actual = shape.shape_type()?;
    if actual != expected {
        return Err(Scope::Shape { actual, expected }.into());
    }
    Ok(())
}

pub(super) fn reversed(orientation: Orientation) -> Checked<bool> {
    match orientation {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        value => Err(Scope::Orientation(value).into()),
    }
}

pub(super) fn placement(location: &Location) -> Checked<ProjectionPlacement> {
    let mut cursor = location;
    for count in 0..=LOCATION_STEPS {
        if cursor.is_identity() {
            return Ok(location.transformation().checked_unit_placement_rows()?);
        }
        if count == LOCATION_STEPS {
            break;
        }
        let datum = cursor.first_datum()?.get().ok_or(Scope::Representation)?;
        datum.transformation().checked_unit_placement_rows()?;
        cursor = cursor.next_location();
    }
    Err(Scope::LocationDepth.into())
}

pub(super) fn curve3d(
    curve: &Handle<dyn GeomCurve>,
    range: [f64; 2],
) -> Checked<NativeProjectionCurve<3>> {
    if let Some(line) = spatial_line(curve, range) {
        return Ok(NativeProjectionCurve::Line {
            origin: *line.location().coordinates(),
            direction: *line.direction().coordinates(),
        });
    }
    let spline = curve
        .downcast_ref::<GeomBSplineCurve>()
        .ok_or(Scope::Representation)?;
    if spline.nb_poles() as usize > MAX_PROJECTION_CURVE_POINTS {
        return Err(Scope::Representation.into());
    }
    let lease = spline
        .try_input_lease()
        .map_err(|_| Scope::Representation)?;
    if lease
        .stored_sizes()
        .into_iter()
        .any(|count| count > MAX_PROJECTION_CURVE_POINTS + 2)
    {
        return Err(Scope::Representation.into());
    }
    let stored = lease.stored_input();
    if stored.degree() != 1
        || stored.periodic()
        || stored.rational()
        || stored.evaluation_override()
    {
        return Err(Scope::Representation.into());
    }
    validate_linear_knots(
        stored.knots(),
        stored.multiplicities(),
        stored.flat_knots(),
        stored.poles().len(),
    )?;
    Ok(NativeProjectionCurve::DegreeOne {
        knots: stored.knots().to_vec(),
        poles: stored.poles().to_vec(),
    })
}

pub(super) fn curve2d(
    curve: &Handle<dyn Geom2dCurve>,
    range: [f64; 2],
) -> Checked<NativeProjectionCurve<2>> {
    if let Some(line) = planar_line(curve, range) {
        return Ok(NativeProjectionCurve::Line {
            origin: [line.location().x(), line.location().y()],
            direction: [line.direction().x(), line.direction().y()],
        });
    }
    let spline = curve
        .downcast_ref::<Geom2dBSplineCurve>()
        .ok_or(Scope::Representation)?;
    if spline.nb_poles() as usize > MAX_PROJECTION_CURVE_POINTS {
        return Err(Scope::Representation.into());
    }
    let lease = spline
        .try_input_lease()
        .map_err(|_| Scope::Representation)?;
    if lease
        .stored_sizes()
        .into_iter()
        .any(|count| count > MAX_PROJECTION_CURVE_POINTS + 2)
    {
        return Err(Scope::Representation.into());
    }
    let stored = lease.stored_input();
    if stored.degree() != 1
        || stored.periodic()
        || stored.rational()
        || stored.evaluation_override()
    {
        return Err(Scope::Representation.into());
    }
    validate_linear_knots(
        stored.knots(),
        stored.multiplicities(),
        stored.flat_knots(),
        stored.poles().len(),
    )?;
    Ok(NativeProjectionCurve::DegreeOne {
        knots: stored.knots().to_vec(),
        poles: stored.poles().to_vec(),
    })
}

fn validate_linear_knots(
    knots: &[f64],
    multiplicities: &[i32],
    flat: &[f64],
    count: usize,
) -> Checked<()> {
    if !(2..=MAX_PROJECTION_CURVE_POINTS).contains(&count)
        || knots.len() != count
        || multiplicities.len() != count
        || flat.len() != count + 2
        || !matches!(multiplicities, [2, inner @ .., 2] if inner.iter().all(|value| *value == 1))
        || flat[0] != knots[0]
        || flat[count + 1] != knots[count - 1]
        || flat[1..=count] != *knots
        || !knots.iter().all(|value| value.is_finite())
        || knots.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(Scope::Representation.into());
    }
    Ok(())
}
