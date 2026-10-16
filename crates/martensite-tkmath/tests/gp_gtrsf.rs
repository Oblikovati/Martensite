//! Plane affinity leaves identity intact and scales the normal component.
#![allow(non_snake_case)]
use martensite_tkmath::gp::{Axis2, Direction3, GeneralTransform3, Point3, Xyz};
#[test]
fn geometry_g_trsf_set_affinity_ratio1_is_identity() {
    let plane = Axis2::new(Point3::default(), Direction3::new(1.0, -1.0, 0.0).unwrap()).unwrap();
    let mut transform = GeneralTransform3::default();
    transform.set_affinity_plane(plane, 1.0);
    let origin = Xyz::new(1.0, 0.0, 0.0);
    let mut result = origin;
    transform.transforms(&mut result);
    assert!((result.x() - 1.0).abs() <= 1e-7);
    assert!(result.y().abs() <= 1e-7);
    assert!(result.z().abs() <= 1e-7);
}
#[test]
fn geometry_g_trsf_set_affinity_ratio2_scales_along_normal() {
    let plane = Axis2::new(Point3::default(), Direction3::new(1.0, -1.0, 0.0).unwrap()).unwrap();
    let mut transform = GeneralTransform3::default();
    transform.set_affinity_plane(plane, 2.0);
    let origin = Xyz::new(1.0, 0.0, 0.0);
    let mut result = origin;
    transform.transforms(&mut result);
    assert!((result.x() - 1.5).abs() <= 1e-7);
    assert!((result.y() + 0.5).abs() <= 1e-7);
    assert!(result.z().abs() <= 1e-7);
}
