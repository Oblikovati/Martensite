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
    offset_faces_type_i_l9,
    "tests/offset/faces_type_i/L9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1120.05, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m1,
    "tests/offset/faces_type_i/M1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 910.985, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m2,
    "tests/offset/faces_type_i/M2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 765.617, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m3,
    "tests/offset/faces_type_i/M3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 678.359, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m4,
    "tests/offset/faces_type_i/M4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 589.391, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m5,
    "tests/offset/faces_type_i/M5",
    Shape,
    1000,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 407.179, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m6,
    "tests/offset/faces_type_i/M6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 785.398, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m7,
    "tests/offset/faces_type_i/M7",
    Shape,
    1000,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 259.68, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m8,
    "tests/offset/faces_type_i/M8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 785.398, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_m9,
    "tests/offset/faces_type_i/M9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 445.962, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_n1,
    "tests/offset/faces_type_i/N1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO CR23530 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::More(More::Surface(Surface::Bezier(
            "c",
            &[
                3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 2.0, 14.0, 3.0, 10.0, -5.0, -3.0, 13.0,
                3.0, 0.0, 12.0, 10.0, 5.0
            ]
        ))),
        Step::More(More::Face("c", "c", None)),
        Step::Sweep(Sweep::Prism("s", "c", [0.0, 0.0, 20.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_5", "s_6"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3340.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_n2,
    "tests/offset/faces_type_i/N2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::More(More::Surface(Surface::Bezier(
            "c",
            &[
                3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 2.0, 14.0, 3.0, 10.0, -5.0, -3.0, 13.0,
                3.0, 0.0, 12.0, 10.0, 5.0
            ]
        ))),
        Step::More(More::Face("c", "c", None)),
        Step::Sweep(Sweep::Prism("s", "c", [0.0, 0.0, 20.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_5", "s_6"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 951.165, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_a1,
    "tests/offset/shape/A1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE900_hkg60iob.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -5.0,
            tolerance: None,
            complete: false,
            faces: &["s_1"]
        })),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            28031200.0,
            "2.80312e+07",
            0.01,
            None
        )),
    ]
);

offset_case!(
    offset_shape_a4,
    "tests/offset/shape/A4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[100.0, 100.0, 100.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -10.0,
            tolerance: None,
            complete: false,
            faces: &["s_1"]
        })),
        Step::Check(Check::Props(Prop::Volume, "result", 424000.0, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a1,
    "tests/offset/shape_type_a/A1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_cc2.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.01, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0709583, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a2,
    "tests/offset/shape_type_a/A2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb60.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.37, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 30.1125, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a3,
    "tests/offset/shape_type_a/A3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb2.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1.79372, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a4,
    "tests/offset/shape_type_a/A4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2462.04, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a6,
    "tests/offset/shape_type_a/A6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb14.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.285805, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a7,
    "tests/offset/shape_type_a/A7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb15.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 15.2493, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a8,
    "tests/offset/shape_type_a/A8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_vevopar.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.650881, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_a9,
    "tests/offset/shape_type_a/A9",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb14.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1.65744, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_b1,
    "tests/offset/shape_type_a/B1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb15.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 26.4725, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_b2,
    "tests/offset/shape_type_a/B2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb9.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.8, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 100.616, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_b4,
    "tests/offset/shape_type_a/B4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_vevopar.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1.89228, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_b5,
    "tests/offset/shape_type_a/B5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb2.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4.55884, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_a_b6,
    "tests/offset/shape_type_a/B6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb9.rle", "s")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", 0.1, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 36.8614, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_a3,
    "tests/offset/shape_type_i/A3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 5204.02, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_a4,
    "tests/offset/shape_type_i/A4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2462.04, 0.01, None)),
    ]
);

offset_case!(
    offset_shape_type_i_a5,
    "tests/offset/shape_type_i/A5",
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
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            3.57246e+19,
            "3.57246e+019",
            0.01,
            None
        )),
    ]
);
