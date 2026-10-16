//! Bnd bound sort box tests.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::{Array1, List};
use martensite_tkmath::bnd::{BoundSortBox, BoundingBox3};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};

// Helper function to create a box from min and max points
fn CreateBox(xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) -> BoundingBox3 {
    let mut aBox = BoundingBox3::default();
    aBox.update_bounds([xmin, ymin, zmin], [xmax, ymax, zmax]);
    aBox
}

/// The shared box fixture.
struct Fixture {
    mySmallBox: BoundingBox3,
    myLargeBox: BoundingBox3,
    myOffsetBox: BoundingBox3,
    myNonIntersectingBox: BoundingBox3,
    myGlobalBox: BoundingBox3,
    myBoxes: Array1<'static, BoundingBox3>,
}
fn SetUp() -> Fixture {
    let mySmallBox = CreateBox(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let myLargeBox = CreateBox(-10.0, -10.0, -10.0, 10.0, 10.0, 10.0);
    let myOffsetBox = CreateBox(5.0, 5.0, 5.0, 7.0, 7.0, 7.0);
    let myNonIntersectingBox = CreateBox(20.0, 20.0, 20.0, 30.0, 30.0, 30.0);
    let mut myBoxes = Array1::with_bounds(1, 4).unwrap();
    myBoxes[1] = mySmallBox.clone();
    myBoxes[2] = myLargeBox.clone();
    myBoxes[3] = myOffsetBox.clone();
    myBoxes[4] = myNonIntersectingBox.clone();
    let myGlobalBox = CreateBox(-20.0, -20.0, -20.0, 40.0, 40.0, 40.0);
    Fixture {
        mySmallBox,
        myLargeBox,
        myOffsetBox,
        myNonIntersectingBox,
        myGlobalBox,
        myBoxes,
    }
}
fn contains(result: &List<i32>, index: i32) -> bool {
    result.iter().any(|value| *value == index)
}
fn two_boxes(first: BoundingBox3, second: BoundingBox3) -> Array1<'static, BoundingBox3> {
    let mut boxes = Array1::with_bounds(1, 2).unwrap();
    boxes[1] = first;
    boxes[2] = second;
    boxes
}

#[test]
fn bounds_bound_sort_box_initialize_with_boxes() {
    let f = SetUp();
    let mut sortBox = BoundSortBox::default();
    sortBox.initialize(f.myBoxes.clone()).unwrap();

    // Test comparing with a box that intersects mySmallBox
    let testBox = CreateBox(0.5, 0.5, 0.5, 1.5, 1.5, 1.5);
    let result = sortBox.compare_box(&testBox).unwrap();
    assert_eq!(2, result.extent(), "Expected to find 2 intersections");

    // Check that box indices 1 (mySmallBox) and 2 (myLargeBox) are in the result
    assert!(
        contains(result, 1),
        "Small box (index 1) should be in the result"
    );
    assert!(
        contains(result, 2),
        "Large box (index 2) should be in the result"
    );
}

#[test]
fn bounds_bound_sort_box_initialize_with_enclosing_box() {
    let f = SetUp();
    let mut sortBox = BoundSortBox::default();
    sortBox
        .initialize_with_enclosing(&f.myGlobalBox, f.myBoxes.clone())
        .unwrap();

    // Test comparing with myOffsetBox
    let result = sortBox.compare_box(&f.myOffsetBox).unwrap();
    assert_eq!(2, result.extent(), "Expected to find 2 intersections");

    // Check that box indices 2 (myLargeBox) and 3 (myOffsetBox) are in the result
    assert!(
        contains(result, 2),
        "Large box (index 2) should be in the result"
    );
    assert!(
        contains(result, 3),
        "Offset box (index 3) should be in the result"
    );
}

#[test]
fn bounds_bound_sort_box_initialize_with_count() {
    let f = SetUp();
    let mut sortBox = BoundSortBox::default();
    sortBox.initialize_with_count(&f.myGlobalBox, 3).unwrap();

    // Add boxes manually
    sortBox.add(&f.mySmallBox, 1).unwrap();
    sortBox.add(&f.myLargeBox, 2).unwrap();
    sortBox.add(&f.myNonIntersectingBox, 3).unwrap();

    // Test comparing with a box that should only intersect myLargeBox
    let testBox = CreateBox(-5.0, -5.0, -5.0, -2.0, -2.0, -2.0);
    let result = sortBox.compare_box(&testBox).unwrap();
    assert_eq!(1, result.extent(), "Expected to find 1 intersection");

    // Verify that only the large box was found
    assert_eq!(
        Some(&2),
        result.first(),
        "Large box (index 2) should be the only result"
    );
}

#[test]
fn bounds_bound_sort_box_compare_with_plane() {
    let f = SetUp();
    let mut sortBox = BoundSortBox::default();
    sortBox.initialize(f.myBoxes.clone()).unwrap();

    // Create a plane that intersects the large box but not others
    let point = Point3::new(0.0, 0.0, 9.0);
    let direction = Direction3::Z;
    let plane = Plane3::new(point, direction).unwrap();

    let result = sortBox.compare_plane(plane).unwrap();

    // Only the large box should intersect this plane
    assert_eq!(1, result.extent(), "Expected to find 1 intersection");
    assert_eq!(
        Some(&2),
        result.first(),
        "Large box (index 2) should be the only result"
    );
}

#[test]
fn bounds_bound_sort_box_void_boxes() {
    let f = SetUp();
    let void_box = BoundingBox3::default(); // Default constructed box is void
    let boxes = two_boxes(void_box.clone(), f.mySmallBox.clone());

    let mut sortBox = BoundSortBox::default();
    sortBox.initialize(boxes).unwrap();

    // Test comparing with mySmallBox - should only find itself
    let result = sortBox.compare_box(&f.mySmallBox).unwrap();
    assert_eq!(1, result.extent(), "Expected to find 1 intersection");
    assert_eq!(
        Some(&2),
        result.first(),
        "Small box (index 2) should be the only result"
    );

    // Test comparing with void box - should find nothing
    let void_result = sortBox.compare_box(&void_box).unwrap();
    assert_eq!(
        0,
        void_result.extent(),
        "Expected to find 0 intersections with void box"
    );
}

#[test]
fn bounds_bound_sort_box_touching_boxes() {
    // Create boxes that touch at a single point
    let box1 = CreateBox(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let box2 = CreateBox(1.0, 1.0, 1.0, 2.0, 2.0, 2.0); // Touches box1 at (1,1,1)

    let mut sortBox = BoundSortBox::default();
    sortBox
        .initialize(two_boxes(box1.clone(), box2.clone()))
        .unwrap();

    // Test comparing with box1
    let result1 = sortBox.compare_box(&box1).unwrap();
    assert_eq!(2, result1.extent(), "Expected to find 2 intersections");

    // Test comparing with box2
    let result2 = sortBox.compare_box(&box2).unwrap();
    assert_eq!(2, result2.extent(), "Expected to find 2 intersections");
}

#[test]
fn bounds_bound_sort_box_disjoint_boxes() {
    let f = SetUp();
    let farBox = CreateBox(100.0, 100.0, 100.0, 110.0, 110.0, 110.0);
    let boxes = two_boxes(f.mySmallBox.clone(), farBox);

    // Create enclosing box large enough
    let enclosingBox = CreateBox(-10.0, -10.0, -10.0, 120.0, 120.0, 120.0);

    let mut sortBox = BoundSortBox::default();
    sortBox
        .initialize_with_enclosing(&enclosingBox, boxes)
        .unwrap();

    // Test comparing with a box near mySmallBox
    let testBox = CreateBox(0.5, 0.5, 0.5, 1.5, 1.5, 1.5);
    let result = sortBox.compare_box(&testBox).unwrap();
    assert_eq!(1, result.extent(), "Expected to find 1 intersection");
    assert_eq!(
        Some(&1),
        result.first(),
        "Small box (index 1) should be the only result"
    );
}

#[test]
fn bounds_bound_sort_box_degenerate_boxes() {
    // Create degenerate boxes
    let mut pointBox = BoundingBox3::default();
    pointBox.update_bounds([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]); // Point at (1,1,1)
    let mut lineBox = BoundingBox3::default();
    lineBox.update_bounds([2.0, 0.0, 0.0], [5.0, 0.0, 0.0]); // Line along X from (2,0,0) to (5,0,0)
    let mut planeBox = BoundingBox3::default();
    planeBox.update_bounds([0.0, 0.0, 3.0], [3.0, 3.0, 3.0]); // XY plane at Z=3

    let mut boxes = Array1::with_bounds(1, 3).unwrap();
    boxes[1] = pointBox;
    boxes[2] = lineBox;
    boxes[3] = planeBox;

    let mut sortBox = BoundSortBox::default();
    sortBox.initialize(boxes).unwrap();

    // Test with a box that should intersect all three degenerate boxes
    let testBox = CreateBox(0.0, 0.0, 0.0, 6.0, 6.0, 6.0);
    let result = sortBox.compare_box(&testBox).unwrap();
    assert_eq!(
        3,
        result.extent(),
        "Expected to find 3 intersections with degenerate boxes"
    );

    // Test with a box that should intersect only the point
    let pointTestBox = CreateBox(0.5, 0.5, 0.5, 1.5, 1.5, 1.5);
    let pointResult = sortBox.compare_box(&pointTestBox).unwrap();
    assert_eq!(
        1,
        pointResult.extent(),
        "Expected to find only the point box"
    );
    assert_eq!(
        Some(&1),
        pointResult.first(),
        "Point box (index 1) should be the only result"
    );
}
