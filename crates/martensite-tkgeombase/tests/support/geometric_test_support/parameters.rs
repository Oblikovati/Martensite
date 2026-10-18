//! Retained development-test parameter grid fixture.
#![allow(non_snake_case, dead_code)]
/// The `CreateUniformParams` helper of the Geom2dGridEval suites: `theNbPoints`
/// parameters from `theFirst` to `theLast`, indexed from 1.
pub fn CreateUniformParams(
    theFirst: f64,
    theLast: f64,
    theNbPoints: i32,
) -> martensite_tkernel::ncollection::Array1<'static, f64> {
    let mut aParams = martensite_tkernel::ncollection::Array1::with_bounds(1, theNbPoints).unwrap();
    let aStep: f64 = (theLast - theFirst) / f64::from(theNbPoints - 1);
    for i in 1..=theNbPoints {
        aParams[i] = theFirst + f64::from(i - 1) * aStep;
    }
    aParams
}
