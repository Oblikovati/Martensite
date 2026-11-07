//! Tie decoded carriers and trims to the admitted rectangle through semantic keys.
use super::{admission::Admitted, failure::*, frame, history::*};
use martensite_cmath::{atan2, cos, hypot, sin};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    topods,
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::{
    geom_abs::{CurveType, SurfaceType},
    gp::Point3,
};

type EdgeSamples = ([f64; 3], [f64; 3], [f64; 3], bool);

pub(super) fn validate(admitted: &Admitted, bindings: &Bindings) -> Checked<()> {
    for (key, shape) in bindings {
        match role_kind(key.role) {
            ShapeEnum::Vertex => check_vertex(admitted, *key, shape)?,
            ShapeEnum::Edge => check_edge(admitted, *key, shape)?,
            ShapeEnum::Face => check_face(admitted, *key, shape, bindings)?,
            _ => {}
        }
    }
    Ok(())
}
pub(super) fn close(
    actual: f64,
    expected: f64,
    tolerance: f64,
    context: &'static str,
) -> Checked<()> {
    if !actual.is_finite() || !expected.is_finite() || (actual - expected).abs() > tolerance {
        return Err(incomplete(
            context,
            format!("actual {actual}, expected {expected} within {tolerance}"),
        ));
    }
    Ok(())
}
pub(super) fn distance(point: Point3, expected: [f64; 3]) -> f64 {
    let delta = [
        point.x() - expected[0],
        point.y() - expected[1],
        point.z() - expected[2],
    ];
    frame::dot(delta, delta).sqrt()
}
fn tolerance(admitted: &Admitted) -> f64 {
    admitted.options.resolution.weld + admitted.coordinate_error
}
fn corner_index(admitted: &Admitted, source: HistorySource) -> Checked<usize> {
    admitted
        .rectangle
        .corners
        .iter()
        .position(|id| source == HistorySource::Vertex(*id))
        .ok_or_else(|| incomplete("authored corner", format!("unexpected source {source:?}")))
}
fn curve_index(admitted: &Admitted, source: HistorySource) -> Checked<usize> {
    admitted
        .rectangle
        .boundaries
        .iter()
        .position(|id| source == HistorySource::Curve(*id))
        .ok_or_else(|| incomplete("authored curve", format!("unexpected source {source:?}")))
}
fn corner(admitted: &Admitted, corner: usize, depth: usize) -> [f64; 3] {
    let s = admitted.rectangle.s[usize::from(corner == 1 || corner == 2)];
    let z = admitted.rectangle.z[usize::from(corner >= 2)];
    let angle = admitted.rectangle.support.chart_origin + s / admitted.rectangle.support.radius;
    frame::point(&admitted.rectangle.support, angle, admitted.radii[depth], z)
}
fn check_vertex(admitted: &Admitted, key: HistoryKey, shape: &topods::Shape) -> Checked<()> {
    let index = corner_index(admitted, key.source)?;
    let depth = usize::from(key.role == HistoryRole::LastDepthVertex);
    let vertex = topods::vertex(shape).map_err(|error| native("semantic vertex", error))?;
    let actual = BRepTool::pnt(&vertex).map_err(|error| native("semantic point", error))?;
    close(
        distance(actual, corner(admitted, index, depth)),
        0.,
        tolerance(admitted),
        "corner image",
    )
}
fn edge_points(admitted: &Admitted, key: HistoryKey) -> Checked<EdgeSamples> {
    if key.role == HistoryRole::NormalEdge {
        let index = corner_index(admitted, key.source)?;
        let first = corner(admitted, index, 0);
        let last = corner(admitted, index, 1);
        return Ok((
            first,
            last,
            std::array::from_fn(|i| (first[i] + last[i]) / 2.),
            false,
        ));
    }
    boundary_points(admitted, key)
}
fn boundary_points(admitted: &Admitted, key: HistoryKey) -> Checked<EdgeSamples> {
    let index = curve_index(admitted, key.source)?;
    let depth = usize::from(key.role == HistoryRole::LastDepthEdge);
    let first = corner(admitted, index, depth);
    let last = corner(admitted, (index + 1) % 4, depth);
    let circular = index.is_multiple_of(2);
    let middle = if circular {
        arc_middle(admitted, index, depth)
    } else {
        std::array::from_fn(|i| (first[i] + last[i]) / 2.)
    };
    Ok((first, last, middle, circular))
}
fn check_edge(admitted: &Admitted, key: HistoryKey, shape: &topods::Shape) -> Checked<()> {
    let edge = topods::edge(shape).map_err(|error| native("semantic edge", error))?;
    let curve =
        BRepAdaptorCurve::with_edge(&edge).map_err(|error| native("semantic curve", error))?;
    let (first, last, middle, circular) = edge_points(admitted, key)?;
    check_curve_kind(&curve, circular)?;
    check_edge_samples(&curve, [first, last, middle], tolerance(admitted))?;
    check_arc_span(&curve, admitted, circular)
}
pub(super) fn check_edge_samples(
    curve: &BRepAdaptorCurve,
    expected: [[f64; 3]; 3],
    tolerance: f64,
) -> Checked<()> {
    let first = curve
        .first_parameter()
        .map_err(|e| native("curve start", e))?;
    let last = curve.last_parameter().map_err(|e| native("curve end", e))?;
    let a = curve_point(curve, first)?;
    let b = curve_point(curve, last)?;
    let ends = endpoint_error(a, b, expected);
    close(ends, 0., tolerance, "edge endpoints")?;
    let middle = curve_point(curve, first + (last - first) / 2.)?;
    let residual = distance(middle, expected[2]);
    close(residual, 0., tolerance, "edge midpoint")
}
fn check_face(
    admitted: &Admitted,
    key: HistoryKey,
    shape: &topods::Shape,
    bindings: &Bindings,
) -> Checked<()> {
    check_face_carrier(admitted, key, shape)?;
    super::trim_binding::validate_face(
        admitted.rectangle.corners,
        admitted.rectangle.boundaries,
        key,
        shape,
        bindings,
    )
}
fn check_face_carrier(admitted: &Admitted, key: HistoryKey, shape: &topods::Shape) -> Checked<()> {
    let face = topods::face(shape).map_err(|error| native("semantic face", error))?;
    let surface = BRepAdaptorSurface::with_face(&face, true)
        .map_err(|error| native("semantic surface", error))?;
    if key.role == HistoryRole::SideFace {
        check_plane(admitted, key, &surface)?;
    } else {
        check_cylinder(admitted, key, &surface)?;
    }
    Ok(())
}
fn check_cylinder(
    admitted: &Admitted,
    key: HistoryKey,
    surface: &BRepAdaptorSurface,
) -> Checked<()> {
    require_surface(surface, SurfaceType::Cylinder)?;
    let cylinder = surface
        .cylinder()
        .map_err(|error| native("depth cylinder", error))?;
    let depth = usize::from(key.role == HistoryRole::LastDepthFace);
    close(
        cylinder.radius(),
        admitted.radii[depth],
        tolerance(admitted),
        "depth radius",
    )?;
    check_cylinder_axis(admitted, cylinder.axis())
}
fn check_plane(admitted: &Admitted, key: HistoryKey, surface: &BRepAdaptorSurface) -> Checked<()> {
    require_surface(surface, SurfaceType::Plane)?;
    let plane = surface
        .plane()
        .map_err(|error| native("side plane", error))?;
    let index = curve_index(admitted, key.source)?;
    let actual_normal = *plane.axis().direction().coordinates();
    check_direction(admitted, actual_normal, side_normal(admitted, index))?;
    close(
        plane_distance(plane, corner(admitted, index, 0)),
        0.,
        tolerance(admitted),
        "side plane location",
    )
}
fn side_normal(admitted: &Admitted, index: usize) -> [f64; 3] {
    if index.is_multiple_of(2) {
        return admitted.rectangle.support.axis;
    }
    let angle = admitted.start + if index == 1 { admitted.angle } else { 0. };
    let support = admitted.rectangle.support;
    let tangent = frame::cross(support.axis, support.radial);
    std::array::from_fn(|i| -sin(angle) * support.radial[i] + cos(angle) * tangent[i])
}

pub(super) fn curve_span(curve: &BRepAdaptorCurve) -> Checked<f64> {
    let first = curve
        .first_parameter()
        .map_err(|e| native("curve start", e))?;
    let last = curve.last_parameter().map_err(|e| native("curve end", e))?;
    Ok((last - first).abs())
}

fn curve_point(curve: &BRepAdaptorCurve, parameter: f64) -> Checked<Point3> {
    curve
        .value(parameter)
        .map_err(|error| native("curve evaluation", error))
}
pub(super) fn check_curve_kind(curve: &BRepAdaptorCurve, circular: bool) -> Checked<()> {
    let expected = if circular {
        CurveType::Circle
    } else {
        CurveType::Line
    };
    if curve.get_type().map_err(|e| native("curve type", e))? != expected {
        return Err(incomplete("edge carrier", format!("expected {expected:?}")));
    }
    Ok(())
}
fn check_arc_span(curve: &BRepAdaptorCurve, admitted: &Admitted, circular: bool) -> Checked<()> {
    if !circular {
        return Ok(());
    }
    close(
        curve_span(curve)?,
        admitted.angle,
        admitted.options.resolution.angular,
        "circular trim span",
    )
}

pub(super) fn require_surface(surface: &BRepAdaptorSurface, expected: SurfaceType) -> Checked<()> {
    if surface.get_type().map_err(|e| native("surface type", e))? != expected {
        return Err(incomplete("face carrier", format!("expected {expected:?}")));
    }
    Ok(())
}
fn check_direction(admitted: &Admitted, actual: [f64; 3], expected: [f64; 3]) -> Checked<()> {
    close(
        unoriented_angle(actual, expected)?,
        0.,
        admitted.options.resolution.angular,
        "carrier direction",
    )
}
fn check_cylinder_axis(admitted: &Admitted, axis: martensite_tkmath::gp::Axis1) -> Checked<()> {
    check_direction(
        admitted,
        *axis.direction().coordinates(),
        admitted.rectangle.support.axis,
    )?;
    let location = axis.location();
    let xyz = [location.x(), location.y(), location.z()];
    let offset = std::array::from_fn(|i| xyz[i] - admitted.rectangle.support.origin[i]);
    let perpendicular = frame::cross(offset, admitted.rectangle.support.axis);
    close(
        frame::dot(perpendicular, perpendicular).sqrt(),
        0.,
        tolerance(admitted),
        "cylinder axis location",
    )
}

fn arc_middle(admitted: &Admitted, index: usize, depth: usize) -> [f64; 3] {
    frame::point(
        &admitted.rectangle.support,
        admitted.start + admitted.angle / 2.,
        admitted.radii[depth],
        admitted.rectangle.z[index / 2],
    )
}
fn endpoint_error(a: Point3, b: Point3, expected: [[f64; 3]; 3]) -> f64 {
    let forward = distance(a, expected[0]).max(distance(b, expected[1]));
    let reverse = distance(a, expected[1]).max(distance(b, expected[0]));
    forward.min(reverse)
}
pub(super) fn plane_distance(plane: martensite_tkmath::gp::Plane3, point: [f64; 3]) -> f64 {
    let location = plane.location();
    let delta = [
        point[0] - location.x(),
        point[1] - location.y(),
        point[2] - location.z(),
    ];
    frame::dot(delta, *plane.axis().direction().coordinates())
}

pub(super) fn unoriented_angle(actual: [f64; 3], expected: [f64; 3]) -> Checked<f64> {
    let actual = unit_direction(actual)?;
    let expected = unit_direction(expected)?;
    let cross = frame::cross(actual, expected);
    let sine = hypot(hypot(cross[0], cross[1]), cross[2]);
    Ok(atan2(sine, frame::dot(actual, expected).abs()))
}
fn unit_direction(vector: [f64; 3]) -> Checked<[f64; 3]> {
    let norm = hypot(hypot(vector[0], vector[1]), vector[2]);
    if !vector.into_iter().all(f64::is_finite) || !norm.is_finite() || norm <= 0. {
        return Err(incomplete(
            "finite nondegenerate carrier direction",
            format!("{vector:?}"),
        ));
    }
    Ok(vector.map(|coordinate| coordinate / norm))
}
#[cfg(test)]
mod direction_tests {
    use super::*;
    #[test]
    fn angular_tolerance_is_measured_in_radians_for_both_axis_senses() {
        let admitted = super::super::semantic_tests::input();
        let axis = [0., 0., 1.];
        assert!(check_direction(&admitted, axis, axis).is_ok());
        assert!(check_direction(&admitted, [0., 0., -1.], axis).is_ok());
        for (angle, accepted) in [(0.5e-9_f64, true), (2e-9, false), (1e-5, false)] {
            let tilted = [sin(angle), 0., cos(angle)];
            assert_eq!(check_direction(&admitted, tilted, axis).is_ok(), accepted);
        }
    }
    #[test]
    fn unusable_carrier_directions_are_inconclusive() {
        for invalid in [[0.; 3], [f64::NAN, 0., 1.], [f64::INFINITY, 0., 1.]] {
            assert!(unoriented_angle(invalid, [0., 0., 1.]).is_err());
        }
    }
}
