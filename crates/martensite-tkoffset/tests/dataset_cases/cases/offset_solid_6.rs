//! Script case tables.
#[allow(unused_imports)]
use crate::conditions::{Cond, Value};
#[allow(unused_imports)]
use crate::extra_steps::*;
#[allow(unused_imports)]
use crate::groups::{End, WireEnd};
#[allow(unused_imports)]
use crate::steps::*;
#[allow(unused_imports)]
use crate::support::{properties::Prop, session::ViewStep};

offset_case!(
    offset_shape_type_i_a6,
    "tests/offset/shape_type_i/A6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 6.32115, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_a7,
    "tests/offset/shape_type_i/A7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape "),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_a8,
    "tests/offset/shape_type_i/A8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 47.0723, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_a9,
    "tests/offset/shape_type_i/A9",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            296865000000000.0,
            "2.96865e+014",
            0.01,
            None
        )),
    ]
);

offset_case!(
    offset_shape_type_i_b1,
    "tests/offset/shape_type_i/B1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2261.97, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_b2,
    "tests/offset/shape_type_i/B2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape "),
        Step::Puts("TODO 25406 ALL: Error: bsection of the result and s is not equal to zero"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_b3,
    "tests/offset/shape_type_i/B3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape "),
        Step::Puts("TODO 25406 ALL: Error: bsection of the result and s is not equal to zero"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_b4,
    "tests/offset/shape_type_i/B4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            177778000000000.0,
            "1.77778e+014",
            0.01,
            None
        )),
    ]
);

offset_case!(
    offset_shape_type_i_b5,
    "tests/offset/shape_type_i/B5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2286.76, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_b6,
    "tests/offset/shape_type_i/B6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error: bsection of the result and s is not equal to zero."),
        Step::Prim(Prim::Sphere("s", &[15.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            177778000000000.0,
            "1.77778e+014",
            0.01,
            None
        )),
    ]
);

offset_case!(
    offset_shape_type_i_b7,
    "tests/offset/shape_type_i/B7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape"),
        Step::Puts("TODO 25406 ALL: Error: bsection of the result and s is not equal to zero"),
        Step::Prim(Prim::Sphere("s", &[15.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_b8,
    "tests/offset/shape_type_i/B8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: result is not a topological shape"),
        Step::Puts("TODO 23068 ALL: TEST INCOMPLETE"),
        Step::Puts("TODO 23068 ALL: Error: The command cannot be built"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_b9,
    "tests/offset/shape_type_i/B9",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 5237.16, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c1,
    "tests/offset/shape_type_i/C1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3779.35, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c2,
    "tests/offset/shape_type_i/C2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1478.56, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c3,
    "tests/offset/shape_type_i/C3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape "),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c4,
    "tests/offset/shape_type_i/C4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4636.84, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c5,
    "tests/offset/shape_type_i/C5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Puts("TODO 23068 ALL: Error: bsection of the result and s is not equal to zero."),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol("s", "w", [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
    ]
);

offset_case!(
    offset_shape_type_i_c6,
    "tests/offset/shape_type_i/C6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol("s", "w", [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2778.05, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c7,
    "tests/offset/shape_type_i/C7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 25406 ALL: Error: bsection of the result and s is not equal to zero"),
        Step::Puts("TODO 23068 ALL:Error : The volume"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 270.0]
        )),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c8,
    "tests/offset/shape_type_i/C8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape is"),
        Step::Puts("TODO 23068 ALL: Error: bsection of the result and s is not equal to zero"),
        Step::Puts("TODO 23068 ALL: Faulty shapes"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 270.0]
        )),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_c9,
    "tests/offset/shape_type_i/C9",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                4.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 20.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2838.99, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d1,
    "tests/offset/shape_type_i/D1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                4.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 20.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1061.03, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d2,
    "tests/offset/shape_type_i/D2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 690.208, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d3,
    "tests/offset/shape_type_i/D3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 107.757, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d4,
    "tests/offset/shape_type_i/D4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 16962.8, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d5,
    "tests/offset/shape_type_i/D5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 11324.9, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d6,
    "tests/offset/shape_type_i/D6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 22864.6, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_d7,
    "tests/offset/shape_type_i/D7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 13139.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e1,
    "tests/offset/shape_type_i/E1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 6316.55, 0.01, None)),
    ]
);
