//! Native Rust translations of complete-input direct `bfuse_complex` regressions.
#![allow(non_snake_case)]
use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::{brep::BRepTool, topexp, topods, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{self, Axis2, Point3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};
#[path = "bfuse_fixture_cases.rs"]
mod fixture_cases;
#[test]
fn Bug32502__SphereCylinderFuse() {
    let sphere = BRepPrimAPIMakeSphere::new(10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let cylinder = BRepPrimAPIMakeCylinder::with_axes(
        Axis2::new(Point3::new(0.0, -20.0, 7.0), gp::dy()).unwrap(),
        3.0,
        40.0,
    )
    .shape()
    .unwrap()
    .clone();
    let fuse = BRepAlgoAPIFuse::from_shapes(&sphere, &cylinder).unwrap();
    assert!(
        fuse.is_done()
            && BRepCheckAnalyzer::new(fuse.shape(), true, false, false)
                .unwrap()
                .is_valid()
                .unwrap()
    );
    let mut surface = GProps::default();
    let mut volume = GProps::default();
    BRepGProp::surface_properties(fuse.shape(), &mut surface, false, false).unwrap();
    BRepGProp::volume_properties(fuse.shape(), &mut volume, false, false, false).unwrap();
    assert!(
        (surface.mass() - 1750.1).abs() / 1750.1 <= 1e-2
            && (volume.mass() - 4947.2).abs() / 4947.2 <= 1e-2
    );
    for (kind, expected) in [
        (ShapeEnum::Vertex, 4),
        (ShapeEnum::Edge, 11),
        (ShapeEnum::Wire, 5),
        (ShapeEnum::Face, 5),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
    ] {
        let mut found = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(fuse.shape(), kind, &mut found).unwrap();
        assert_eq!(found.extent(), expected, "{kind:?}");
    }
    for vertex in topexp::Explorer::new(fuse.shape(), ShapeEnum::Vertex, ShapeEnum::Shape).unwrap()
    {
        assert!(BRepTool::vertex_tolerance(&topods::vertex(&vertex).unwrap()).unwrap() <= 3e-5);
    }
}
#[path = "bfuse_a123_case.rs"]
mod bfuse_a123_case;
#[test]
fn BFuseComplex__A1() {
    let result = bfuse_a123_case::checked_result(1);
    bfuse_a123_case::check_view_result(&result);
    bfuse_a123_case::check_end(&result);
}
#[test]
fn BFuseComplex__A2() {
    let result = bfuse_a123_case::checked_result(2);
    bfuse_a123_case::check_view_result(&result);
    bfuse_a123_case::check_end(&result);
}
#[test]
fn BFuseComplex__A3() {
    let result = bfuse_a123_case::checked_result(3);
    bfuse_a123_case::check_view_result(&result);
    bfuse_a123_case::check_end(&result);
}
#[path = "bfuse_b123_case.rs"]
mod bfuse_b123_case;
#[test]
fn BFuseComplex__B1() {
    let result = bfuse_b123_case::checked_result(1);
    bfuse_b123_case::check_view_result(&result);
    bfuse_b123_case::check_end(&result);
}
#[test]
fn BFuseComplex__B2() {
    let result = bfuse_b123_case::checked_result(2);
    bfuse_b123_case::check_view_result(&result);
    bfuse_b123_case::check_end(&result);
}
#[test]
fn BFuseComplex__B3() {
    let result = bfuse_b123_case::checked_result(3);
    bfuse_b123_case::check_view_result(&result);
    bfuse_b123_case::check_end(&result);
}
#[path = "bfuse_b4b9_case.rs"]
mod bfuse_b4b9_case;
#[test]
fn BFuseComplex__B4() {
    let result = bfuse_b4b9_case::checked_result(4);
    bfuse_b4b9_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_b4b9_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__B5() {
    let result = bfuse_b4b9_case::checked_result(5);
    bfuse_b4b9_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_b4b9_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__B6() {
    let result = bfuse_b4b9_case::checked_result(6);
    bfuse_b4b9_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_b4b9_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__B7() {
    let result = bfuse_b4b9_case::checked_result(7);
    bfuse_b4b9_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_b4b9_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__B8() {
    let result = bfuse_b4b9_case::checked_result(8);
    bfuse_b4b9_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_b4b9_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__B9() {
    let result = bfuse_b4b9_case::checked_result(9);
    bfuse_b4b9_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_b4b9_case::boolean_unconditional_case::check_end(&result);
}

#[path = "bfuse_scaled_case.rs"]
mod scaled_case;

#[test]
fn BFuseComplex__C6() {
    let result = scaled_case::c6();
    scaled_case::check_view_area(&result);
    scaled_case::check_end_validity(&result);
}

#[test]
fn BFuseComplex__C7() {
    let result = scaled_case::c7();
    scaled_case::check_view_area(&result);
    scaled_case::check_end_validity(&result);
}

#[test]
fn BFuseComplex__C8() {
    let result = scaled_case::c8();
    scaled_case::check_view_area(&result);
    scaled_case::check_end_validity(&result);
}

#[test]
fn BFuseComplex__F3() {
    let result = bfuse_fg_case::checked_f3();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__F4() {
    let result = bfuse_fg_case::checked_f4();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__F9() {
    let result = bfuse_fg_case::checked_f9();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G3() {
    let result = bfuse_fg_case::checked_g3();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G4() {
    let result = bfuse_fg_case::checked_g4();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G5() {
    let result = bfuse_fg_case::checked_g5();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G6() {
    let result = bfuse_fg_case::checked_g6();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G7() {
    let result = bfuse_fg_case::checked_g7();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G8() {
    let result = bfuse_fg_case::checked_g8();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__G9() {
    let result = bfuse_fg_case::checked_g9();
    bfuse_fg_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_fg_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__J4() {
    let result = bfuse_jkq_case::checked_j4();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__J7() {
    let result = bfuse_jkq_case::checked_j7();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__J8() {
    let result = bfuse_jkq_case::checked_j8();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__J9() {
    let result = bfuse_jkq_case::checked_j9();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__K1() {
    let result = bfuse_jkq_case::checked_k1();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__K2() {
    let result = bfuse_jkq_case::checked_k2();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__K5() {
    let result = bfuse_jkq_case::checked_k5();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}
#[test]
fn BFuseComplex__K6() {
    let result = bfuse_jkq_case::checked_k6();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}

#[test]
fn BFuseComplex__Q6() {
    let result = bfuse_jkq_case::checked_q6();
    bfuse_jkq_case::boolean_unconditional_case::check_view_result(&result);
    bfuse_jkq_case::boolean_unconditional_case::check_end(&result);
}

#[path = "bfuse_fg_case.rs"]
mod bfuse_fg_case;

#[path = "bfuse_jkq_case.rs"]
mod bfuse_jkq_case;
