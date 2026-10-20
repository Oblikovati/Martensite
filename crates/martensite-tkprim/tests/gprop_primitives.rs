//! Original topology property regressions hosted with their primitive builder dependency.
#[path = "common/floating.rs"]
mod floating;
mod primitive_legacy_support;
use floating::within_absolute_error as near;
#[path = "common/property_assertions.rs"]
mod property_assertions;
use martensite_tkernel::precision;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use primitive_legacy_support::legacy_box_sizes;
use property_assertions::expect_properties_near as assert_gprops_near;
use std::f64::consts::PI;

// BRepGPropTest.SurfaceProperties_BoxFaceArea
#[test]
fn surface_properties_box_face_area() {
    let mut props = GProps::new();
    BRepGProp::surface_properties(&legacy_box_sizes(10., 20., 30.), &mut props, false, false)
        .unwrap();
    assert!(near(props.mass(), 2200.0, precision::confusion()));
}

// BRepGPropTest.VolumeProperties_UnitBox
#[test]
fn volume_properties_unit_box() {
    let mut props = GProps::new();
    BRepGProp::volume_properties(
        &legacy_box_sizes(1., 1., 1.),
        &mut props,
        false,
        false,
        false,
    )
    .unwrap();
    assert!(near(props.mass(), 1.0, precision::confusion()));
}

// BRepGPropTest.VolumeProperties_Sphere
#[test]
fn volume_properties_sphere() {
    let mut maker = BRepPrimAPIMakeSphere::new(5.).unwrap();
    let sphere = maker.shape().unwrap().clone();
    assert!(maker.is_done(), "Sphere creation failed");
    let mut props = GProps::new();
    BRepGProp::volume_properties(&sphere, &mut props, false, false, false).unwrap();
    assert!(near(props.mass(), (4.0 / 3.0) * PI * 125.0, 0.01));
}

// BRepGPropTest.VolumeProperties_BoxCenterOfMass
#[test]
fn volume_properties_box_center_of_mass() {
    let mut props = GProps::new();
    BRepGProp::volume_properties(
        &legacy_box_sizes(10., 10., 10.),
        &mut props,
        false,
        false,
        false,
    )
    .unwrap();
    let com = props.centre_of_mass();
    for c in [com.x(), com.y(), com.z()] {
        assert!(near(c, 5.0, precision::confusion()));
    }
}

// BRepGPropTest.AdaptiveSurfaceProperties_Box
#[test]
fn adaptive_surface_properties_box() {
    let mut props = GProps::new();
    BRepGProp::surface_properties_eps(&legacy_box_sizes(10., 20., 30.), &mut props, 1.0e-6, false)
        .unwrap();
    assert!(near(props.mass(), 2200.0, 1.0e-9));
}

// BRepGPropTest.AdaptiveVolumeProperties_Box
#[test]
fn adaptive_volume_properties_box() {
    let mut props = GProps::new();
    BRepGProp::volume_properties_eps(
        &legacy_box_sizes(10., 20., 30.),
        &mut props,
        1.0e-6,
        false,
        false,
    )
    .unwrap();
    assert!(near(props.mass(), 6000.0, 1.0e-9));
}

// BRepGPropTest.LinearProperties_SkipShared
#[test]
fn linear_properties_skip_shared() {
    let solid = legacy_box_sizes(10., 10., 10.);
    let (mut not_skipped, mut skipped) = (GProps::new(), GProps::new());
    BRepGProp::linear_properties(&solid, &mut not_skipped, false, false).unwrap();
    BRepGProp::linear_properties(&solid, &mut skipped, true, false).unwrap();
    assert!(near(skipped.mass(), 120.0, precision::confusion()));
    assert!(near(not_skipped.mass(), 240.0, precision::confusion()));
}

// CylinderHasSymmetryAxis
#[test]
fn occ49_cylinder_has_symmetry_axis() {
    let cylinder = BRepPrimAPIMakeCylinder::new(10., 20.)
        .shape()
        .unwrap()
        .clone();
    let mut props = GProps::new();
    BRepGProp::volume_properties(&cylinder, &mut props, false, false, false).unwrap();
    assert!(props.principal_properties().unwrap().has_symmetry_axis());
}

// BRepGPropTest.AdaptiveVolumeProperties_Cylinder
#[test]
fn adaptive_volume_properties_cylinder() {
    let (radius, height) = (3.0, 7.0);
    let cylinder = BRepPrimAPIMakeCylinder::new(radius, height)
        .shape()
        .unwrap()
        .clone();
    let mut fixed = GProps::new();
    BRepGProp::volume_properties(&cylinder, &mut fixed, false, false, false).unwrap();
    let mut adaptive = GProps::new();
    let error =
        BRepGProp::volume_properties_eps(&cylinder, &mut adaptive, 1.0e-8, false, false).unwrap();
    assert!((0.0..=1.0e-8).contains(&error));
    let mass = PI * radius * radius * height;
    assert!(near(adaptive.mass(), mass, 1.0e-10));
    let com = adaptive.centre_of_mass();
    assert!(near(com.x(), 0.0, 1.0e-12) && near(com.y(), 0.0, 1.0e-12));
    assert!(near(com.z(), height * 0.5, 1.0e-12));
    let m = adaptive.matrix_of_inertia();
    let side = mass * (3.0 * radius * radius + height * height) / 12.0;
    assert!(near(m.value(1, 1).unwrap(), side, 2.0e-4));
    assert!(near(m.value(2, 2).unwrap(), side, 2.0e-4));
    assert!(near(
        m.value(3, 3).unwrap(),
        mass * radius * radius / 2.0,
        2.0e-4
    ));
    assert_gprops_near(&adaptive, &fixed, 2.0e-4);
}
