#![allow(clippy::excessive_precision)]
const POINT: &[&[f64]] = &[
    include!("kronrod_point_0.in"),
    include!("kronrod_point_1.in"),
    include!("kronrod_point_2.in"),
    include!("kronrod_point_3.in"),
    include!("kronrod_point_4.in"),
    include!("kronrod_point_5.in"),
    include!("kronrod_point_6.in"),
    include!("kronrod_point_7.in"),
];
pub(super) fn point(index: usize) -> f64 {
    POINT[index / 256][index % 256]
}
const WEIGHT: &[&[f64]] = &[
    include!("kronrod_weight_0.in"),
    include!("kronrod_weight_1.in"),
    include!("kronrod_weight_2.in"),
    include!("kronrod_weight_3.in"),
    include!("kronrod_weight_4.in"),
    include!("kronrod_weight_5.in"),
    include!("kronrod_weight_6.in"),
    include!("kronrod_weight_7.in"),
];
pub(super) fn weight(index: usize) -> f64 {
    WEIGHT[index / 256][index % 256]
}
