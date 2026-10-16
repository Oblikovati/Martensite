//=================================================================================================
// Class2d Tests
//=================================================================================================

// Test point inside a simple square polygon
#[test]
fn surface_class2d_si_dans_point_inside_square() {
    let aPnts = unitSquare();

    let aClassifier = Class2d::from_array(&aPnts, 1e-10, 1e-10, 0.0, 0.0, 1.0, 1.0);

    // Point clearly inside
    let aPointInside = Point2::new(0.5, 0.5);
    assert_eq!(aClassifier.si_dans(aPointInside) as i32, 1);
}

#[test]
fn surface_class2d_si_dans_point_outside_square() {
    let aPnts = unitSquare();

    let aClassifier = Class2d::from_array(&aPnts, 1e-10, 1e-10, 0.0, 0.0, 1.0, 1.0);

    // Point clearly outside
    let aPointOutside = Point2::new(2.0, 2.0);
    assert_eq!(aClassifier.si_dans(aPointOutside) as i32, -1);
}

#[test]
fn surface_class2d_si_dans_point_on_boundary() {
    let aPnts = unitSquare();

    let aClassifier = Class2d::from_array(&aPnts, 0.01, 0.01, 0.0, 0.0, 1.0, 1.0);

    // Point on boundary (with tolerance)
    let aPointOnEdge = Point2::new(0.5, 0.0);
    // Point on boundary should return 0 (uncertain)
    assert_eq!(aClassifier.si_dans(aPointOnEdge) as i32, 0);
}

#[test]
fn surface_class2d_deep_copy_preserves_classification() {
    let aPnts = unitSquare();

    let aSource = Class2d::from_array(&aPnts, 0.01, 0.01, 0.0, 0.0, 1.0, 1.0);
    let aCopy = aSource.clone();
    let mut anAssigned = Class2d::default();
    anAssigned.clone_from(&aSource);

    let aSamples = [
        Point2::new(0.5, 0.5),
        Point2::new(2.0, 2.0),
        Point2::new(0.5, 0.0),
    ];
    for aPoint in aSamples {
        let aState: Class2dResult = aSource.si_dans(aPoint);
        assert_eq!(aCopy.si_dans(aPoint), aState);
        assert_eq!(anAssigned.si_dans(aPoint), aState);
    }
}

#[test]
fn surface_class2d_si_dans_triangular_polygon() {
    let mut aPnts = Array1::with_bounds(1, 3).unwrap();
    aPnts[1] = Point2::new(0.0, 0.0);
    aPnts[2] = Point2::new(2.0, 0.0);
    aPnts[3] = Point2::new(1.0, 2.0);

    let aClassifier = Class2d::from_array(&aPnts, 1e-10, 1e-10, 0.0, 0.0, 2.0, 2.0);

    // Point inside triangle (centroid)
    let aPointInside = Point2::new(1.0, 0.5);
    assert_eq!(aClassifier.si_dans(aPointInside) as i32, 1);

    // Point outside triangle
    let aPointOutside = Point2::new(0.0, 1.0);
    assert_eq!(aClassifier.si_dans(aPointOutside) as i32, -1);
}

// Test with Sequence constructor
#[test]
fn surface_class2d_sequence_constructor_point_inside_square() {
    let mut aPnts: Sequence<Point2> = Sequence::default();
    aPnts.append(Point2::new(0.0, 0.0));
    aPnts.append(Point2::new(1.0, 0.0));
    aPnts.append(Point2::new(1.0, 1.0));
    aPnts.append(Point2::new(0.0, 1.0));

    let aClassifier = Class2d::from_sequence(&aPnts, 1e-10, 1e-10, 0.0, 0.0, 1.0, 1.0);

    let aPointInside = Point2::new(0.5, 0.5);
    assert_eq!(aClassifier.si_dans(aPointInside) as i32, 1);
}

// Test InternalSiDans with normalized coordinates
#[test]
fn surface_class2d_internal_si_dans_normalized_coordinates() {
    let mut aPnts = Array1::with_bounds(1, 4).unwrap();
    aPnts[1] = Point2::new(0.0, 0.0);
    aPnts[2] = Point2::new(10.0, 0.0);
    aPnts[3] = Point2::new(10.0, 10.0);
    aPnts[4] = Point2::new(0.0, 10.0);

    let aClassifier = Class2d::from_array(&aPnts, 0.1, 0.1, 0.0, 0.0, 10.0, 10.0);

    // Test classification with actual coordinates
    assert_eq!(
        aClassifier.si_dans(Point2::new(5.0, 5.0)),
        Class2dResult::Inside
    );
    assert_eq!(
        aClassifier.si_dans(Point2::new(15.0, 15.0)),
        Class2dResult::Outside
    );
}
