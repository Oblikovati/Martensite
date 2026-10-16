#![allow(clippy::excessive_precision)]
const POINT: &[&[f64]] = &[
    include!("gauss_point_0.in"),
    include!("gauss_point_1.in"),
    include!("gauss_point_2.in"),
    include!("gauss_point_3.in"),
];
pub(super) fn point(index: usize) -> f64 {
    POINT[index / 256][index % 256]
}
const WEIGHT: &[&[f64]] = &[
    include!("gauss_weight_0.in"),
    include!("gauss_weight_1.in"),
    include!("gauss_weight_2.in"),
    include!("gauss_weight_3.in"),
];
pub(super) fn weight(index: usize) -> f64 {
    WEIGHT[index / 256][index % 256]
}
