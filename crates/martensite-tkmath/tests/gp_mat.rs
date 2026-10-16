//! The default matrix initializes every coefficient.
#![allow(non_snake_case)]
use martensite_tkmath::gp::Matrix3;
#[test]
fn geometry_mat_default_constructor() {
    let matrix = Matrix3::default();
    for row in 1..=3 {
        for column in 1..=3 {
            assert_eq!(0.0, matrix.value(row, column).unwrap());
        }
    }
}
