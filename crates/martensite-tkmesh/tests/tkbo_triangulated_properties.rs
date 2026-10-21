//! Boolean result measurements checked against their actual triangulations.
#![allow(non_snake_case)]
use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkmesh::{brep_mesh::BRepMeshIncrementalMesh, imesh_tools::MeshParameters};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};
use martensite_topology_test_support::expect_near;
use std::f64::consts::PI;
fn ExpectSamePrincipalProperties(exact: &GProps, triangulated: &GProps) {
    expect_near(exact.mass(), triangulated.mass(), 1.0e-7);
    let (exact_x, exact_y, exact_z) = exact.principal_properties().unwrap().moments();
    let (mesh_x, mesh_y, mesh_z) = triangulated.principal_properties().unwrap().moments();
    expect_near(exact_x, mesh_x, 1.0e-7);
    expect_near(exact_y, mesh_y, 1.0e-7);
    expect_near(exact_z, mesh_z, 1.0e-7);
}
fn CompareExactAndTriangulatedProperties(shape: &Shape, property_type: i32) {
    let mut exact = GProps::default();
    let mut triangulated = GProps::default();
    match property_type {
        0 => {
            BRepGProp::linear_properties(shape, &mut exact, false, false).unwrap();
            BRepGProp::linear_properties(shape, &mut triangulated, false, true).unwrap();
        }
        1 => {
            BRepGProp::surface_properties(shape, &mut exact, false, false).unwrap();
            BRepGProp::surface_properties(shape, &mut triangulated, false, true).unwrap();
        }
        _ => {
            BRepGProp::volume_properties(shape, &mut exact, false, false, false).unwrap();
            BRepGProp::volume_properties(shape, &mut triangulated, false, false, true).unwrap();
        }
    }
    ExpectSamePrincipalProperties(&exact, &triangulated);
}
// bugs/modalg_7/bug29734: global properties computed from planar face
// triangulations must agree with the exact-geometry calculations.
#[test]
fn BRepTools_Test__ModalgBug_29734_ExactAndTriangulatedProperties() {
    let first_box = BRepPrimAPIMakeBox::from_sizes(1.0, 2.0, 3.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let second_box = BRepPrimAPIMakeBox::from_sizes(3.0, 2.0, 1.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();

    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(0.5, 0.5, 0.5));
    let translated = BRepBuilderAPITransform::with_shape(&second_box, translation, false, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone();

    let mut rotation = Transform3::default();
    let axis = Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 1.0, 1.0).unwrap(),
    );
    rotation.set_rotation(axis, PI / 6.0).unwrap();
    let rotated = BRepBuilderAPITransform::with_shape(&translated, rotation, false, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let fused = BRepAlgoAPIFuse::from_shapes(&first_box, &rotated)
        .unwrap()
        .shape()
        .clone();
    assert!(!fused.is_null());

    BRepMeshIncrementalMesh::with_parameters(
        &fused,
        &MeshParameters {
            deflection: 0.01,
            ..MeshParameters::default()
        },
        &ProgressRange::default(),
    )
    .expect("BRepMesh_IncrementalMesh::IsDone() should be true");
    assert!(
        BRepCheckAnalyzer::new(&fused, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );

    CompareExactAndTriangulatedProperties(&fused, 0);
    CompareExactAndTriangulatedProperties(&fused, 1);
    CompareExactAndTriangulatedProperties(&fused, 2);
}
