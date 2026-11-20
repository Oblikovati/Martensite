use super::*;
use crate::surface_sketch::wrapped_profile::math_cancellation::MathCancellation;

#[test]
fn refused_and_cancelled_clearance_charge_after_prior_work() {
    for limit in [5, 1_000_000] {
        check_charged_refusal(limit);
    }
}
fn check_charged_refusal(limit: u32) {
    let region = SurfaceRegionId(1);
    let mut work = Work::new(ConeClearanceOptions {
        max_operations: 2_000_000,
        ..Default::default()
    });
    work.charge(region, None, 17).unwrap();
    let sources = [
        clearance_source(&[[0., 0.], [1., 0.]]),
        clearance_source(&[[0., 1.], [1., 1.]]),
    ];
    let mut options = work.remaining();
    options.max_operations = limit;
    let error = refused_clearance(&sources, options);
    let expected = error.clone();
    let ConeCause::Clearance(actual) = charge(Err(error), region, &mut work).unwrap_err().cause
    else {
        panic!("clearance cause was replaced");
    };
    assert_eq!(*actual, expected.cause);
    assert_eq!(work.consumed, 17 + expected.operations);
}
fn clearance_source(authored: &[[f64; 2]]) -> CylindricalClearanceSource<'_> {
    CylindricalClearanceSource {
        authored,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
    }
}
fn refused_clearance(
    sources: &[CylindricalClearanceSource<'_>],
    options: ConeClearanceOptions,
) -> CylindricalClearanceWorkError {
    let mut polls = 0;
    let error =
        certify_cylindrical_clearance_with_work(query(sources, &[], 0.125, options), || {
            polls += 1;
            polls > 10
        })
        .unwrap_err();
    assert!(error.operations > 0);
    assert_eq!(error.cause.is_cancelled(), options.max_operations > 5);
    error
}
