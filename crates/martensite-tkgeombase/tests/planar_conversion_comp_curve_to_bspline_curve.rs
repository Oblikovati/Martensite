//! Geom2d convert comp curve to bspline curve contracts.
#![allow(non_snake_case)]
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBoundedCurve, Geom2dCircle, Geom2dCurve, Geom2dTrimmedCurve,
};
use martensite_tkgeombase::gc::MakeCircle2d as GC_MakeCircle2d;
use martensite_tkgeombase::geom2d_convert::CompCurveToBSplineCurve as Geom2dConvert_CompCurveToBSplineCurve;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::gp::Point2;

fn trimmed(circle: &Handle<Geom2dCircle>, u1: f64, u2: f64) -> Handle<Geom2dTrimmedCurve> {
    let basis: Handle<dyn Geom2dCurve> = circle.cast();
    Handle::new(Geom2dTrimmedCurve::new(&basis, u1, u2, true, true).unwrap())
}

#[test]
fn comp_curve_to_b_spline_curve_closed_contour_from_circle_arcs() {
    // the boundary regression: 2d Curves concatenator must properly handle closed contours.
    // Split a full circle into 10 arcs and assemble them into a closed BSpline.
    let aCirc: Handle<Geom2dCircle> =
        Handle::try_from(&GC_MakeCircle2d::from_center(Point2::new(0.0, 0.0), 50.0, true).unwrap())
            .unwrap();
    assert!(!aCirc.is_null());

    let aF: f64 = aCirc.first_parameter();
    let aL: f64 = aCirc.last_parameter();
    let aNb: i32 = 10;
    let aDelta: f64 = (aF + aL) / f64::from(aNb);

    let aFTrim = trimmed(&aCirc, aF, aDelta);
    let mut aRes = Geom2dConvert_CompCurveToBSplineCurve::with_curve(
        &aFTrim.cast::<dyn Geom2dBoundedCurve>(),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();

    for anId in 1..aNb {
        let aLTrim: Handle<Geom2dTrimmedCurve> = if anId == (aNb - 1) {
            trimmed(&aCirc, f64::from(anId) * aDelta, aF)
        } else {
            trimmed(
                &aCirc,
                f64::from(anId) * aDelta,
                f64::from(anId + 1) * aDelta,
            )
        };
        aRes.add(
            &aLTrim.cast::<dyn Geom2dBoundedCurve>(),
            precision::p_confusion(),
            false,
        )
        .unwrap();
    }

    let aBSpline: Handle<Geom2dBSplineCurve> = aRes.bspline_curve();
    assert!(!aBSpline.is_null());
    assert!(
        aBSpline.is_closed().unwrap(),
        "Assembled BSpline curve from closed circle arcs must be closed"
    );
}
