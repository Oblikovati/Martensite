// =======================================================================================
// Tests for BvhTraverse
// =======================================================================================

#[test]
fn hierarchy_traverse_count_all_elements() {
    let aBVH = CreateSimpleTriangulationBVH(10);

    let mut aSelector = BVH_CountAllElements::default();
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 10);
    assert_eq!(aSelector.AcceptedCount(), 10);
}

#[test]
fn hierarchy_traverse_empty_tree() {
    let aBVH = BvhTree::<BvhVec3d>::default();

    let mut aSelector = BVH_CountAllElements::default();
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 0);
}

#[test]
fn hierarchy_traverse_null_tree() {
    // A null handle.
    let aBVH: Option<&BvhTree<BvhVec3d>> = None;

    let mut aSelector = BVH_CountAllElements::default();
    let aCount = aSelector.select_tree(aBVH).unwrap();

    assert_eq!(aCount, 0);
}

#[test]
fn hierarchy_traverse_box_selection() {
    let aBVH = CreateSimpleTriangulationBVH(10);

    // Select elements in the first half
    let aBox = BvhBox::from_corners(BvhVec3d::new(0.0, 0.0, -1.0), BvhVec3d::new(10.0, 2.0, 1.0));
    let mut aSelector = BVH_BoxSelector::new(aBox);
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    // Should select approximately half of the elements
    assert!(aCount > 0);
    assert!(aCount < 10);
}

#[test]
fn hierarchy_traverse_empty_box_selection() {
    let aBVH = CreateSimpleTriangulationBVH(10);

    // Select with box that doesn't intersect any elements
    let aBox = BvhBox::from_corners(
        BvhVec3d::new(100.0, 100.0, 100.0),
        BvhVec3d::new(200.0, 200.0, 200.0),
    );
    let mut aSelector = BVH_BoxSelector::new(aBox);
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 0);
}

#[test]
fn hierarchy_traverse_full_box_selection() {
    let aBVH = CreateSimpleTriangulationBVH(10);

    // Select with box that contains all elements
    let aBox = BvhBox::from_corners(
        BvhVec3d::new(-100.0, -100.0, -100.0),
        BvhVec3d::new(100.0, 100.0, 100.0),
    );
    let mut aSelector = BVH_BoxSelector::new(aBox);
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 10);
}

#[test]
fn hierarchy_traverse_distance_based_selection() {
    let aBVH = CreateSimpleTriangulationBVH(10);

    // Find elements near a point
    let aPoint = BvhVec3d::new(5.0, 0.5, 0.0);
    let aMaxDist = 5.0;
    let mut aSelector = BVH_DistanceSelector::new(aPoint, aMaxDist);
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert!(aCount > 0);
    assert!(aCount <= 10);
    assert!(aSelector.ClosestIndex() >= 0);
    assert!(aSelector.ClosestIndex() < 10);
}

#[test]
fn hierarchy_traverse_early_termination() {
    let aBVH = CreateSimpleTriangulationBVH(100);

    // Stop after finding 5 elements
    let mut aSelector = BVH_LimitedSelector::new(5);
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 5);
    assert_eq!(aSelector.AcceptedCount(), 5);
}

#[test]
fn hierarchy_traverse_large_data_set() {
    let aBVH = CreateSimpleTriangulationBVH(1000);

    let mut aSelector = BVH_CountAllElements::default();
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 1000);
}

#[test]
fn hierarchy_traverse_metric_based_pruning() {
    let aBVH = CreateSimpleTriangulationBVH(50);

    // Very restrictive distance should result in few acceptances
    let aPoint = BvhVec3d::new(1000.0, 1000.0, 1000.0); // Far away
    let aMaxDist = 1.0; // Small radius
    let mut aSelector = BVH_DistanceSelector::new(aPoint, aMaxDist);
    let aCount = aSelector.select_tree(Some(&aBVH)).unwrap();

    assert_eq!(aCount, 0); // Nothing should be within range
}
