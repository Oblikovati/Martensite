//! Independent sampled normal and estimate controls use the same detached completed result.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods::ShapeIterator};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Vector3;
pub(super) fn check(result: &WrappedConeNormalResult, detached: &DetachedWrappedBodies) {
    let mut checked = [0usize; 4];
    for face in faces(&detached.bodies) {
        check_face(&face, key(detached, &face), result, &mut checked);
    }
    assert_eq!(checked[0], 2);
    assert!(checked[1] > 0 && checked[2] > 0 && checked[3] > checked[1] + checked[2]);
    native_estimate(result);
    super::native_flux::check(result, detached);
}
fn check_face(
    face: &Shape,
    key: WrappedHistoryKey,
    result: &WrappedConeNormalResult,
    checked: &mut [usize; 4],
) {
    if cap(key.role) {
        let role = physical_cap_role(key.role, result.support().normal);
        check_physical_cap(face, role, result.support().semi_angle);
        checked[0] += 1;
    }
    if curve(key) == Some(SurfaceCurveId(1)) {
        check_side(face, Vector3::new(0., -1., 0.));
        checked[1] += 1;
    }
    if curve(key) == Some(SurfaceCurveId(5)) {
        check_hole(face, result.support().semi_angle);
        checked[2] += 1;
    }
    if key.role == HistoryRole::SideFace {
        checked[3] += 1;
    }
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn faces(
    root: &Shape,
) -> Vec<Shape> {
    children(root)
        .iter()
        .flat_map(children)
        .flat_map(|shell| children(&shell))
        .collect()
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn children(
    parent: &Shape,
) -> Vec<Shape> {
    let mut iterator = ShapeIterator::new(parent, true, true);
    let mut result = Vec::new();
    while iterator.more() {
        result.push(iterator.value().unwrap().clone());
        iterator.next();
    }
    result
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn key(
    detached: &DetachedWrappedBodies,
    face: &Shape,
) -> WrappedHistoryKey {
    assert_eq!(face.shape_type().unwrap(), ShapeEnum::Face);
    let keys: Vec<_> = detached
        .entities
        .iter()
        .filter(|(key, shape)| {
            (cap(key.role) || key.role == HistoryRole::SideFace) && shape.is_same(face)
        })
        .map(|(key, _)| *key)
        .collect();
    assert_eq!(keys.len(), 1);
    keys[0]
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn cap(
    role: HistoryRole,
) -> bool {
    matches!(
        role,
        HistoryRole::FirstDepthFace | HistoryRole::LastDepthFace
    )
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn physical_cap_role(
    role: HistoryRole,
    normal: NormalSense,
) -> HistoryRole {
    match (normal, role) {
        (NormalSense::Inward, HistoryRole::FirstDepthFace) => HistoryRole::LastDepthFace,
        (NormalSense::Inward, HistoryRole::LastDepthFace) => HistoryRole::FirstDepthFace,
        _ => role,
    }
}
fn curve(key: WrappedHistoryKey) -> Option<SurfaceCurveId> {
    if key.role != HistoryRole::SideFace {
        return None;
    }
    match key.source {
        WrappedEntitySource::Curve { curve, .. } => Some(curve),
        _ => None,
    }
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn derivatives(
    face: &Shape,
    u: f64,
    v: f64,
) -> martensite_tkg3d::geom::GeomSurfaceD1 {
    let face = martensite_tkbrep::topods::face(face).unwrap();
    let (surface, location) = BRepTool::surface_with_location(&face).unwrap();
    assert!(location.is_identity());
    surface.get().unwrap().d1(u, v).unwrap()
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn normal(
    face: &Shape,
    derivatives: &martensite_tkg3d::geom::GeomSurfaceD1,
) -> Vector3 {
    let normal = derivatives.d1u.crossed(derivatives.d1v);
    match face.orientation() {
        Orientation::Forward => normal,
        Orientation::Reversed => normal.reversed(),
        other => panic!("actual face use {other:?}"),
    }
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn check_physical_cap(
    face: &Shape,
    role: HistoryRole,
    alpha: f64,
) {
    let (s, c) = (martensite_cmath::sin(alpha), martensite_cmath::cos(alpha));
    let u = martensite_cmath::atan2(0.1, 4.1) / s;
    let v = martensite_cmath::hypot(4.1, 0.1) - 4.;
    let expected = Vector3::new(
        c * martensite_cmath::cos(u),
        c * martensite_cmath::sin(u),
        -s,
    );
    let dot = normal(face, &derivatives(face, u, v)).dot(expected);
    if role == HistoryRole::FirstDepthFace {
        assert!(dot < 0.);
    } else {
        assert!(dot > 0.);
    }
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn check_side(
    face: &Shape,
    expected: Vector3,
) {
    let actual = normal(face, &derivatives(face, 0.5, 0.5));
    assert!(
        actual.dot(expected) > 0.,
        "sampled outer side normal={actual:?}, expected={expected:?}"
    );
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone::tests) fn check_hole(
    face: &Shape,
    alpha: f64,
) {
    let derivative = derivatives(face, 0.5, 0.5);
    let point = derivative.point;
    let (s, c) = (martensite_cmath::sin(alpha), martensite_cmath::cos(alpha));
    let theta = martensite_cmath::atan2(point.y(), point.x());
    let (ct, st) = (martensite_cmath::cos(theta), martensite_cmath::sin(theta));
    let radius = martensite_cmath::hypot(point.x(), point.y());
    let slant = radius * s + point.z() * c;
    let phase = theta * s;
    let (cp, sp) = (martensite_cmath::cos(phase), martensite_cmath::sin(phase));
    let expected = Vector3::new(
        s * ct * cp + radius * st * sp / (s * slant),
        s * st * cp - radius * ct * sp / (s * slant),
        c * cp,
    );
    assert!(
        normal(face, &derivative).dot(expected) > 0.,
        "sampled hole side must face into the void"
    );
}
fn native_estimate(result: &WrappedConeNormalResult) {
    let native = result.reports()[&SurfaceRegionId(1)].native_volume;
    assert!(native.value.is_finite() && native.value > 0.);
    assert!(native.integration_error.is_finite() && native.integration_error >= 0.);
}
