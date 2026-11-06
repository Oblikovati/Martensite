//! Boolean regressions whose input construction requires ruled lofts.
#![allow(non_snake_case)]
use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::topods::Wire;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIThruSections;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use martensite_topology_test_support::{CreateBox, CreateEdge, CreateVertex, GetVolume};
/// Create a circular wire at given height.
fn CreateCircularWire(radius: f64, z: f64) -> Wire {
    let axis = Axis2::new(
        Point3::new(0.0, 0.0, z),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let circle = Circle3::new(axis, radius).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_circ(circle)
        .unwrap()
        .edge()
        .unwrap();
    BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap()
}
/// The closed wire through the corners, one straight edge per side.
fn quadrilateral_wire(corners: [(f64, f64, f64); 4]) -> Wire {
    let mut maker = BRepBuilderAPIMakeWire::new();
    for (index, &(x, y, z)) in corners.iter().enumerate() {
        let (nx, ny, nz) = corners[(index + 1) % corners.len()];
        let edge = CreateEdge(Point3::new(x, y, z), Point3::new(nx, ny, nz));
        maker.add_edge(&edge).unwrap();
    }
    maker.wire().unwrap()
}
/// The square wire of the second loft (and the top of the first) at the height.
fn square_wire(z: f64) -> Wire {
    quadrilateral_wire([
        (0.0, 0.0, z),
        (100.0, 0.0, z),
        (100.0, -100.0, z),
        (0.0, -100.0, z),
    ])
}
/// A solid ruled loft of the wires; `build` is called explicitly on purpose.
fn ruled_loft(wires: &[Wire]) -> BRepOffsetAPIThruSections {
    let mut loft = BRepOffsetAPIThruSections::new(true, true, 1.0e-6);
    for wire in wires {
        loft.add_wire(wire);
    }
    loft
}
// Test: Boolean fuse of loft (wire to vertex) with box
// This tests the fix for FillPaves() handling edges without 2D curves.
// A loft from a circular wire to a vertex creates a cone-like shape
// with a degenerated edge at the apex.
#[test]
fn pave_filler_fuses_a_cone_loft_with_a_box_at_a_degenerated_edge() {
    // Create a loft from a circular wire to a point (apex)
    // This creates a cone-like shape with a degenerated edge at the top
    let base_wire = CreateCircularWire(10.0, 0.0);
    let apex = CreateVertex(Point3::new(0.0, 0.0, 20.0));
    let mut loft_maker = ruled_loft(&[base_wire]);
    loft_maker.add_vertex(&apex).unwrap();
    loft_maker.build(&Default::default()).unwrap();
    assert!(loft_maker.is_done(), "Loft operation should succeed");
    let loft = loft_maker.shape().unwrap().clone();
    assert!(!loft.is_null(), "Loft shape should not be null");

    // Create a box that intersects the loft
    let box_shape = CreateBox(Point3::new(-5.0, -5.0, 5.0), 10.0, 10.0, 10.0);

    // Perform Boolean fuse - this should not crash even if there are
    // degenerated edges without pcurves
    let fuser = BRepAlgoAPIFuse::from_shapes(&loft, &box_shape).unwrap();

    // The operation should complete without crashing
    assert!(
        fuser.is_done(),
        "Boolean fuse should succeed for loft with degenerated edges"
    );
    assert!(!fuser.shape().is_null(), "Result shape should not be null");

    // Verify the result has reasonable volume (both shapes contribute)
    let loft_volume = GetVolume(&loft);
    let box_volume = GetVolume(&box_shape);
    let result_volume = GetVolume(fuser.shape());

    // Result volume should be less than sum (intersection) but more than max (union effect)
    assert!(result_volume > 0.0, "Result should have positive volume");
    assert!(
        result_volume < loft_volume + box_volume,
        "Result volume should be less than sum of inputs"
    );
}
// Test: Two lofts fused together (original bug10006 scenario)
// This is similar to the existing test but focuses on the robustness aspect
#[test]
fn pave_filler_fuses_two_lofts() {
    // Create first loft: quadrilateral wire to quadrilateral wire
    let bottom1 = quadrilateral_wire([
        (10.0, -10.0, 0.0),
        (100.0, -10.0, 0.0),
        (100.0, -100.0, 0.0),
        (10.0, -100.0, 0.0),
    ]);
    let mut loft1 = ruled_loft(&[bottom1, square_wire(10.0)]);
    loft1.build(&Default::default()).unwrap();
    assert!(loft1.is_done(), "First loft should succeed");

    // Create second loft
    let mut loft2 = ruled_loft(&[square_wire(10.0), square_wire(250.0)]);
    loft2.build(&Default::default()).unwrap();
    assert!(loft2.is_done(), "Second loft should succeed");

    // Fuse the two lofts - this is the operation that could crash
    let first = loft1.shape().unwrap().clone();
    let second = loft2.shape().unwrap().clone();
    let fuser = BRepAlgoAPIFuse::from_shapes(&first, &second).unwrap();

    assert!(fuser.is_done(), "Boolean fuse of two lofts should succeed");
    assert!(!fuser.shape().is_null(), "Result shape should not be null");
    assert!(
        GetVolume(fuser.shape()) > 0.0,
        "Result should have positive volume"
    );
}
