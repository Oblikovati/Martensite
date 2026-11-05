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
    offset_faces_type_i_f3,
    "tests/offset/faces_type_i/F3",
    Shape,
    400,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("CPU and elapsed time limit set to 400 seconds"),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 13155.2, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f4,
    "tests/offset/faces_type_i/F4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 270.0]
        )),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 13155.2, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f5,
    "tests/offset/faces_type_i/F5",
    Shape,
    600,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 13155.2, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f6,
    "tests/offset/faces_type_i/F6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 925.617, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f7,
    "tests/offset/faces_type_i/F7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 703.738, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f8,
    "tests/offset/faces_type_i/F8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_1", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 811.465, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f9,
    "tests/offset/faces_type_i/F9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_1", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 653.428, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g1,
    "tests/offset/faces_type_i/G1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 251.643, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g2,
    "tests/offset/faces_type_i/G2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 163.155, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g3,
    "tests/offset/faces_type_i/G3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2775.18, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g4,
    "tests/offset/faces_type_i/G4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2505.07, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g5,
    "tests/offset/faces_type_i/G5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
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
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3048.04, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g6,
    "tests/offset/faces_type_i/G6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 584.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g7,
    "tests/offset/faces_type_i/G7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 424.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g8,
    "tests/offset/faces_type_i/G8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3", "s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 452.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_g9,
    "tests/offset/faces_type_i/G9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3", "s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 352.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h1,
    "tests/offset/faces_type_i/H1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("s", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 360.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h2,
    "tests/offset/faces_type_i/H2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 292.953, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h3,
    "tests/offset/faces_type_i/H3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 184.115, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h4,
    "tests/offset/faces_type_i/H4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 231.759, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h5,
    "tests/offset/faces_type_i/H5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 160.829, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h6,
    "tests/offset/faces_type_i/H6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_1"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 175.914, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h7,
    "tests/offset/faces_type_i/H7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Wedge("s", &[4.0, 10.0, 10.0, 2.0, 2.0, 7.0, 7.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_1"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 139.765, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h8,
    "tests/offset/faces_type_i/H8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 214.608, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_h9,
    "tests/offset/faces_type_i/H9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 127.482, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i1,
    "tests/offset/faces_type_i/I1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_5", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 142.942, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i2,
    "tests/offset/faces_type_i/I2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_5", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 95.8186, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i3,
    "tests/offset/faces_type_i/I3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3", "s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 215.836, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i4,
    "tests/offset/faces_type_i/I4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3", "s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 139.845, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i5,
    "tests/offset/faces_type_i/I5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 589.049, 0.01, None)),
    ]
);
