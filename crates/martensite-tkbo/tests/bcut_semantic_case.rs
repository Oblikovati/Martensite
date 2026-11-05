//! Preserve shell half-space construction, signed box dimensions and typed solid exploration.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use boolean_unconditional_case::{checked_operation, direct, restored_shape};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Shape},
    toptools::MapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeHalfSpace};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
macro_rules! restore {
    ($file:literal, $kind:ident) => {
        restored_shape(
            include_bytes!(concat!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/",
                $file
            )),
            ShapeEnum::$kind,
        )
    };
}
/// Preserve the original shell overload and reference point; `checked_e3()`.
pub fn checked_e3() -> Shape {
    let revol1 = restore!("pro5557_revol.brep", Solid);
    let bsurf2 = restore!("pro5557_bsurf.brep", Shell);
    let pnt1 = Point3::new(0., -0.3, 0.);
    // topology-command::getShape rejects the face lookup: the restored support is a shell.
    let space2 = BRepPrimAPIMakeHalfSpace::with_shell(&topods::shell(&bsurf2).unwrap(), pnt1)
        .expect("E3 shell half-space: expected a completed solid");
    assert!(space2.is_done(), "E3 shell half-space: expected completion");
    checked_operation(
        &revol1,
        &space2.solid().unwrap().clone().into_shape(),
        Operation::Cut,
        0.402138,
    )
}
/// Preserve Fuse followed by the explicit-origin signed-dimension box and Cut; `checked_i4()`.
pub fn checked_i4() -> Shape {
    let a = restore!("CTO902_cts20456a.rle", Solid);
    let b = restore!("cts20456b.rle", Solid);
    let fused = direct(&a, &b, Operation::Fuse);
    let bb = BRepPrimAPIMakeBox::at_point(Point3::default(), 100., 100., -200.)
        .expect("I4 signed box: expected dimensions (100, 100, -200)")
        .shape()
        .unwrap()
        .clone();
    checked_operation(&fused, &bb, Operation::Cut, 1.22443e6)
}
fn exploded_solids(shape: &Shape) -> Vec<Shape> {
    // topology-command typed explode seeds identity membership with the input, then retains encounter order.
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape)
        .expect("N8 explode: expected traversable compound")
        .filter(|solid| seen.add(solid.clone()))
        .collect()
}
/// Preserve all typed explode bindings and use the first encountered solid; `checked_n8()`.
pub fn checked_n8() -> Shape {
    let a = restore!("pro14957a.rle", Compound);
    let b = restore!("pro14957b.rle", Solid);
    let solids = exploded_solids(&a);
    assert_eq!(
        solids.len(),
        2,
        "N8 native explode: expected two distinct solids"
    );
    checked_operation(&solids[0], &b, Operation::Cut, 90833.6)
}
