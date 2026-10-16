// ============================================================================
// Line Tests
// ============================================================================

#[test]
fn hash_curve_hasher_line_copied_lines_same_hash() {
    let myHasher = SetUp();
    let aLoc = pnt(1.0, 2.0);
    let aDir = dir(1.0, 0.0);
    let aLine1 = line(aLoc, aDir);
    let aLine2: Handle<Geom2dLine> = copied(&aLine1);

    assert_eq!(
        myHasher.hash(&curve(&aLine1)),
        myHasher.hash(&curve(&aLine2))
    );
    assert!(myHasher.equal(&curve(&aLine1), &curve(&aLine2)));
}

#[test]
fn hash_curve_hasher_line_different_lines_different_hash() {
    let myHasher = SetUp();
    let aLine1 = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    let aLine2 = line(pnt(1.0, 0.0), dir(1.0, 0.0));

    assert_ne!(
        myHasher.hash(&curve(&aLine1)),
        myHasher.hash(&curve(&aLine2))
    );
    assert!(!myHasher.equal(&curve(&aLine1), &curve(&aLine2)));
}

// ============================================================================
// Circle Tests
// ============================================================================

#[test]
fn hash_curve_hasher_circle_copied_circles_same_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let aCircle1 = circle(anAxis, 5.0);
    let aCircle2: Handle<Geom2dCircle> = copied(&aCircle1);

    assert_eq!(
        myHasher.hash(&curve(&aCircle1)),
        myHasher.hash(&curve(&aCircle2))
    );
    assert!(myHasher.equal(&curve(&aCircle1), &curve(&aCircle2)));
}

#[test]
fn hash_curve_hasher_circle_different_radius_different_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let aCircle1 = circle(anAxis, 5.0);
    let aCircle2 = circle(anAxis, 10.0);

    assert_ne!(
        myHasher.hash(&curve(&aCircle1)),
        myHasher.hash(&curve(&aCircle2))
    );
    assert!(!myHasher.equal(&curve(&aCircle1), &curve(&aCircle2)));
}

// ============================================================================
// Ellipse Tests
// ============================================================================

#[test]
fn hash_curve_hasher_ellipse_copied_ellipses_same_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let anEllipse1 = Handle::new(Geom2dEllipse::new(anAxis, 10.0, 5.0).unwrap());
    let anEllipse2: Handle<Geom2dEllipse> = copied(&anEllipse1);

    assert_eq!(
        myHasher.hash(&curve(&anEllipse1)),
        myHasher.hash(&curve(&anEllipse2))
    );
    assert!(myHasher.equal(&curve(&anEllipse1), &curve(&anEllipse2)));
}

#[test]
fn hash_curve_hasher_ellipse_different_radii_different_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let anEllipse1 = Handle::new(Geom2dEllipse::new(anAxis, 10.0, 5.0).unwrap());
    let anEllipse2 = Handle::new(Geom2dEllipse::new(anAxis, 10.0, 7.0).unwrap());

    assert_ne!(
        myHasher.hash(&curve(&anEllipse1)),
        myHasher.hash(&curve(&anEllipse2))
    );
    assert!(!myHasher.equal(&curve(&anEllipse1), &curve(&anEllipse2)));
}

// ============================================================================
// Hyperbola Tests
// ============================================================================

#[test]
fn hash_curve_hasher_hyperbola_copied_hyperbolas_same_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let aHyp1 = Handle::new(Geom2dHyperbola::new(anAxis, 5.0, 3.0).unwrap());
    let aHyp2: Handle<Geom2dHyperbola> = copied(&aHyp1);

    assert_eq!(myHasher.hash(&curve(&aHyp1)), myHasher.hash(&curve(&aHyp2)));
    assert!(myHasher.equal(&curve(&aHyp1), &curve(&aHyp2)));
}

#[test]
fn hash_curve_hasher_hyperbola_different_radii_different_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let aHyp1 = Handle::new(Geom2dHyperbola::new(anAxis, 5.0, 3.0).unwrap());
    let aHyp2 = Handle::new(Geom2dHyperbola::new(anAxis, 5.0, 4.0).unwrap());

    assert_ne!(myHasher.hash(&curve(&aHyp1)), myHasher.hash(&curve(&aHyp2)));
    assert!(!myHasher.equal(&curve(&aHyp1), &curve(&aHyp2)));
}

// ============================================================================
// Parabola Tests
// ============================================================================

#[test]
fn hash_curve_hasher_parabola_copied_parabolas_same_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let aPar1 = Handle::new(Geom2dParabola::new(anAxis, 2.0).unwrap());
    let aPar2: Handle<Geom2dParabola> = copied(&aPar1);

    assert_eq!(myHasher.hash(&curve(&aPar1)), myHasher.hash(&curve(&aPar2)));
    assert!(myHasher.equal(&curve(&aPar1), &curve(&aPar2)));
}

#[test]
fn hash_curve_hasher_parabola_different_focal_different_hash() {
    let myHasher = SetUp();
    let anAxis = standard_axes();
    let aPar1 = Handle::new(Geom2dParabola::new(anAxis, 2.0).unwrap());
    let aPar2 = Handle::new(Geom2dParabola::new(anAxis, 3.0).unwrap());

    assert_ne!(myHasher.hash(&curve(&aPar1)), myHasher.hash(&curve(&aPar2)));
    assert!(!myHasher.equal(&curve(&aPar1), &curve(&aPar2)));
}

// ============================================================================
// BezierCurve Tests
// ============================================================================

#[test]
fn hash_curve_hasher_bezier_curve_copied_curves_same_hash() {
    let myHasher = SetUp();
    let aPoles = poles(&[(0.0, 0.0), (1.0, 2.0), (3.0, 2.0), (4.0, 0.0)]);

    let aCurve1 = Handle::new(Geom2dBezierCurve::new(&aPoles).unwrap());
    let aCurve2: Handle<Geom2dBezierCurve> = copied(&aCurve1);

    assert_eq!(
        myHasher.hash(&curve(&aCurve1)),
        myHasher.hash(&curve(&aCurve2))
    );
    assert!(myHasher.equal(&curve(&aCurve1), &curve(&aCurve2)));
}

#[test]
fn hash_curve_hasher_bezier_curve_different_poles_different_comparison() {
    let myHasher = SetUp();
    let aPoles1 = poles(&[(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)]);
    // Different middle pole
    let aPoles2 = poles(&[(0.0, 0.0), (1.0, 3.0), (2.0, 0.0)]);

    let aCurve1 = Handle::new(Geom2dBezierCurve::new(&aPoles1).unwrap());
    let aCurve2 = Handle::new(Geom2dBezierCurve::new(&aPoles2).unwrap());

    // Hash may be same (metadata only), but comparison should differ
    assert!(!myHasher.equal(&curve(&aCurve1), &curve(&aCurve2)));
}

// ============================================================================
// BSplineCurve Tests
// ============================================================================

#[test]
fn hash_curve_hasher_b_spline_curve_copied_curves_same_hash() {
    let myHasher = SetUp();
    let aPoles = poles(&[(0.0, 0.0), (1.0, 1.0), (2.0, 1.0), (3.0, 0.0)]);
    let aKnots = reals(&[0.0, 1.0]);
    let aMults = integers(&[4, 4]);

    let aCurve1 =
        Handle::new(Geom2dBSplineCurve::new(&aPoles, &aKnots, &aMults, 3, false).unwrap());
    let aCurve2: Handle<Geom2dBSplineCurve> = copied(&aCurve1);

    assert_eq!(
        myHasher.hash(&curve(&aCurve1)),
        myHasher.hash(&curve(&aCurve2))
    );
    assert!(myHasher.equal(&curve(&aCurve1), &curve(&aCurve2)));
}

// ============================================================================
// TrimmedCurve Tests
// ============================================================================

#[test]
fn hash_curve_hasher_trimmed_curve_copied_curves_same_hash() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    let aTrimmed1 =
        Handle::new(Geom2dTrimmedCurve::new(&curve(&aLine), 0.0, 10.0, true, true).unwrap());
    let aTrimmed2: Handle<Geom2dTrimmedCurve> = copied(&aTrimmed1);

    assert_eq!(
        myHasher.hash(&curve(&aTrimmed1)),
        myHasher.hash(&curve(&aTrimmed2))
    );
    assert!(myHasher.equal(&curve(&aTrimmed1), &curve(&aTrimmed2)));
}

#[test]
fn hash_curve_hasher_trimmed_curve_different_bounds_different_hash() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    let aTrimmed1 =
        Handle::new(Geom2dTrimmedCurve::new(&curve(&aLine), 0.0, 10.0, true, true).unwrap());
    let aTrimmed2 =
        Handle::new(Geom2dTrimmedCurve::new(&curve(&aLine), 0.0, 20.0, true, true).unwrap());

    assert_ne!(
        myHasher.hash(&curve(&aTrimmed1)),
        myHasher.hash(&curve(&aTrimmed2))
    );
    assert!(!myHasher.equal(&curve(&aTrimmed1), &curve(&aTrimmed2)));
}

// ============================================================================
// OffsetCurve Tests
// ============================================================================

#[test]
fn hash_curve_hasher_offset_curve_copied_curves_same_hash() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    let anOffset1 = Handle::new(Geom2dOffsetCurve::new(&curve(&aLine), 5.0, false).unwrap());
    let anOffset2: Handle<Geom2dOffsetCurve> = copied(&anOffset1);

    assert_eq!(
        myHasher.hash(&curve(&anOffset1)),
        myHasher.hash(&curve(&anOffset2))
    );
    assert!(myHasher.equal(&curve(&anOffset1), &curve(&anOffset2)));
}

#[test]
fn hash_curve_hasher_offset_curve_different_offset_different_hash() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    let anOffset1 = Handle::new(Geom2dOffsetCurve::new(&curve(&aLine), 5.0, false).unwrap());
    let anOffset2 = Handle::new(Geom2dOffsetCurve::new(&curve(&aLine), 10.0, false).unwrap());

    assert_ne!(
        myHasher.hash(&curve(&anOffset1)),
        myHasher.hash(&curve(&anOffset2))
    );
    assert!(!myHasher.equal(&curve(&anOffset1), &curve(&anOffset2)));
}

// ============================================================================
// Cross-type Tests
// ============================================================================

#[test]
fn hash_curve_hasher_different_types_different_comparison() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    let anAxis = standard_axes();
    let aCircle = circle(anAxis, 5.0);

    assert!(!myHasher.equal(&curve(&aLine), &curve(&aCircle)));
}

#[test]
fn hash_curve_hasher_null_curves_handled_correctly() {
    let myHasher = SetUp();
    let aNullCurve: Handle<dyn Geom2dCurve> = Handle::default();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));

    assert_eq!(myHasher.hash(&aNullCurve), 0);
    assert!(myHasher.equal(&aNullCurve, &aNullCurve));
    assert!(!myHasher.equal(&curve(&aLine), &aNullCurve));
}

#[test]
fn hash_curve_hasher_same_object_equal() {
    let myHasher = SetUp();
    let aLine = line(pnt(0.0, 0.0), dir(1.0, 0.0));
    assert!(myHasher.equal(&curve(&aLine), &curve(&aLine)));
}

// ============================================================================
// Weighted BSpline Tests
// ============================================================================

#[test]
fn hash_curve_hasher_b_spline_curve_weighted_copied_curves_same_hash() {
    let myHasher = SetUp();
    let aPoles = poles(&[(0.0, 0.0), (1.0, 1.0), (2.0, 1.0), (3.0, 0.0)]);
    let aWeights = reals(&[1.0, 2.0, 2.0, 1.0]);
    let aKnots = reals(&[0.0, 1.0]);
    let aMults = integers(&[4, 4]);

    let aCurve1 = Handle::new(
        Geom2dBSplineCurve::with_weights(&aPoles, &aWeights, &aKnots, &aMults, 3, false).unwrap(),
    );
    let aCurve2: Handle<Geom2dBSplineCurve> = copied(&aCurve1);

    assert_eq!(
        myHasher.hash(&curve(&aCurve1)),
        myHasher.hash(&curve(&aCurve2))
    );
    assert!(myHasher.equal(&curve(&aCurve1), &curve(&aCurve2)));
}
