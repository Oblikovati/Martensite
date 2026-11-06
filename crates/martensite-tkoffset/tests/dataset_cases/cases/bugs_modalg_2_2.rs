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
    bugs_modalg_2_bug427_4,
    "tests/bugs/modalg_2/bug427_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 427 "),
        Step::Puts("(case 4)"),
        Step::Puts("========================"),
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
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 1306.3, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug427_5,
    "tests/bugs/modalg_2/bug427_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 427 "),
        Step::Puts("(case 5)"),
        Step::Puts("========================"),
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
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 1297.39, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug427_6,
    "tests/bugs/modalg_2/bug427_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 427 "),
        Step::Puts("(case 6)"),
        Step::Puts("========================"),
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
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 808.448, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_1,
    "tests/bugs/modalg_2/bug4717_1",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of"),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("s", "b")),
        Step::Bool(Bool::TwoStep("result", BopKind::Fuse)),
        Step::Check(Check::Props(Prop::Area, "result", 680565.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 11),
                ("EDGE", 16),
                ("WIRE", 8),
                ("FACE", 7),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 45)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_2,
    "tests/bugs/modalg_2/bug4717_2",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of"),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("b", "s")),
        Step::Bool(Bool::TwoStep("result", BopKind::Fuse)),
        Step::Check(Check::Props(Prop::Area, "result", 680565.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 11),
                ("EDGE", 16),
                ("WIRE", 8),
                ("FACE", 7),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 45)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_3,
    "tests/bugs/modalg_2/bug4717_3",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of"),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("s", "b")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 181937.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 4),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 14)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_4,
    "tests/bugs/modalg_2/bug4717_4",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of"),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("b", "s")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 680565.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 11),
                ("EDGE", 16),
                ("WIRE", 8),
                ("FACE", 7),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 45)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_5,
    "tests/bugs/modalg_2/bug4717_5",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of "),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("s", "b")),
        Step::Bool(Bool::TwoStep("result", BopKind::Tuc)),
        Step::Check(Check::Props(Prop::Area, "result", 680565.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 11),
                ("EDGE", 16),
                ("WIRE", 8),
                ("FACE", 7),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 45)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_6,
    "tests/bugs/modalg_2/bug4717_6",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of "),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("b", "s")),
        Step::Bool(Bool::TwoStep("result", BopKind::Tuc)),
        Step::Check(Check::Props(Prop::Area, "result", 181937.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 4),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 14)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_7,
    "tests/bugs/modalg_2/bug4717_7",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of "),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("s", "b")),
        Step::Bool(Bool::TwoStep("result", BopKind::Common)),
        Step::Check(Check::Props(Prop::Area, "result", 181937.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 4),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 14)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug4717_8,
    "tests/bugs/modalg_2/bug4717_8",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of "),
        Step::Puts("============"),
        Step::Puts("4717"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::Prepare("b", "s")),
        Step::Bool(Bool::TwoStep("result", BopKind::Common)),
        Step::Check(Check::Props(Prop::Area, "result", 181937.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 4),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 14)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug500,
    "tests/bugs/modalg_2/bug500",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========="),
        Step::Puts(" 500 "),
        Step::Puts("========="),
        Step::Topo(Topo::Restore("brep/OCC500a.brep", "a_1")),
        Step::Check(Check::Shape("a_1")),
        Step::Topo(Topo::Restore("brep/OCC500b.brep", "a_2")),
        Step::Check(Check::Shape("a_2")),
        Step::If(
            Cond::Failed(&Step::Sweep(Sweep::Pipe("result", "a_1", "a_2", 0, false))),
            &[Step::Puts("Faulty 500 : function PIPE works wrongly ")],
            &[Step::Puts("500 : function PIPE works without hangs up ")]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 1246.13, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_1,
    "tests/bugs/modalg_2/bug5805_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Puts("demonstration of pipes and hollowing for GTISoft"),
        Step::Puts(
            "Scenario 2A: Hollowed elliptical cylinder - ThroughSection and MakeThickSolid (work-around: ellipse is split into 2)"
        ),
        Step::Curve(Curve::Ellipse(
            "e",
            &[10.0, 60.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::More(More::TrimCurve("e1", "e", [0.0, 3.141592653589793])),
        Step::More(More::TrimCurve(
            "e2",
            "e",
            [3.141592653589793, 6.283185307179586]
        )),
        Step::Profile(Profile::Edge("ee1", "e1", None)),
        Step::Profile(Profile::Edge("ee2", "e2", None)),
        Step::Profile(Profile::Wire("w1", &["ee1", "ee2"])),
        Step::Topo(Topo::SharedCopy("w1", "w2")),
        Step::Place(Place::Translate(&["w2"], [-20.0, 0.0, 0.0])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "rthru2",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["w1", "w2"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Topo(Topo::Explode("rthru2", "f")),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Shape(OffsetShape {
                result: "result",
                shape: "rthru2",
                value: -0.5,
                tolerance: None,
                complete: false,
                faces: &["rthru2_3", "rthru2_4"]
            }))),
            &[Step::Puts("Faulty 5805 : offsetshape is wrong")],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", None, Some(1.0))),
        Step::Check(Check::Props(Prop::Area, "result", 981.941, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 8), ("FACE", 6), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_10,
    "tests/bugs/modalg_2/bug5805_10",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::Ellipse(
            "ell",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Topo(Topo::SharedCopy("ee1", "ee2")),
        Step::Place(Place::Translate(&["ee2"], [-20.0, 0.0, 0.0])),
        Step::Profile(Profile::Wire("BottomCover", &["ee2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_2"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 1065.08, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 5), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_11,
    "tests/bugs/modalg_2/bug5805_11",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Curve(Curve::Ellipse(
            "ell",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Topo(Topo::SharedCopy("ee1", "ee2")),
        Step::Place(Place::Translate(&["ee2"], [-20.0, 0.0, 0.0])),
        Step::Profile(Profile::Wire("BottomCover", &["ee2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_3"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 1065.08, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 5), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_12,
    "tests/bugs/modalg_2/bug5805_12",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::Ellipse(
            "ell",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Topo(Topo::SharedCopy("ee1", "ee2")),
        Step::Place(Place::Translate(&["ee2"], [-20.0, 0.0, 0.0])),
        Step::Profile(Profile::Wire("BottomCover", &["ee2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_2", "s_3"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 1006.59, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 4), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_13,
    "tests/bugs/modalg_2/bug5805_13",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "UpCover",
            &[
                0.0, 0.0, 5.0, 5.0, 0.0, 5.0, 5.0, 5.0, 5.0, 0.0, 5.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                0.0, 0.0, -5.0, 5.0, 0.0, -5.0, 5.0, 5.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, -5.0
            ]
        )),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 176.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 8),
                ("EDGE", 12),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 34)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_14,
    "tests/bugs/modalg_2/bug5805_14",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "UpCover",
            &[
                0.0, 0.0, 5.0, 5.0, 0.0, 5.0, 5.0, 5.0, 5.0, 0.0, 5.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                0.0, 0.0, -5.0, 5.0, 0.0, -5.0, 5.0, 5.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, -5.0
            ]
        )),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_5"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 402.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 16),
                ("EDGE", 24),
                ("WIRE", 12),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 65)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_15,
    "tests/bugs/modalg_2/bug5805_15",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "UpCover",
            &[
                0.0, 0.0, 5.0, 5.0, 0.0, 5.0, 5.0, 5.0, 5.0, 0.0, 5.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                0.0, 0.0, -5.0, 5.0, 0.0, -5.0, 5.0, 5.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, -5.0
            ]
        )),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_6"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 402.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 16),
                ("EDGE", 24),
                ("WIRE", 12),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 65)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_16,
    "tests/bugs/modalg_2/bug5805_16",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "UpCover",
            &[
                0.0, 0.0, 5.0, 5.0, 0.0, 5.0, 5.0, 5.0, 5.0, 0.0, 5.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                0.0, 0.0, -5.0, 5.0, 0.0, -5.0, 5.0, 5.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, -5.0
            ]
        )),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_5", "s_6"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 378.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 16),
                ("EDGE", 24),
                ("WIRE", 12),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 64)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_17,
    "tests/bugs/modalg_2/bug5805_17",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25925 ALL: Faulty 5805 : result is not Closed shape"),
        Step::Puts("TODO 25925 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 25925 ALL: TEST INCOMPLETE"),
        Step::Puts(
            "TODO 25925 ALL: ScriptStep Exception: Error : command \\\"nbshapes result\\\" gives an empty result"
        ),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 5.0, 5.0, -5.0, 5.0, -5.0, -5.0, 5.0, -5.0, 5.0, 5.0, 5.0, 5.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, -5.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.001, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 495.635, 0.01, None)),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 13)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_18,
    "tests/bugs/modalg_2/bug5805_18",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25925 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 5.0, 5.0, -5.0, 5.0, -5.0, -5.0, 5.0, -5.0, 5.0, 5.0, 5.0, 5.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, -5.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.001, &["s_2"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 358.795, 0.01, None)),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 3), ("FACE", 3), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_19,
    "tests/bugs/modalg_2/bug5805_19",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25925 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 5.0, 5.0, -5.0, 5.0, -5.0, -5.0, 5.0, -5.0, 5.0, 5.0, 5.0, 5.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, -5.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.001, &["s_3"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 495.635, 0.01, None)),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 1),
                ("EDGE", 2),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 11)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_2,
    "tests/bugs/modalg_2/bug5805_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Puts("demonstration of pipes and hollowing for GTISoft"),
        Step::Puts("Scenario 2A: Hollowed elliptical cylinder - ThroughSection and MakeThickSolid"),
        Step::Curve(Curve::Ellipse(
            "e",
            &[10.0, 60.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee", "e", None)),
        Step::Profile(Profile::Wire("w1", &["ee"])),
        Step::Topo(Topo::SharedCopy("w1", "w2")),
        Step::Place(Place::Translate(&["w2"], [-20.0, 0.0, 0.0])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "rthru2",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["w1", "w2"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Topo(Topo::Explode("rthru2", "f")),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Shape(OffsetShape {
                result: "result",
                shape: "rthru2",
                value: -0.5,
                tolerance: None,
                complete: false,
                faces: &["rthru2_2", "rthru2_3"]
            }))),
            &[Step::Puts("Faulty 5805 : offsetshape is wrong")],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", None, Some(1.0))),
        Step::Check(Check::Props(Prop::Area, "result", 1006.59, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 4), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_20,
    "tests/bugs/modalg_2/bug5805_20",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25925 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 5.0, 5.0, -5.0, 5.0, -5.0, -5.0, 5.0, -5.0, 5.0, 5.0, 5.0, 5.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, -5.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.001, &["s_2", "s_3"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 495.635, 0.01, None)),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 1),
                ("EDGE", 2),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 11)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_21,
    "tests/bugs/modalg_2/bug5805_21",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::PeriodicBSpline(
            "pbc1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.1, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 304.115, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 13)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_22,
    "tests/bugs/modalg_2/bug5805_22",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::PeriodicBSpline(
            "pbc1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.1, &["s_3"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 552.913, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 5), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_23,
    "tests/bugs/modalg_2/bug5805_23",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::PeriodicBSpline(
            "pbc1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.1, &["s_2"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 552.913, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 5), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_24,
    "tests/bugs/modalg_2/bug5805_24",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Curve(Curve::PeriodicBSpline(
            "pbc1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbc1", None)),
        Step::Profile(Profile::Wire("UpCover", &["e1"])),
        Step::Topo(Topo::SharedCopy("e1", "e2")),
        Step::Place(Place::Translate(&["e2"], [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Wire("BottomCover", &["e2"])),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.1, &["s_3", "s_2"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 540.231, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::TypedCounts(
            "result",
            &[("WIRE", 6), ("FACE", 4), ("SHELL", 1), ("SOLID", 1)]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_25,
    "tests/bugs/modalg_2/bug5805_25",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("============"),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Profile(Profile::Polyline(
            "UpCover",
            &[
                0.0, 0.0, 10.0, 5.0, 5.0, 10.0, 0.0, 10.0, 10.0, 0.0, 0.0, 10.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                -5.0, -5.0, -10.0, 10.0, 10.0, -10.0, -5.0, 20.0, -10.0, -5.0, -5.0, -10.0
            ]
        )),
        Step::If(
            Cond::Failed(&Step::Loft(Loft {
                result: "s",
                solid: true,
                ruled: true,
                check: true,
                mutable_input: true,
                sections: &["UpCover", "BottomCover"]
            })),
            &[Step::Puts("Faulty 5805 : thrusections is wrong")],
            &[]
        ),
        Step::Catch(&Step::Block(&[
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 914.876, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 6),
                ("EDGE", 9),
                ("WIRE", 5),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 27)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "s",
            "Faulty 5805 : s is not Closed shape"
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 5805 : result is not Closed shape"
        )),
        Step::CheckView("result"),
    ]
);
