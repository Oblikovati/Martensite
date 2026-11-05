//! Native Rust translations of complete-input direct `bcut_complex` regressions.
#![allow(non_snake_case)]
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkbrep::{topexp, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis1, Direction3, Plane3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};
#[test]
fn Bug33541__CylinderSphereCut() {
    let cylinder = BRepPrimAPIMakeCylinder::new(1.0, 1.0)
        .shape()
        .unwrap()
        .clone();
    let sphere = BRepPrimAPIMakeSphere::at_center(Point3::new(1.0, 1.0, 1.0), 1.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let cut = BRepAlgoAPICut::from_shapes(&cylinder, &sphere).unwrap();
    assert!(
        cut.is_done(),
        "tests/bugs/modalg_8/bug33541: bcut incomplete"
    );
    for (kind, expected) in [
        (ShapeEnum::Vertex, 3),
        (ShapeEnum::Edge, 5),
        (ShapeEnum::Wire, 4),
        (ShapeEnum::Face, 4),
    ] {
        let mut shapes = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(cut.shape(), kind, &mut shapes).unwrap();
        assert_eq!(shapes.extent(), expected, "{kind:?}");
    }
}

#[test]
fn Bug33570__SphereCut() {
    let first = BRepPrimAPIMakeSphere::at_center(Point3::new(0.0, 5.5, 18.0), 4.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let second = BRepPrimAPIMakeSphere::at_center(Point3::new(0.0, 4.5, 14.0), 1.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let cut = BRepAlgoAPICut::from_shapes(&first, &second).unwrap();
    assert!(
        cut.is_done(),
        "tests/bugs/modalg_8/bug33570: bcut incomplete"
    );
    for (kind, expected) in [
        (ShapeEnum::Vertex, 4),
        (ShapeEnum::Edge, 7),
        (ShapeEnum::Wire, 2),
        (ShapeEnum::Face, 2),
    ] {
        let mut shapes = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(cut.shape(), kind, &mut shapes).unwrap();
        assert_eq!(shapes.extent(), expected, "{kind:?}");
    }
}

#[test]
fn Bug30154__PlanarCut() {
    let plane = Plane3::new(
        Point3::new(165424660.0, 597500.0, 42834196.0),
        Direction3::new(-1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane(plane)
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    let mut box_shape = BRepPrimAPIMakeBox::at_point(
        Point3::new(-94190864.0, -46229000.0, -17178478.4),
        519231048.0,
        93653000.0,
        120025348.8,
    )
    .unwrap()
    .shape()
    .unwrap()
    .clone();
    box_shape.reverse();
    let cut = BRepAlgoAPICut::from_shapes(&face, &box_shape).unwrap();
    assert!(
        cut.is_done(),
        "tests/bugs/modalg_7/bug30154_1: bcut incomplete"
    );
    assert!(
        BRepCheckAnalyzer::new(cut.shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let mut properties = GProps::default();
    BRepGProp::surface_properties(cut.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 1.12407e16).abs() / 1.12407e16 <= 1e-2,
        "area {}",
        properties.mass()
    );
    for (kind, expected) in [(ShapeEnum::Wire, 1), (ShapeEnum::Face, 1)] {
        let mut shapes = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(cut.shape(), kind, &mut shapes).unwrap();
        assert_eq!(shapes.extent(), expected, "{kind:?}");
    }
}

#[test]
fn Bug32470__RotatedBoxSphereCut() {
    let sphere = BRepPrimAPIMakeSphere::new(10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let mut box_shape =
        BRepPrimAPIMakeBox::at_point(Point3::new(0.0, -50.0, -50.0), 20.0, 100.0, 100.0)
            .unwrap()
            .shape()
            .unwrap()
            .clone();
    let mut transform = Transform3::default();
    transform
        .set_rotation(
            Axis1::new(
                Point3::new(0.0, 0.0, 10.0),
                Direction3::new(0.0, -1.0, 0.0).unwrap(),
            ),
            std::f64::consts::FRAC_PI_4,
        )
        .unwrap();
    box_shape.move_by(&Location::new(transform), false).unwrap();
    let cut = BRepAlgoAPICut::from_shapes(&sphere, &box_shape).unwrap();
    assert!(
        cut.is_done(),
        "tests/bugs/modalg_7/bug32470: bcut incomplete"
    );
    assert!(
        BRepCheckAnalyzer::new(cut.shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let mut surface = GProps::default();
    let mut volume = GProps::default();
    BRepGProp::surface_properties(cut.shape(), &mut surface, false, false).unwrap();
    BRepGProp::volume_properties(cut.shape(), &mut volume, false, false, false).unwrap();
    assert!((surface.mass() - 1229.69).abs() / 1229.69 <= 1e-2);
    assert!((volume.mass() - 3945.6).abs() / 3945.6 <= 1e-2);
    for (kind, expected) in [
        (ShapeEnum::Vertex, 3),
        (ShapeEnum::Edge, 5),
        (ShapeEnum::Wire, 2),
        (ShapeEnum::Face, 2),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
    ] {
        let mut found = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(cut.shape(), kind, &mut found).unwrap();
        assert_eq!(found.extent(), expected, "{kind:?}");
    }
    for vertex in topexp::Explorer::new(cut.shape(), ShapeEnum::Vertex, ShapeEnum::Shape).unwrap() {
        assert!(BRepTool::vertex_tolerance(&topods::vertex(&vertex).unwrap()).unwrap() <= 1e-5);
    }
}
#[test]
fn BCutComplex__E3() {
    let result = bcut_semantic_case::checked_e3();
    bcut_semantic_case::boolean_unconditional_case::check_view_result(&result);
    bcut_semantic_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCutComplex__E4() {
    let result = bcut_ef_case::checked_e4();
    bcut_ef_case::boolean_unconditional_case::check_view_result(&result);
    bcut_ef_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__E6() {
    let result = bcut_ef_case::checked_e6();
    bcut_ef_case::boolean_unconditional_case::check_view_result(&result);
    bcut_ef_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__E8() {
    let result = bcut_ef_case::checked_e8();
    bcut_ef_case::boolean_unconditional_case::check_view_result(&result);
    bcut_ef_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__E9() {
    let result = bcut_ef_case::checked_e9();
    bcut_ef_case::boolean_unconditional_case::check_view_result(&result);
    bcut_ef_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__F1() {
    let result = bcut_ef_case::checked_f1();
    bcut_ef_case::boolean_unconditional_case::check_view_result(&result);
    bcut_ef_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__F3() {
    let result = bcut_ef_case::checked_f3();
    bcut_ef_case::boolean_unconditional_case::check_view_result(&result);
    bcut_ef_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__N8() {
    let result = bcut_semantic_case::checked_n8();
    bcut_semantic_case::boolean_unconditional_case::check_view_result(&result);
    bcut_semantic_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCutComplex__F8() {
    let result = bcut_fgh_case::checked_f8();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__F9() {
    let result = bcut_fgh_case::checked_f9();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__G4() {
    let result = bcut_fgh_case::checked_g4();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__G5() {
    let result = bcut_fgh_case::checked_g5();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__G9() {
    let result = bcut_fgh_case::checked_g9();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__H2() {
    let result = bcut_fgh_case::checked_h2();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__H3() {
    let result = bcut_fgh_case::checked_h3();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__H4() {
    let result = bcut_fgh_case::checked_h4();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__H5() {
    let result = bcut_fgh_case::checked_h5();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[path = "bcut_h7_case.rs"]
mod bcut_h7_case;
#[test]
fn BCutComplex__H7() {
    let result = bcut_h7_case::checked_h7();
    bcut_h7_case::boolean_unconditional_case::check_view_result(&result);
    bcut_h7_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BCutComplex__H9() {
    let result = bcut_fgh_case::checked_h9();
    bcut_fgh_case::boolean_unconditional_case::check_view_result(&result);
    bcut_fgh_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__I4() {
    let result = bcut_semantic_case::checked_i4();
    bcut_semantic_case::boolean_unconditional_case::check_view_result(&result);
    bcut_semantic_case::boolean_unconditional_case::check_end(&result);
}

// O9's original Fuse commands match J4; retain its independent cut-family declaration.
#[allow(dead_code)] // Only the shared J4 replay belongs to this target.
#[path = "bfuse_jkq_case.rs"]
mod o9_source_fuse;
#[test]
fn BCutComplex__O9() {
    let result = o9_source_fuse::checked_j4();
    o9_source_fuse::boolean_unconditional_case::check_view_result(&result);
    o9_source_fuse::boolean_unconditional_case::check_end(&result);
}

#[path = "bcut_members_case.rs"]
mod bcut_members_case;
#[test]
fn BCutComplex__O2() {
    let result = bcut_members_case::checked_o2();
    bcut_members_case::boolean_unconditional_case::check_view_result(&result);
    bcut_members_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BCutComplex__Q9() {
    let result = bcut_members_case::checked_q9();
    bcut_members_case::boolean_unconditional_case::check_view_result(&result);
    bcut_members_case::boolean_unconditional_case::check_end(&result);
}

#[path = "bcut_ef_case.rs"]
mod bcut_ef_case;

#[path = "bcut_fgh_case.rs"]
mod bcut_fgh_case;

#[path = "bcut_semantic_case.rs"]
mod bcut_semantic_case;
