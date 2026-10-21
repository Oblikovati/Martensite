//! Bnd_OBB: the oriented box of a BRepPrimAPI box added to a void box.
//!
//! The math crate's oriented-box case that needs primitive shapes.
#![allow(non_snake_case)]
use martensite_tkmath::bnd::OrientedBoundingBox;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{brep_bnd_lib::BRepBndLib, brep_builder_api::BRepBuilderAPIMakeShape};

// EXPECT_DOUBLE_EQ uses a four-ULP comparison.
fn double_eq(a: f64, b: f64) {
    assert!(!a.is_nan() && !b.is_nan());
    assert!(ordered_bits(a).abs_diff(ordered_bits(b)) <= 4, "{a} != {b}");
}
fn ordered_bits(value: f64) -> u64 {
    let bits: u64 = value.to_bits();
    if bits >> 63 != 0 {
        (!bits).wrapping_add(1)
    } else {
        bits | (1 << 63)
    }
}
/// The original case `AddBoundingBoxToVoidBox`.
///
/// `BRepBndLib::AddOBB(BRepPrimAPI_MakeBox(gp_Pnt(100,100,100),100,100,100), aOBB,
/// false, false, false)` rebuilds the void OBB from the box vertices without tolerances
/// (`CheckPoints`); this now uses those builders instead of the earlier plain-point surrogate.
#[test]
fn Bnd_OBB_Test__OCC30704_AddBoundingBoxToVoidBox() {
    let mut maker =
        BRepPrimAPIMakeBox::at_point(Point3::new(100.0, 100.0, 100.0), 100.0, 100.0, 100.0)
            .unwrap();
    let shape = maker.shape().unwrap();
    let mut aVoidBox = OrientedBoundingBox::default();
    let mut aOBB = OrientedBoundingBox::default();
    BRepBndLib::add_obb(shape, &mut aOBB, false, false, false).unwrap();
    aVoidBox.add_box(&aOBB).unwrap();

    // Check the center point of the bounding box
    let aCenter = aVoidBox.center();
    double_eq(aCenter.x(), 150.0);
    double_eq(aCenter.y(), 150.0);
    double_eq(aCenter.z(), 150.0);
}
