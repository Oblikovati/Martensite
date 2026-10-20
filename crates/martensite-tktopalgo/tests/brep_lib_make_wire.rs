//! Tests of `BRepLib_MakeWire`.
#![allow(non_snake_case)]

use martensite_tkbrep::topods::Wire;
use martensite_tktopalgo::brep_lib::BRepLibMakeWire;

#[test]
fn BRepLib_MakeWire_Test__OCC30708_2_InitializeWithNullWire() {
    let empty = Wire::default();

    // Should not throw exception when initializing with null wire
    // EXPECT_NO_THROW(BRepLib_MakeWire aWBuilder(empty));
    assert!(BRepLibMakeWire::from_wire(&empty).is_ok());
}
