//! Grid eval parabola construction and evaluation contracts.
#![allow(non_snake_case)]
#[path = "common/mod.rs"]
mod common;
use common::{CreateUniformParams, EXPECT_NEAR};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomParabola};
use martensite_tkg3d::geom_grid_eval::{
    GeomGridEvalCurveD1, GeomGridEvalCurveD2, GeomGridEvalCurveD3, GeomGridEvalParabola,
};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

const THE_TOLERANCE: f64 = 1e-10;

// `new GeomParabola(Axis2(Point3(0, 0, 0), Direction3(0, 0, 1)), 1.0)`.
fn Parabola() -> Handle<GeomParabola> {
    let anAxis = Axis2::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    Handle::new(GeomParabola::new(anAxis, 1.0).unwrap())
}

#[test]
fn parabola_basic_evaluation() {
    // Parabola in XY plane, Focal=1.0, center at origin
    // Y^2 = 4*F*X => Y^2 = 4X
    let aParab = Parabola();

    // By default in the implementation:
    // XAxis is symmetry axis.
    // P(u) = O + u^2/(4f)*XDir + u*YDir
    // So X = u^2/4, Y = u

    let anEval = GeomGridEvalParabola::new(aParab.clone());
    assert!(!anEval.geometry().is_null());

    // Test from -2 to 2
    let aParams = CreateUniformParams(-2.0, 2.0, 5);

    let aGrid: Array1<Point3> = anEval.evaluate_grid(&aParams).unwrap();
    assert_eq!(aGrid.len(), 5);

    // Verify points
    for i in 1..=5 {
        let aExpected: Point3 = aParab.value(aParams[i]).unwrap();
        EXPECT_NEAR(aGrid[i].distance(&aExpected), 0.0, THE_TOLERANCE);
    }
}

#[test]
fn parabola_derivative_d1() {
    let aParab = Parabola();
    let anEval = GeomGridEvalParabola::new(aParab.clone());

    let aParams = CreateUniformParams(-2.0, 2.0, 9);

    let aGrid: Array1<GeomGridEvalCurveD1> = anEval.evaluate_grid_d1(&aParams).unwrap();

    for i in 1..=9 {
        let (aPnt, aD1) = aParab.d1(aParams[i]).unwrap();
        EXPECT_NEAR(aGrid[i].point.distance(&aPnt), 0.0, THE_TOLERANCE);
        EXPECT_NEAR((aGrid[i].d1 - aD1).magnitude(), 0.0, THE_TOLERANCE);
    }
}

#[test]
fn parabola_derivative_d2() {
    let aParab = Parabola();
    let anEval = GeomGridEvalParabola::new(aParab.clone());

    let aParams = CreateUniformParams(-2.0, 2.0, 9);

    let aGrid: Array1<GeomGridEvalCurveD2> = anEval.evaluate_grid_d2(&aParams).unwrap();

    for i in 1..=9 {
        let (aPnt, aD1, aD2) = aParab.d2(aParams[i]).unwrap();
        EXPECT_NEAR(aGrid[i].point.distance(&aPnt), 0.0, THE_TOLERANCE);
        EXPECT_NEAR((aGrid[i].d1 - aD1).magnitude(), 0.0, THE_TOLERANCE);
        EXPECT_NEAR((aGrid[i].d2 - aD2).magnitude(), 0.0, THE_TOLERANCE);
    }
}

#[test]
fn parabola_derivative_d3() {
    let aParab = Parabola();
    let anEval = GeomGridEvalParabola::new(aParab.clone());

    let aParams = CreateUniformParams(-2.0, 2.0, 9);

    let aGrid: Array1<GeomGridEvalCurveD3> = anEval.evaluate_grid_d3(&aParams).unwrap();

    for i in 1..=9 {
        let (aPnt, aD1, aD2, aD3) = aParab.d3(aParams[i]).unwrap();
        EXPECT_NEAR(aGrid[i].point.distance(&aPnt), 0.0, THE_TOLERANCE);
        EXPECT_NEAR((aGrid[i].d1 - aD1).magnitude(), 0.0, THE_TOLERANCE);
        EXPECT_NEAR((aGrid[i].d2 - aD2).magnitude(), 0.0, THE_TOLERANCE);
        EXPECT_NEAR((aGrid[i].d3 - aD3).magnitude(), 0.0, THE_TOLERANCE);
    }
}
