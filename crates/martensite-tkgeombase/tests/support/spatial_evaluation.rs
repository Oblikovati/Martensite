//! Exact geometry/begin val3d repeated-addition parameter loop.
use martensite_tkg3d::geom::GeomCurve;
/// Evaluate every original value/D1/D2 request; `evaluate_val3d(curve, [0.,1.], 20)`.
pub fn evaluate_val3d(curve: &dyn GeomCurve, range: [f64; 2], intervals: i32) -> usize {
    let step = (range[1] - range[0]) / f64::from(intervals);
    let (mut parameter, mut count) = (range[0], 0);
    while parameter <= range[1] {
        let _point_and_derivatives = curve.eval_d2(parameter).unwrap();
        count += 1;
        parameter += step; // Unlike val2d, the original advances u by repeated addition.
    }
    count
}
