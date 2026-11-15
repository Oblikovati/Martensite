//! The public operation certifies represented coordinates, independently of chart offset (#446).
use super::*;
fn issue_resolution() -> Resolution {
    Resolution::new(
        1e-12,
        1e-11,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}
mod controls;
mod fixtures;
#[test]
fn translated_unit_square_keeps_its_certified_area() {
    let result = build_planar_regions(
        &fixtures::rectangle(1e8, [1.; 2], false),
        issue_resolution(),
    )
    .unwrap();
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(1.));
}
#[test]
fn translated_decimal_square_uses_exact_represented_width() {
    let result = build_planar_regions(
        &fixtures::rectangle(1e8, [0.01; 2], false),
        issue_resolution(),
    )
    .unwrap();
    let represented_area = 450_360_445_921.0 / 4_503_599_627_370_496.0;
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(represented_area));
}
