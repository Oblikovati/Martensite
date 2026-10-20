//! Tests of `TopoDS_Edge`.
//! BRepBuilderAPI_MakeEdge is the BRep fake of `support`.
#![allow(non_snake_case)]
mod support;
use martensite_tkmath::gp::Point3;
use support::make_edge;

// TopoDS_Edge Infinite flag getter/setter
#[test]
fn TopoDS_Edge_Test__BUC60828_InfiniteFlag() {
    // Create a simple edge
    let anEdge = make_edge(Point3::new(0., 0., 0.), Point3::new(0., 0., 1.));

    // Check initial flag value (should be false by default)
    let anInitialValue = anEdge.infinite().unwrap();
    assert!(!anInitialValue, "Initial Infinite flag should be false");

    // Set the flag to true
    anEdge.set_infinite(true).unwrap();

    // Verify the flag was set correctly
    let aCurrentValue = anEdge.infinite().unwrap();
    assert!(aCurrentValue, "Infinite flag should be true after setting");
}
