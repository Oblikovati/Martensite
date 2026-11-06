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
    offset_shape_type_i_c_i4,
    "tests/offset/shape_type_i_c/I4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 7.0, 0.0, 3.0, 3.0, 0.0, 3.0, 4.0, 0.0, 1.0, 1.0,
                0.0, 1.0, 2.0, 0.0, 3.0, -2.0, 0.0, 3.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 0.6, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 216.363, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_i5,
    "tests/offset/shape_type_i_c/I5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 7.0, 0.0, 3.0, 3.0, 0.0, 3.0, 4.0, 0.0, 1.0, 1.0,
                0.0, 1.0, 2.0, 0.0, 3.0, -2.0, 0.0, 3.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.2, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 394.982, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_o8,
    "tests/offset/shape_type_i_c/O8",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 7.0, 6.0, 0.0, 7.0, 6.0, 0.0, 3.0, 5.0,
                0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 3.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3136.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_o9,
    "tests/offset/shape_type_i_c/O9",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 7.0, 6.0, 0.0, 7.0, 6.0, 0.0, 3.0, 5.0,
                0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1152.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_p1,
    "tests/offset/shape_type_i_c/P1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 3.0, 3.0, 0.0, 3.0, 2.0, 0.0, 1.0, 1.0,
                0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 1.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 4.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1188.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_p2,
    "tests/offset/shape_type_i_c/P2",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, 4.0, 1.0,
                0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 1.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::OnFace(&[("s_4", 2.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 94.2982, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_p3,
    "tests/offset/shape_type_i_c/P3",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 2.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 2.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 1.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::OnFace(&[("s_4", 3.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 60.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v1,
    "tests/offset/shape_type_i_c/V1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                4.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 8.0, 0.0, 10.0, 8.0, 0.0, 4.0, 6.0,
                0.0, 4.0, 6.0, 0.0, 6.0, 4.0, 0.0, 6.0, 4.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 2.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1116.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v2,
    "tests/offset/shape_type_i_c/V2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                4.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 8.0, 0.0, 10.0, 8.0, 0.0, 4.0, 6.0,
                0.0, 4.0, 6.0, 0.0, 6.0, 4.0, 0.0, 6.0, 4.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s1", "f", [0.0, 5.0, 0.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[6.0, 0.0, 4.0, 6.0, 5.0, 4.0, 6.0, 2.5, 5.0, 6.0, 0.0, 4.0]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s2", "f", [2.0, 0.0, 0.0])),
        Step::Bool(Bool::Fuse("s", "s1", "s2")),
        Step::Topo(Topo::Explode("s", "so")),
        Step::Topo(Topo::SharedCopy("s_1", "s")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 2.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1116.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v3,
    "tests/offset/shape_type_i_c/V3",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                4.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 8.0, 0.0, 10.0, 8.0, 0.0, 4.0, 6.0,
                0.0, 4.0, 6.0, 0.0, 6.0, 4.0, 0.0, 6.0, 4.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s1", "f", [0.0, 5.0, 0.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[6.0, 0.0, 4.0, 6.0, 5.0, 4.0, 6.0, 2.5, 3.0, 6.0, 0.0, 4.0]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s2", "f", [2.0, 0.0, 0.0])),
        Step::Bool(Bool::Cut("s", "s1", "s2")),
        Step::Topo(Topo::Explode("s", "so")),
        Step::Topo(Topo::SharedCopy("s_1", "s")),
        Step::Topo(Topo::UnifySameDomain("s", "s")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 2.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1116.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v4,
    "tests/offset/shape_type_i_c/V4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 3.0, 3.0, 0.0, 3.0, 3.0, 0.0, 5.0, 0.0,
                0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_4", 5.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 441.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v5,
    "tests/offset/shape_type_i_c/V5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 4.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.5, 4.0, 0.0, 2.5, 4.0, 10.0, 9.5, 4.0, 10.0, 7.5, 4.0, 0.0, 0.5, 4.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("b2", "f", [0.0, 0.0, 4.0])),
        Step::Bool(Bool::Fuse("s", "b1", "b2")),
        Step::Topo(Topo::UnifySameDomain("s", "s")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 3.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3229.82, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v7,
    "tests/offset/shape_type_i_c/V7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("b1", &[10.0, 6.0, 10.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                2.0, 0.0, 5.0, 2.0, 2.0, 5.0, 2.0, 3.0, 3.0, 2.0, 3.0, 10.0, 2.0, 0.0, 10.0, 2.0,
                0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("b2", "f", [6.0, 0.0, 0.0])),
        Step::Bool(Bool::Cut("s", "b1", "b2")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 2.5, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2460.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v8,
    "tests/offset/shape_type_i_c/V8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("b1", &[10.0, 6.0, 10.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                2.0, 0.0, 5.0, 2.0, 2.0, 5.0, 2.0, 3.0, 3.0, 2.0, 3.0, 10.0, 2.0, 0.0, 10.0, 2.0,
                0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("b2", "f", [6.0, 0.0, 0.0])),
        Step::Bool(Bool::Cut("s", "b1", "b2")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 4.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4536.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_v9,
    "tests/offset/shape_type_i_c/V9",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("b1", &[10.0, 6.0, 10.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                2.0, 0.0, 5.0, 2.0, 2.0, 5.0, 2.0, 2.5, 3.0, 2.0, 3.0, 5.0, 2.0, 3.0, 10.0, 2.0,
                0.0, 10.0, 2.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("b2", "f", [6.0, 0.0, 0.0])),
        Step::Bool(Bool::Cut("s", "b1", "b2")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', true)),
        Step::Offset(Offset::Load("s", 2.5, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 1140.0, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 2460.0, 0.01, None))
        ]),
        Step::Topo(Topo::UnifySameDomain("result_unif", "result")),
        Step::Check(Check::Counts("result_unif", &[("FACE", 10), ("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_w1,
    "tests/offset/shape_type_i_c/W1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Box("b1", &[10.0, 6.0, 10.0])),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                2.0, 0.0, 5.0, 2.0, 2.0, 5.0, 2.0, 2.5, 3.0, 2.0, 3.0, 5.0, 2.0, 3.0, 10.0, 2.0,
                0.0, 10.0, 2.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("b2", "f", [6.0, 0.0, 0.0])),
        Step::Bool(Bool::Cut("s", "b1", "b2")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 4.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4536.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_w4,
    "tests/offset/shape_type_i_c/W4",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 11.0, 0.0, 0.0, 11.0, 0.0, 4.0, 10.0, 0.0, 4.0, 10.0, 0.0, 3.0, 9.0,
                0.0, 2.0, 8.0, 0.0, 2.0, 7.0, 0.0, 3.0, 7.0, 0.0, 4.0, 4.0, 0.0, 4.0, 4.0, 0.0,
                3.0, 3.0, 0.0, 2.0, 2.0, 0.0, 2.0, 1.0, 0.0, 3.0, 1.0, 0.0, 4.0, 0.0, 0.0, 4.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.2, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 618.17, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_w5,
    "tests/offset/shape_type_i_c/W5",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 11.0, 0.0, 0.0, 11.0, 0.0, 4.0, 10.0, 0.0, 4.0, 10.0, 0.0, 3.0, 9.0,
                0.0, 2.0, 8.0, 0.0, 2.0, 7.0, 0.0, 3.0, 7.0, 0.0, 4.0, 4.0, 0.0, 4.0, 4.0, 0.0,
                3.0, 3.0, 0.0, 2.0, 2.0, 0.0, 2.0, 1.0, 0.0, 3.0, 1.0, 0.0, 4.0, 0.0, 0.0, 4.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.5, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 784.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_w6,
    "tests/offset/shape_type_i_c/W6",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 11.0, 0.0, 0.0, 11.0, 0.0, 4.0, 10.0, 0.0, 4.0, 10.0, 0.0, 3.0, 9.0,
                0.0, 2.0, 8.0, 0.0, 2.0, 7.0, 0.0, 3.0, 7.0, 0.0, 4.0, 4.0, 0.0, 4.0, 4.0, 0.0,
                3.0, 3.0, 0.0, 2.0, 2.0, 0.0, 2.0, 1.0, 0.0, 3.0, 1.0, 0.0, 4.0, 0.0, 0.0, 4.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.8, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 954.256, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_w7,
    "tests/offset/shape_type_i_c/W7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 3.0, 3.0, 0.0, 3.0, 2.0, 0.0, 1.0, 1.0,
                0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 1.0, 0.0])),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 3.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 630.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_xt1,
    "tests/offset/shape_type_i_c/XT1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO CR27414 ALL: Error : The area of result shape is"),
        Step::Puts("TODO CR27414 ALL: Error : The volume of result shape is"),
        Step::Puts("TODO CR27414 ALL: Error :  is WRONG because number of"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 2.0, 0.0, 10.0, 2.0, 0.0, 2.0, 3.0, 0.0, 6.0, 5.0,
                0.0, 6.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', true)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::OnFace(&[("s_4", 6.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 410.0, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 504.0, 0.01, None))
        ]),
        Step::Topo(Topo::UnifySameDomain("result_unif", "result")),
        Step::Check(Check::Counts(
            "result_unif",
            &[
                ("VERTEX", 12),
                ("EDGE", 18),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("result_unif"),
    ]
);

offset_case!(
    offset_shape_type_i_c_xt2,
    "tests/offset/shape_type_i_c/XT2",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 2.0, 0.0, 10.0, 2.0, 0.0, 2.0, 3.0, 0.0, 6.0, 5.0,
                0.0, 6.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', true)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::OnFace(&[("s_4", 4.5)])),
        Step::Offset(Offset::Perform("result")),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 410.0, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 504.0, 0.01, None))
        ]),
        Step::Topo(Topo::UnifySameDomain("result_unif", "result")),
        Step::Check(Check::Counts(
            "result_unif",
            &[
                ("VERTEX", 12),
                ("EDGE", 18),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("result_unif"),
    ]
);

offset_case!(
    offset_shape_type_i_c_y1,
    "tests/offset/shape_type_i_c/Y1",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 7.0, 6.0, 0.0, 7.0, 6.0, 0.0, 3.0, 5.0,
                0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1152.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y2,
    "tests/offset/shape_type_i_c/Y2",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 7.0, 6.0, 0.0, 7.0, 6.0, 0.0, 3.0, 5.0,
                0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 3.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3136.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y3,
    "tests/offset/shape_type_i_c/Y3",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 2.0, 2.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_3", 5.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 252.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y4,
    "tests/offset/shape_type_i_c/Y4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 2.0, 2.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_2", 5.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 269.397, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y5,
    "tests/offset/shape_type_i_c/Y5",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, 3.0, 1.0,
                0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_4", 3.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 233.186, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y6,
    "tests/offset/shape_type_i_c/Y6",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, 3.0, 1.0,
                0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_4", 3.5)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 243.731, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y7,
    "tests/offset/shape_type_i_c/Y7",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 1.0, 1.0, 0.0, 1.0, 5.0, 0.0, 5.0, 0.0,
                0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_4", 4.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 455.0, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);

offset_case!(
    offset_shape_type_i_c_y8,
    "tests/offset/shape_type_i_c/Y8",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 1.0, 1.0, 0.0, 1.0, 5.0, 0.0, 5.0, 0.0,
                0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 5.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::View(ViewStep::Donly(&["s"])),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_4", 6.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 587.279, 0.01, None)),
        Step::Check(Check::Counts("result", &[("SHELL", 1)])),
    ]
);
