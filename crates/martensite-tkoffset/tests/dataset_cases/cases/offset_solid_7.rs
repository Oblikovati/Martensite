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
    offset_shape_type_i_e2,
    "tests/offset/shape_type_i/E2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 14212.2, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e3,
    "tests/offset/shape_type_i/E3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 17157.3, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e4,
    "tests/offset/shape_type_i/E4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 11494.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e6,
    "tests/offset/shape_type_i/E6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1728.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e7,
    "tests/offset/shape_type_i/E7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 512.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e8,
    "tests/offset/shape_type_i/E8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 746.105, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_e9,
    "tests/offset/shape_type_i/E9",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 103.814, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f1,
    "tests/offset/shape_type_i/F1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 494.623, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f2,
    "tests/offset/shape_type_i/F2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 45.204, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f3,
    "tests/offset/shape_type_i/F3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 230.266, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f4,
    "tests/offset/shape_type_i/F4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1070.66, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f5,
    "tests/offset/shape_type_i/F5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 191.044, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f6,
    "tests/offset/shape_type_i/F6",
    Shape,
    500,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: result is not a topological shape"),
        Step::Puts("TODO 23068 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23068 ALL: TEST INCOMPLETE"),
        Step::Puts("CPU and elapsed time limit set to 500 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f7,
    "tests/offset/shape_type_i/F7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3238.1, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f8,
    "tests/offset/shape_type_i/F8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1130.69, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_f9,
    "tests/offset/shape_type_i/F9",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 8951.52, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_g1,
    "tests/offset/shape_type_i/G1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 270.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3739.13, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_g2,
    "tests/offset/shape_type_i/G2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1163.29, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_g3,
    "tests/offset/shape_type_i/G3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 279.306, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_g4,
    "tests/offset/shape_type_i/G4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::Bezier(
            "w1",
            &[
                5.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 5.0, 0.0, 25.0, 10.0, 0.0, 10.0, 20.0,
                0.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 20.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [-2.0, 7.0, 10.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4278.74, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_g5,
    "tests/offset/shape_type_i/G5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.2566370614359172]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1836.88, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_g6,
    "tests/offset/shape_type_i/G6",
    Shape,
    1000,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.2566370614359172]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 196.68, 0.01, None)),
    ]
);
