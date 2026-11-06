//! Regressions of self-contained `bcommon_simple` prism regressions D9 through F6.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::{Point3, Vector3};

fn rectangular_prism(points: [[f64; 3]; 4], direction: [f64; 3]) -> Shape {
    let points = points.map(|point| Point3::new(point[0], point[1], point[2]));
    let wire = CreatePolygonWire(&points, true);
    let face = CreateFaceFromWire(&wire);
    CreatePrism(
        &face,
        Vector3::new(direction[0], direction[1], direction[2]),
    )
}

fn common_prism_case(
    first_points: [[f64; 3]; 4],
    first_direction: [f64; 3],
    second_points: [[f64; 3]; 4],
    second_direction: [f64; 3],
    expected_area: f64,
) {
    let first = rectangular_prism(first_points, first_direction);
    let second = rectangular_prism(second_points, second_direction);
    let result = PerformCommon(&first, &second);
    ValidateResult(&result, expected_area, -1.0, false);
}

macro_rules! common_prism_case {
    ($name:ident, $first_points:expr, $first_direction:expr, $second_points:expr, $second_direction:expr, $area:expr) => {
        #[test]
        fn $name() {
            common_prism_case(
                $first_points,
                $first_direction,
                $second_points,
                $second_direction,
                $area,
            );
        }
    };
}

const UNIT_SQUARE: [[f64; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]];
const LARGE_SQUARE: [[f64; 3]; 4] = [[0., 0., 0.], [2., 0., 0.], [2., 2., 0.], [0., 2., 0.]];
const UNIT_SQUARE_TOP: [[f64; 3]; 4] = [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]];

// Test bcommon_simple/D9: identically bounded boxes extruded toward each other; checkprops result -s 6
common_prism_case!(
    BCommonSimpleTest__OpposedUnitPrisms_D9,
    UNIT_SQUARE,
    [0., 0., 1.],
    UNIT_SQUARE_TOP,
    [0., 0., -1.],
    6.0
);

// Test bcommon_simple/E1: nested rectangular prisms; checkprops result -s 4
common_prism_case!(
    BCommonSimpleTest__NestedPrisms_E1,
    UNIT_SQUARE,
    [0., 0., 1.],
    [[0., 0., 0.], [1., 0., 0.], [1., 0.5, 0.], [0., 0.5, 0.]],
    [0., 0., 1.],
    4.0
);

// Test bcommon_simple/E2: nested rectangular prisms with the inner prism extruded downward; checkprops result -s 4
common_prism_case!(
    BCommonSimpleTest__OpposedNestedPrisms_E2,
    UNIT_SQUARE,
    [0., 0., 1.],
    [[0., 0., 1.], [1., 0., 1.], [1., 0.5, 1.], [0., 0.5, 1.]],
    [0., 0., -1.],
    4.0
);

// Test bcommon_simple/E3: unit prism contained in a two-unit prism; checkprops result -s 6
common_prism_case!(
    BCommonSimpleTest__ContainedPrisms_E3,
    LARGE_SQUARE,
    [0., 0., 2.],
    UNIT_SQUARE,
    [0., 0., 1.],
    6.0
);

// Test bcommon_simple/E4: contained prisms with opposite extrusion; checkprops result -s 6
common_prism_case!(
    BCommonSimpleTest__OpposedContainedPrisms_E4,
    LARGE_SQUARE,
    [0., 0., 2.],
    UNIT_SQUARE_TOP,
    [0., 0., -1.],
    6.0
);

// Test bcommon_simple/E5: reversed inner-wire construction; checkprops result -s 6
common_prism_case!(
    BCommonSimpleTest__ReversedWirePrisms_E5,
    LARGE_SQUARE,
    [0., 0., 2.],
    [[0., 0., 0.], [0., 1., 0.], [1., 1., 0.], [1., 0., 0.]],
    [0., 0., 1.],
    6.0
);

const UPPER_PRISM: [[f64; 3]; 4] = [[2., 2., 0.], [4., 2., 0.], [4., 4., 0.], [2., 4., 0.]];
const LOWER_PRISM: [[f64; 3]; 4] = [[2., 0., 0.], [3., 0., 0.], [3., 3., 0.], [2., 3., 0.]];
const LOWER_PRISM_TOP: [[f64; 3]; 4] = [[2., 0., 4.], [3., 0., 4.], [3., 3., 4.], [2., 3., 4.]];

// Test bcommon_simple/E6: overlapping vertical prisms; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__OverlappingVerticalPrisms_E6,
    UPPER_PRISM,
    [0., 0., 4.],
    LOWER_PRISM,
    [0., 0., 4.],
    18.0
);

// Test bcommon_simple/E7: lower prism constructed at the top and extruded downward; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__OpposedVerticalPrisms_E7,
    UPPER_PRISM,
    [0., 0., 4.],
    LOWER_PRISM_TOP,
    [0., 0., -4.],
    18.0
);

// Test bcommon_simple/E8: reversed lower-wire construction; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__ReversedVerticalPrisms_E8,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 3., 0.], [2., 0., 0.], [3., 0., 0.], [3., 3., 0.]],
    [0., 0., 4.],
    18.0
);

// Test bcommon_simple/E9: prism extruded from the left; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__LeftPrism_E9,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 0., 0.], [2., 3., 0.], [2., 3., 4.], [2., 0., 4.]],
    [1., 0., 0.],
    18.0
);

// Test bcommon_simple/F1: prism extruded from the front; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__FrontPrism_F1,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 1., 0.], [3., 1., 0.], [3., 1., 4.], [2., 1., 4.]],
    [0., 2., 0.],
    18.0
);

// Test bcommon_simple/F2: prism extruded from the back; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__BackPrism_F2,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 3., 0.], [3., 3., 0.], [3., 3., 4.], [2., 3., 4.]],
    [0., -2., 0.],
    18.0
);

// Test bcommon_simple/F3: prism extruded from the right; checkprops result -s 18
common_prism_case!(
    BCommonSimpleTest__RightPrism_F3,
    UPPER_PRISM,
    [0., 0., 4.],
    [[3., 1., 0.], [3., 3., 0.], [3., 3., 4.], [3., 1., 4.]],
    [-1., 0., 0.],
    18.0
);

// Test bcommon_simple/F4: narrow vertical prism; checkprops result -s 16
common_prism_case!(
    BCommonSimpleTest__NarrowVerticalPrism_F4,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 2., 0.], [3., 2., 0.], [3., 4., 0.], [2., 4., 0.]],
    [0., 0., 2.],
    16.0
);

// Test bcommon_simple/F5: narrow front prism; checkprops result -s 16
common_prism_case!(
    BCommonSimpleTest__NarrowFrontPrism_F5,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 2., 0.], [3., 2., 0.], [3., 2., 2.], [2., 2., 2.]],
    [0., 2., 0.],
    16.0
);

// Test bcommon_simple/F6: oblong back prism; checkprops result -s 10
common_prism_case!(
    BCommonSimpleTest__OblongBackPrism_F6,
    UPPER_PRISM,
    [0., 0., 4.],
    [[2., 3., 0.], [3., 3., 0.], [3., 3., 2.], [2., 3., 2.]],
    [0., -4., 0.],
    10.0
);
