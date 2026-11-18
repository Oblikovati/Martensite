//! Cylinder native identities preserve caller-relative depth names for both normals (#522).
use super::super::*;
use crate::surface_sketch::{HistoryRole, NormalSense};
use martensite_tkbrep::{brep::BRepTool, topods::vertex};

#[test]
fn public_cylinder_depth_names_retain_the_requested_radii_for_both_normals() {
    for normal in [NormalSense::Outward, NormalSense::Inward] {
        let mut request = operation_tests::request();
        request.support.normal = normal;
        let result = normal_extrude_profile(&request).unwrap();
        check_cylinder_depths(&request, &result);
    }
}

fn check_cylinder_depths(request: &WrappedNormalRequest, result: &WrappedNormalResult) {
    let native = result.export_native().unwrap();
    let mut checked = 0;
    for (key, shape) in &native.entities {
        let Some(depth) = depth_for_role(key.role, request.depths) else {
            continue;
        };
        let point = BRepTool::pnt(&vertex(shape).unwrap()).unwrap();
        let [x, y, _] = *point.coordinates();
        let expected = request.support.radius + signed_depth(depth, request.support.normal);
        assert_close(martensite_cmath::hypot(x, y), expected);
        checked += 1;
    }
    assert!(
        checked >= 8,
        "checked={checked}; expected both depth vertex sets"
    );
}

fn signed_depth(depth: f64, normal: NormalSense) -> f64 {
    match normal {
        NormalSense::Outward => depth,
        NormalSense::Inward => -depth,
    }
}
fn depth_for_role(role: HistoryRole, depths: [f64; 2]) -> Option<f64> {
    match role {
        HistoryRole::FirstDepthVertex => Some(depths[0]),
        HistoryRole::LastDepthVertex => Some(depths[1]),
        _ => None,
    }
}
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-10,
        "actual={actual}, expected={expected}"
    );
}
