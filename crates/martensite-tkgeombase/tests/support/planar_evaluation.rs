//! Exact geometry/begin val2d sampling and original componentwise comparison policy.
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dCurveD2};
fn check_sample(reference: [[f64; 2]; 3], actual: Geom2dCurveD2) {
    let evaluated = [
        [actual.point.x(), actual.point.y()],
        [actual.d1.x(), actual.d1.y()],
        [actual.d2.x(), actual.d2.y()],
    ];
    for (expected, actual) in reference.iter().flatten().zip(evaluated.iter().flatten()) {
        assert!((expected - actual).abs() < 1e-6); // Original compare rejects >= tolerance, componentwise.
    }
}

/// Check original value/D1/D2 pairs; `check_val2d(curve, [0.,1.], 10, &references)`.
pub fn check_val2d(
    curve: &dyn Geom2dCurve,
    range: [f64; 2],
    intervals: i32,
    references: &[[[f64; 2]; 3]],
) {
    let count = visit_val2d(curve, range, intervals, |index, value| {
        check_sample(references[index], value);
    });
    assert_eq!(count, references.len());
}
fn visit_val2d(
    curve: &dyn Geom2dCurve,
    range: [f64; 2],
    intervals: i32,
    mut sample: impl FnMut(usize, Geom2dCurveD2),
) -> usize {
    let step = (range[1] - range[0]) / f64::from(intervals);
    let (mut parameter, mut index) = (range[0], 1);
    while parameter <= range[1] {
        sample(index - 1, curve.eval_d2(parameter).unwrap());
        index += 1;
        parameter = range[0] + index as f64 * step; // Original body increments i before for-update.
    }
    index - 1
}
/// Execute requests with no original reference pairs; `evaluate_val2d(curve, [0.,6.28], 20)`.
#[allow(dead_code)] // Paired-reference hosts use check_val2d; conics use this original no-reference form.
pub fn evaluate_val2d(curve: &dyn Geom2dCurve, range: [f64; 2], intervals: i32) -> usize {
    visit_val2d(curve, range, intervals, |_, _| {})
}
