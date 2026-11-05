//! Direct translation of ChFi3dHatching tests.
#![allow(non_snake_case, non_upper_case_globals)]
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkfillet::chfi3d::compute_hatching_domains;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeomalgo::geom2d_hatch::{Hatcher, Intersector};
use martensite_tkgeombase::gc::MakeSegment2d;
use martensite_tkmath::gp::{Axis2d, Direction2, Point2};

fn AddContour(theHatcher: &mut Hatcher, thePoints: &[Point2]) {
    for anIndex in 0..thePoints.len() {
        let aNextIndex = if anIndex == thePoints.len() - 1 {
            0
        } else {
            anIndex + 1
        };
        let aSegment: Handle<dyn Geom2dCurve> =
            MakeSegment2d::from_points(thePoints[anIndex], thePoints[aNextIndex])
                .unwrap()
                .value()
                .unwrap()
                .cast();
        theHatcher
            .add_element_curve(&aSegment, Orientation::Forward)
            .unwrap();
    }
}

fn AddHorizontalHatching(theHatcher: &mut Hatcher) -> i32 {
    let aLine: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::new(
        Point2::new(0.0, 0.0),
        Direction2::new(1.0, 0.0).unwrap(),
    )))
    .cast();
    theHatcher
        .trim_curve(Geom2dAdaptorCurve::with_curve(&aLine).unwrap())
        .unwrap()
}

/// The hatcher of the tests: tolerances 1e-7, no points nor segments kept.
fn NewHatcher() -> Hatcher {
    const aTolerance: f64 = 1.0e-7;
    let anIntersector = Intersector::new(aTolerance, aTolerance);
    Hatcher::new(&anIntersector, aTolerance, aTolerance, false, false)
}

/// Asserts the hatching has the one domain [first, last].
fn AssertSingleDomain(aHatcher: &Hatcher, aHatchingIndex: i32, first: f64, last: f64) {
    assert_eq!(aHatcher.nb_domains(aHatchingIndex).unwrap(), 1);
    let aDomain = aHatcher.domain(aHatchingIndex, 1).unwrap();
    assert!(aDomain.has_first_point());
    assert!(aDomain.has_second_point());
    assert!((aDomain.first_point().unwrap().parameter() - first).abs() <= precision::p_confusion());
    assert!((aDomain.second_point().unwrap().parameter() - last).abs() <= precision::p_confusion());
}

#[test]
fn ChFi3dHatchingTest__CoincidentSegmentFormsLimitingDomain() {
    let mut aHatcher = NewHatcher();
    let aContour = [
        Point2::new(0.0, 0.0),
        Point2::new(10.0, 0.0),
        Point2::new(10.0, 10.0),
        Point2::new(0.0, 10.0),
    ];
    AddContour(&mut aHatcher, &aContour);

    let aHatchingIndex = AddHorizontalHatching(&mut aHatcher);
    assert!(compute_hatching_domains(&mut aHatcher, aHatchingIndex).unwrap());
    AssertSingleDomain(&aHatcher, aHatchingIndex, 0.0, 10.0);
}

#[test]
fn ChFi3dHatchingTest__BoundaryOverlapDoesNotDuplicateInteriorDomain() {
    let mut aHatcher = NewHatcher();
    let aContour = [
        Point2::new(-10.0, -10.0),
        Point2::new(0.0, -10.0),
        Point2::new(0.0, 0.0),
        Point2::new(10.0, 0.0),
        Point2::new(10.0, 10.0),
        Point2::new(-10.0, 10.0),
    ];
    AddContour(&mut aHatcher, &aContour);

    let aHatchingIndex = AddHorizontalHatching(&mut aHatcher);
    assert!(compute_hatching_domains(&mut aHatcher, aHatchingIndex).unwrap());
    AssertSingleDomain(&aHatcher, aHatchingIndex, -10.0, 0.0);
}
