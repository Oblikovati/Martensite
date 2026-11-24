//! Check analytic carrier apex, angle, axis and the actual trimmed half-cone.
use super::{
    cone_admission::AdmittedCone, cone_math, cone_semantic::*, failure::*, frame, history::*,
    semantic::*,
};
use martensite_cmath::{cos, sin};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::{geom_abs::SurfaceType, gp::Point3};
use std::f64::consts::FRAC_PI_2;

pub(super) fn check(
    admitted: &AdmittedCone,
    key: HistoryKey,
    shape: &topods::Shape,
    bindings: &Bindings,
) -> Checked<()> {
    check_carrier(admitted, key, shape)?;
    super::trim_binding::validate_face(
        admitted.sector.corners,
        admitted.sector.boundaries,
        key,
        shape,
        bindings,
    )
}
fn check_carrier(admitted: &AdmittedCone, key: HistoryKey, shape: &topods::Shape) -> Checked<()> {
    let face = topods::face(shape).map_err(|e| native("cone semantic face", e))?;
    let surface = BRepAdaptorSurface::with_face(&face, true)
        .map_err(|e| native("cone semantic surface", e))?;
    let boundary = if key.role == HistoryRole::SideFace {
        Some(curve_index(admitted, key.source)?)
    } else {
        None
    };
    match boundary {
        Some(index) if !index.is_multiple_of(2) => check_plane(admitted, index, &surface)?,
        _ => check_cone(admitted, key, boundary, &surface)?,
    }
    Ok(())
}
struct ExpectedCarrier {
    apex: [f64; 3],
    axis: [f64; 3],
    angle: f64,
}
fn carrier(admitted: &AdmittedCone, key: HistoryKey, boundary: Option<usize>) -> ExpectedCarrier {
    let support = admitted.sector.support;
    if let Some(index) = boundary {
        return ExpectedCarrier {
            apex: cone_math::slant_apex(admitted, index / 2),
            axis: support.axis.map(|v| -v),
            angle: FRAC_PI_2 - support.semi_angle,
        };
    }
    let depth = usize::from(key.role == HistoryRole::LastDepthFace);
    ExpectedCarrier {
        apex: cone_math::depth_apex(admitted, depth),
        axis: support.axis,
        angle: support.semi_angle,
    }
}
fn check_cone(
    admitted: &AdmittedCone,
    key: HistoryKey,
    boundary: Option<usize>,
    surface: &BRepAdaptorSurface,
) -> Checked<()> {
    require_surface(surface, SurfaceType::Cone)?;
    let cone = surface.cone().map_err(|e| native("cone carrier", e))?;
    let expected = carrier(admitted, key, boundary);
    let offset = distance(cone.apex(), expected.apex);
    close(offset, 0., tolerance(admitted), "cone carrier apex")?;
    carrier_angles(admitted, &cone, &expected)?;
    let point = surface_midpoint(surface)?;
    check_half_cone(point, &expected, tolerance(admitted))
}

fn carrier_angles(
    admitted: &AdmittedCone,
    cone: &martensite_tkmath::gp::Cone3,
    expected: &ExpectedCarrier,
) -> Checked<()> {
    let angular = admitted.sweep.options.resolution.angular;
    close(
        cone.semi_angle().abs(),
        expected.angle,
        angular,
        "cone carrier angle",
    )?;
    let actual = *cone.axis().direction().coordinates();
    close(
        unoriented_angle(actual, expected.axis)?,
        0.,
        angular,
        "cone carrier axis",
    )
}
fn surface_midpoint(surface: &BRepAdaptorSurface) -> Checked<Point3> {
    let first_u = surface
        .first_u_parameter()
        .map_err(|e| native("cone first u", e))?;
    let last_u = surface
        .last_u_parameter()
        .map_err(|e| native("cone last u", e))?;
    let first_v = surface
        .first_v_parameter()
        .map_err(|e| native("cone first v", e))?;
    let last_v = surface
        .last_v_parameter()
        .map_err(|e| native("cone last v", e))?;
    surface
        .value(
            first_u + (last_u - first_u) / 2.,
            first_v + (last_v - first_v) / 2.,
        )
        .map_err(|e| native("cone trim midpoint", e))
}
fn check_half_cone(point: Point3, expected: &ExpectedCarrier, tolerance: f64) -> Checked<()> {
    let ExpectedCarrier { apex, axis, angle } = *expected;
    let xyz = [point.x(), point.y(), point.z()];
    let offset = std::array::from_fn(|i| xyz[i] - apex[i]);
    let axial = frame::dot(offset, axis);
    require_positive_half(axial)?;
    let perpendicular = frame::cross(offset, axis);
    let radius = frame::dot(perpendicular, perpendicular).sqrt();
    close(
        radius * cos(angle) - axial * sin(angle),
        0.,
        tolerance,
        "cone trim half-carrier",
    )
}
fn require_positive_half(axial: f64) -> Checked<()> {
    if !axial.is_finite() || axial <= 0. {
        return Err(incomplete(
            "selected cone half",
            format!("axial {axial}, expected positive"),
        ));
    }
    Ok(())
}
fn plane_normal(admitted: &AdmittedCone, boundary: usize) -> [f64; 3] {
    let support = admitted.sector.support;
    let angle = admitted.start
        + if boundary == 1 {
            admitted.sweep.angle
        } else {
            0.
        };
    let tangent = frame::cross(support.axis, support.radial);
    std::array::from_fn(|i| -sin(angle) * support.radial[i] + cos(angle) * tangent[i])
}
fn check_plane(
    admitted: &AdmittedCone,
    boundary: usize,
    surface: &BRepAdaptorSurface,
) -> Checked<()> {
    require_surface(surface, SurfaceType::Plane)?;
    let plane = surface
        .plane()
        .map_err(|e| native("cone angular plane", e))?;
    check_plane_direction(admitted, boundary, plane)?;
    let offset = plane_distance(plane, cone_math::corner(admitted, boundary, 0));
    close(
        offset,
        0.,
        tolerance(admitted),
        "cone angular plane position",
    )
}

fn check_plane_direction(
    admitted: &AdmittedCone,
    boundary: usize,
    plane: martensite_tkmath::gp::Plane3,
) -> Checked<()> {
    let angle = unoriented_angle(
        *plane.axis().direction().coordinates(),
        plane_normal(admitted, boundary),
    )?;
    close(
        angle,
        0.,
        admitted.sweep.options.resolution.angular,
        "cone angular plane normal",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opposite_half_cone_is_rejected_despite_matching_double_cone() {
        let angle = martensite_cmath::atan2(3., 4.);
        let apex = [0.; 3];
        let axis = [0., 0., 1.];
        let expected = ExpectedCarrier { apex, axis, angle };
        assert!(check_half_cone(Point3::new(3., 0., 4.), &expected, 1e-9).is_ok());
        for point in [
            Point3::new(3., 0., -4.),
            Point3::new(0., 0., 0.),
            Point3::new(f64::NAN, 0., 4.),
        ] {
            assert!(check_half_cone(point, &expected, 1e-9).is_err());
        }
    }
}
