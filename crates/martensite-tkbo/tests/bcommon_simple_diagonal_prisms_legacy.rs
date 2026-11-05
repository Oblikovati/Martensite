//! Regressions of diagonal and rotated prism common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::{Point3, Vector3};

fn prism(points: [[f64; 3]; 4], direction: [f64; 3]) -> Shape {
    let wire = CreatePolygonWire(
        &points.map(|point| Point3::new(point[0], point[1], point[2])),
        true,
    );
    let face = CreateFaceFromWire(&wire);
    CreatePrism(
        &face,
        Vector3::new(direction[0], direction[1], direction[2]),
    )
}

fn common(
    second_points: [[f64; 3]; 4],
    direction: [f64; 3],
    pivot: [f64; 3],
    angle: f64,
    area: f64,
) {
    let first = prism(
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        [0.0, 0.0, 1.0],
    );
    let mut second = prism(second_points, direction);
    RotateAroundAxis(
        &mut second,
        Point3::new(pivot[0], pivot[1], pivot[2]),
        [0.0, 0.0, 1.0],
        angle,
    );
    ValidateResult(&PerformCommon(&first, &second), area, -1.0, false);
}

macro_rules! diagonal_case {
    ($name:ident, $points:expr, $direction:expr, $pivot:expr, $angle:expr, $area:expr) => {
        #[test]
        fn $name() {
            common($points, $direction, $pivot, $angle, $area);
        }
    };
}

const SQRT2: f64 = std::f64::consts::SQRT_2 * 2.0;
const SQRT2_HALF: f64 = std::f64::consts::FRAC_1_SQRT_2;
const SQRT5: f64 = 2.23606797749979;

diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_F7,
    [
        [SQRT2, 0.0, 0.0],
        [SQRT2, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_F8,
    [
        [SQRT2, 0.0, 0.0],
        [SQRT2, SQRT2_HALF, 0.0],
        [0.0, SQRT2_HALF, 0.0],
        [0.0, 0.0, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_F9,
    [
        [SQRT2, 0.0, 0.0],
        [SQRT2, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0]
    ],
    [0.0, SQRT2_HALF, 0.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G1,
    [
        [0.0, SQRT2_HALF, 0.0],
        [0.0, SQRT2_HALF, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0]
    ],
    [SQRT2, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G2,
    [
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 1.0]
    ],
    [-0.5, 0.5, 0.0],
    [0.0, 0.0, 0.0],
    0.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G3,
    [
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 1.0]
    ],
    [-1.0, 1.0, 0.0],
    [0.0, 0.0, 0.0],
    0.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G4,
    [
        [0.0, 0.0, 0.0],
        [1.5, 0.0, 0.0],
        [1.5, SQRT2_HALF, 0.0],
        [0.0, SQRT2_HALF, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G5,
    [
        [0.0, SQRT2_HALF, 0.0],
        [1.5, SQRT2_HALF, 0.0],
        [1.5, 0.0, 0.0],
        [0.0, 0.0, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G6,
    [
        [0.0, 0.0, 1.0],
        [1.5, 0.0, 1.0],
        [1.5, SQRT2_HALF, 1.0],
        [0.0, SQRT2_HALF, 1.0]
    ],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G7,
    [
        [0.0, 0.0, 0.0],
        [SQRT5 / 2.0, 0.0, 0.0],
        [SQRT5 / 2.0, 1.0 / SQRT5, 0.0],
        [0.0, 1.0 / SQRT5, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    3.55724
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G8,
    [
        [0.0, 0.0, 1.0],
        [SQRT5 / 2.0, 0.0, 1.0],
        [SQRT5 / 2.0, 1.0 / SQRT5, 1.0],
        [0.0, 1.0 / SQRT5, 1.0]
    ],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 0.0],
    45.0,
    3.55724
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_G9,
    [
        [0.0, 0.5, 0.0],
        [SQRT2_HALF, 0.5, 0.0],
        [SQRT2_HALF, SQRT2_HALF, 0.0],
        [0.0, SQRT2_HALF, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    2.12132
);

diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_H1,
    [
        [0.0, 0.5, 0.0],
        [SQRT2_HALF, 0.5, 0.0],
        [SQRT2_HALF, SQRT2_HALF, 0.0],
        [0.0, SQRT2_HALF, 0.0]
    ],
    [0.0, 0.0, 1.5],
    [0.0, 0.5, 0.0],
    -45.0,
    2.12132
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_H2,
    [
        [0.0, 0.5, 0.0],
        [1.0606601717798212, 0.5, 0.0],
        [1.0606601717798212, 1.0606601717798212, 0.0],
        [0.0, 1.0606601717798212, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    3.93566
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_H3,
    [
        [0.0, 0.5, 0.0],
        [1.25, 0.5, 0.0],
        [1.25, 1.0 / SQRT5, 0.0],
        [0.0, 1.0 / SQRT5, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [0.0, 0.5, 0.0],
    26.56505117707799,
    2.43812
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_H4,
    [
        [-0.25, -0.25, 0.0],
        [2.0, -0.25, 0.0],
        [2.0, SQRT2_HALF, 0.0],
        [-0.25, SQRT2_HALF, 0.0]
    ],
    [0.0, 0.0, 1.0],
    [-0.25, -0.25, 0.0],
    45.0,
    4.41421
);
diagonal_case!(
    BCommonSimpleTest__DiagonalPrisms_H5,
    [
        [-0.25, -0.25, 0.0],
        [2.0, -0.25, 0.0],
        [2.0, SQRT2_HALF, 0.0],
        [-0.25, SQRT2_HALF, 0.0]
    ],
    [0.0, 0.0, 1.5],
    [-0.25, -0.25, 0.0],
    45.0,
    4.41421
);
