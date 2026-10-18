// The comparison loop of the point tests over `theCounts` = (NbU, NbV) cells: every
// grid point against `Value` (or `D0`, which natively is the same EvalD0 call).
fn CheckGridPoints(
    theSurf: &Handle<GeomBSplineSurface>,
    theGrid: &Array2<'_, Point3>,
    theUParams: &Array1<'_, f64>,
    theVParams: &Array1<'_, f64>,
    theCounts: (i32, i32),
) {
    for iU in 1..=theCounts.0 {
        for iV in 1..=theCounts.1 {
            let aExpected: Point3 = theSurf.value(theUParams[iU], theVParams[iV]).unwrap();
            EXPECT_NEAR((*theGrid.value(iU, iV).unwrap()).distance(&aExpected), 0.0, THE_TOLERANCE);
        }
    }
}

//=================================================================================================
// Basic Evaluation Tests
//=================================================================================================

#[test]
fn b_spline_surface_basic_evaluation() {
    let aSurf = CreateSimpleBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aSurf.clone());
    assert!(!anEval.geometry().is_null());

    let aUParams = CreateUniformParams(0.0, 1.0, 5);
    let aVParams = CreateUniformParams(0.0, 1.0, 5);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    CheckGridPoints(&aSurf, &aGrid, &aUParams, &aVParams, (5, 5));
}

#[test]
fn b_spline_surface_corner_points() {
    let aSurf = CreateSimpleBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aSurf);

    let mut aUParams: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    let mut aVParams: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    aUParams[1] = 0.0;
    aUParams[2] = 1.0;
    aVParams[1] = 0.0;
    aVParams[2] = 1.0;

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    EXPECT_NEAR(
        (*aGrid.value(1, 1).unwrap()).distance(&Point3::new(0.0, 0.0, 0.0)),
        0.0,
        THE_TOLERANCE,
    );
    EXPECT_NEAR(
        (*aGrid.value(2, 1).unwrap()).distance(&Point3::new(1.0, 0.0, 0.0)),
        0.0,
        THE_TOLERANCE,
    );
    EXPECT_NEAR(
        (*aGrid.value(1, 2).unwrap()).distance(&Point3::new(0.0, 1.0, 0.0)),
        0.0,
        THE_TOLERANCE,
    );
    EXPECT_NEAR(
        (*aGrid.value(2, 2).unwrap()).distance(&Point3::new(1.0, 1.0, 1.0)),
        0.0,
        THE_TOLERANCE,
    );
}

#[test]
fn b_spline_surface_rational_surface() {
    let aSurf = CreateRationalBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aSurf.clone());

    let aUParams = CreateUniformParams(0.0, 1.0, 11);
    let aVParams = CreateUniformParams(0.0, 1.0, 11);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    CheckGridPoints(&aSurf, &aGrid, &aUParams, &aVParams, (11, 11));
}

#[test]
fn b_spline_surface_multi_span_surface() {
    let aSurf = CreateMultiSpanBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aSurf.clone());

    let aUParams = CreateUniformParams(0.0, 1.0, 21);
    let aVParams = CreateUniformParams(0.0, 1.0, 21);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    CheckGridPoints(&aSurf, &aGrid, &aUParams, &aVParams, (21, 21));
}

#[test]
fn b_spline_surface_higher_degree() {
    let mut aPoles: Array2<'static, Point3> = Array2::with_bounds(1, 4, 1, 4).unwrap();
    for i in 1..=4 {
        for j in 1..=4 {
            let x: f64 = f64::from(i - 1) * 1.0;
            let y: f64 = f64::from(j - 1) * 1.0;
            let z: f64 = (f64::from(i + j - 2) * PI / 6.0).cos();
            aPoles.set_value(i, j, Point3::new(x, y, z)).unwrap();
        }
    }

    let mut aUKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    let mut aVKnots: Array1<'static, f64> = Array1::with_bounds(1, 2).unwrap();
    let mut aUMults: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    let mut aVMults: Array1<'static, i32> = Array1::with_bounds(1, 2).unwrap();
    aUKnots[1] = 0.0;
    aUKnots[2] = 1.0;
    aVKnots[1] = 0.0;
    aVKnots[2] = 1.0;
    aUMults[1] = 4;
    aUMults[2] = 4;
    aVMults[1] = 4;
    aVMults[2] = 4;

    let aSurf = Handle::new(
        GeomBSplineSurface::new(
            &aPoles,
            (&aUKnots, &aUMults),
            (&aVKnots, &aVMults),
            (3, 3),
            (false, false),
        )
        .unwrap(),
    );
    let anEval = GeomGridEvalBSplineSurface::new(aSurf.clone());

    let aUParams = CreateUniformParams(0.0, 1.0, 17);
    let aVParams = CreateUniformParams(0.0, 1.0, 17);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    CheckGridPoints(&aSurf, &aGrid, &aUParams, &aVParams, (17, 17));
}

//=================================================================================================
// Isoline Evaluation Tests
//=================================================================================================

// A one-element parameter array `aParams(1, 1)` holding `theValue`.
fn SingleParam(theValue: f64) -> Array1<'static, f64> {
    let mut aParams: Array1<'static, f64> = Array1::with_bounds(1, 1).unwrap();
    aParams[1] = theValue;
    aParams
}

#[test]
fn b_spline_surface_isoline_u_compare_to_geom_d0() {
    let aBSpline = CreateMultiSpanBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aBSpline.clone());

    // U-isoline: 1 U param, multiple V params (triggers isoline path)
    let aUParams = SingleParam(0.5);
    let aVParams = CreateUniformParams(0.0, 1.0, 15);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    // Compare against GeomBSplineSurface::D0
    // Note: ColLength() = number of U params (rows), RowLength() = number of V params (columns)
    let aNbU = aGrid.nb_rows() as i32;
    let aNbV = aGrid.nb_columns() as i32;
    CheckGridPoints(&aBSpline, &aGrid, &aUParams, &aVParams, (aNbU, aNbV));
}

#[test]
fn b_spline_surface_isoline_v_compare_to_geom_d0() {
    let aBSpline = CreateMultiSpanBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aBSpline.clone());

    // V-isoline: multiple U params, 1 V param (triggers isoline path)
    let aUParams = CreateUniformParams(0.0, 1.0, 15);
    let aVParams = SingleParam(0.7);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    // Compare against GeomBSplineSurface::D0
    let aNbU = aGrid.nb_rows() as i32;
    let aNbV = aGrid.nb_columns() as i32;
    CheckGridPoints(&aBSpline, &aGrid, &aUParams, &aVParams, (aNbU, aNbV));
}

#[test]
fn b_spline_surface_isoline_multi_span_compare_to_geom_d0() {
    let aBSpline = CreateMultiSpanBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aBSpline.clone());

    // U-isoline on multi-span surface
    let aUParams = SingleParam(0.35);
    let aVParams = CreateUniformParams(0.0, 1.0, 20);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    // Compare against GeomBSplineSurface::D0
    let aNbU = aGrid.nb_rows() as i32;
    let aNbV = aGrid.nb_columns() as i32;
    CheckGridPoints(&aBSpline, &aGrid, &aUParams, &aVParams, (aNbU, aNbV));
}

#[test]
fn b_spline_surface_isoline_rational_compare_to_geom_d0() {
    let aBSpline = CreateRationalBSplineSurface();
    let anEval = GeomGridEvalBSplineSurface::new(aBSpline.clone());

    // V-isoline on rational surface
    let aUParams = CreateUniformParams(0.0, 1.0, 15);
    let aVParams = SingleParam(0.3);

    let aGrid: Array2<Point3> = anEval.evaluate_grid(&aUParams, &aVParams).unwrap();

    // Compare against GeomBSplineSurface::D0
    let aNbU = aGrid.nb_rows() as i32;
    let aNbV = aGrid.nb_columns() as i32;
    CheckGridPoints(&aBSpline, &aGrid, &aUParams, &aVParams, (aNbU, aNbV));
}
