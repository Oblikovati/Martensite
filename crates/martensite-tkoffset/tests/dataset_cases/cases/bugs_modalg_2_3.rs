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
    bugs_modalg_2_bug5805_26,
    "tests/bugs/modalg_2/bug5805_26",
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
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_4"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 2011.91, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 18),
                ("WIRE", 10),
                ("FACE", 9),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 51)
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
    bugs_modalg_2_bug5805_27,
    "tests/bugs/modalg_2/bug5805_27",
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
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_5"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 1755.54, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 18),
                ("WIRE", 10),
                ("FACE", 9),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 51)
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
    bugs_modalg_2_bug5805_28,
    "tests/bugs/modalg_2/bug5805_28",
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
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_4", "s_5"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 1735.06, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 18),
                ("WIRE", 10),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 50)
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
    bugs_modalg_2_bug5805_29,
    "tests/bugs/modalg_2/bug5805_29",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("5805"),
        Step::Puts("7582"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ec1"])),
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 30.0])),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("BottomCover", &["ec2"])),
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
        Step::Check(Check::Props(Prop::Area, "result", 5829.59, 0.01, None)),
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
    bugs_modalg_2_bug5805_3,
    "tests/bugs/modalg_2/bug5805_3",
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
        Step::Topo(Topo::Restore("brep/OCC5805.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Shape(OffsetShape {
                result: "result",
                shape: "a",
                value: -1.0,
                tolerance: None,
                complete: false,
                faces: &["a_6"]
            }))),
            &[Step::Puts("Faulty 5805 : offsetshape is wrong")],
            &[]
        ),
        Step::If(
            Cond::Bound("result"),
            &[
                Step::Check(Check::MaxTolerance("result", None, Some(1.0))),
                Step::Check(Check::Props(Prop::Area, "result", 1185.03, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 10),
                        ("EDGE", 15),
                        ("WIRE", 7),
                        ("FACE", 7),
                        ("SHELL", 1),
                        ("SOLID", 1),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 41)
                    ]
                )),
                Step::CheckView("result")
            ],
            &[]
        ),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_30,
    "tests/bugs/modalg_2/bug5805_30",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ec1"])),
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 30.0])),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("BottomCover", &["ec2"])),
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
        Step::Check(Check::Props(Prop::Area, "result", 1336.75, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 23)
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
    bugs_modalg_2_bug5805_31,
    "tests/bugs/modalg_2/bug5805_31",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ec1"])),
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 30.0])),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("BottomCover", &["ec2"])),
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
        Step::Check(Check::Props(Prop::Area, "result", 7157.43, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 23)
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
    bugs_modalg_2_bug5805_32,
    "tests/bugs/modalg_2/bug5805_32",
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
        Step::Puts("7582"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ec1"])),
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 30.0])),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("BottomCover", &["ec2"])),
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
        Step::Check(Check::Props(Prop::Area, "result", 1223.65, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 22)
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
    bugs_modalg_2_bug5805_33,
    "tests/bugs/modalg_2/bug5805_33",
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
            "ell1",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Curve(Curve::Ellipse(
            "ell2",
            &[-10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 4.0, 2.0]
        )),
        Step::Profile(Profile::Edge("ee2", "ell2", None)),
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
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 443.123, 0.01, None)),
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
    bugs_modalg_2_bug5805_34,
    "tests/bugs/modalg_2/bug5805_34",
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
            "ell1",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Curve(Curve::Ellipse(
            "ell2",
            &[-10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 4.0, 2.0]
        )),
        Step::Profile(Profile::Edge("ee2", "ell2", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 906.895, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 23)
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
    bugs_modalg_2_bug5805_35,
    "tests/bugs/modalg_2/bug5805_35",
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
            "ell1",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Curve(Curve::Ellipse(
            "ell2",
            &[-10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 4.0, 2.0]
        )),
        Step::Profile(Profile::Edge("ee2", "ell2", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 940.867, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 23)
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
    bugs_modalg_2_bug5805_36,
    "tests/bugs/modalg_2/bug5805_36",
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
            "ell1",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0, 3.0]
        )),
        Step::Profile(Profile::Edge("ee1", "ell1", None)),
        Step::Profile(Profile::Wire("UpCover", &["ee1"])),
        Step::Curve(Curve::Ellipse(
            "ell2",
            &[-10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 4.0, 2.0]
        )),
        Step::Profile(Profile::Edge("ee2", "ell2", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 882.928, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 22)
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
    bugs_modalg_2_bug5805_37,
    "tests/bugs/modalg_2/bug5805_37",
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
                0.0, 0.0, 5.0, 6.0, 0.0, 5.0, 6.0, 6.0, 5.0, 0.0, 6.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                3.0, -2.0, -5.0, 8.0, 3.0, -5.0, 3.0, 8.0, -5.0, -2.0, 3.0, -5.0, 3.0, -2.0, -5.0
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
        Step::Check(Check::Props(Prop::Area, "result", 246.821, 0.01, None)),
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
    bugs_modalg_2_bug5805_38,
    "tests/bugs/modalg_2/bug5805_38",
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
                0.0, 0.0, 5.0, 6.0, 0.0, 5.0, 6.0, 6.0, 5.0, 0.0, 6.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                3.0, -2.0, -5.0, 8.0, 3.0, -5.0, 3.0, 8.0, -5.0, -2.0, 3.0, -5.0, 3.0, -2.0, -5.0
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
        Step::Check(Check::Props(Prop::Area, "result", 544.931, 0.01, None)),
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
    bugs_modalg_2_bug5805_39,
    "tests/bugs/modalg_2/bug5805_39",
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
                0.0, 0.0, 5.0, 6.0, 0.0, 5.0, 6.0, 6.0, 5.0, 0.0, 6.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                3.0, -2.0, -5.0, 8.0, 3.0, -5.0, 3.0, 8.0, -5.0, -2.0, 3.0, -5.0, 3.0, -2.0, -5.0
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
        Step::Check(Check::Props(Prop::Area, "result", 524.758, 0.01, None)),
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
    bugs_modalg_2_bug5805_4,
    "tests/bugs/modalg_2/bug5805_4",
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
        Step::Topo(Topo::Restore("brep/OCC5805.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("a", -1.0, &["a_6"])),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Perform("result"))),
            &[Step::Puts("Faulty 5805 : offsetshape is wrong")],
            &[]
        ),
        Step::If(
            Cond::Bound("result"),
            &[
                Step::Check(Check::MaxTolerance("result", None, Some(1.0))),
                Step::Check(Check::Props(Prop::Area, "result", 1185.03, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 10),
                        ("EDGE", 15),
                        ("WIRE", 7),
                        ("FACE", 7),
                        ("SHELL", 1),
                        ("SOLID", 1),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 41)
                    ]
                )),
                Step::CheckView("result")
            ],
            &[]
        ),
    ]
);

offset_case!(
    bugs_modalg_2_bug5805_40,
    "tests/bugs/modalg_2/bug5805_40",
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
                0.0, 0.0, 5.0, 6.0, 0.0, 5.0, 6.0, 6.0, 5.0, 0.0, 6.0, 5.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "BottomCover",
            &[
                3.0, -2.0, -5.0, 8.0, 3.0, -5.0, 3.0, 8.0, -5.0, -2.0, 3.0, -5.0, 3.0, -2.0, -5.0
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
        Step::Check(Check::Props(Prop::Area, "result", 485.358, 0.01, None)),
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
    bugs_modalg_2_bug5805_41,
    "tests/bugs/modalg_2/bug5805_41",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
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
        Step::Curve(Curve::Bezier(
            "bc2",
            &[
                5.0, 4.0, 4.0, -5.0, 4.0, -4.0, -5.0, -4.0, -4.0, -5.0, -4.0, 4.0, -5.0, 4.0, 4.0,
                -5.0
            ]
        )),
        Step::Profile(Profile::Edge("e2", "bc2", None)),
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
    bugs_modalg_2_bug5805_42,
    "tests/bugs/modalg_2/bug5805_42",
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
        Step::Curve(Curve::Bezier(
            "bc2",
            &[
                5.0, 4.0, 4.0, -5.0, 4.0, -4.0, -5.0, -4.0, -4.0, -5.0, -4.0, 4.0, -5.0, 4.0, 4.0,
                -5.0
            ]
        )),
        Step::Profile(Profile::Edge("e2", "bc2", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 495.635, 0.01, None)),
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
    bugs_modalg_2_bug5805_43,
    "tests/bugs/modalg_2/bug5805_43",
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
        Step::Curve(Curve::Bezier(
            "bc2",
            &[
                5.0, 4.0, 4.0, -5.0, 4.0, -4.0, -5.0, -4.0, -4.0, -5.0, -4.0, 4.0, -5.0, 4.0, 4.0,
                -5.0
            ]
        )),
        Step::Profile(Profile::Edge("e2", "bc2", None)),
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
    bugs_modalg_2_bug5805_44,
    "tests/bugs/modalg_2/bug5805_44",
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
        Step::Curve(Curve::Bezier(
            "bc2",
            &[
                5.0, 4.0, 4.0, -5.0, 4.0, -4.0, -5.0, -4.0, -4.0, -5.0, -4.0, 4.0, -5.0, 4.0, 4.0,
                -5.0
            ]
        )),
        Step::Profile(Profile::Edge("e2", "bc2", None)),
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
    bugs_modalg_2_bug5805_45,
    "tests/bugs/modalg_2/bug5805_45",
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
        Step::Place(Place::Scale(&["e2"], [0.0, 0.0, 0.0, 2.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 909.684, 0.01, None)),
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
    bugs_modalg_2_bug5805_46,
    "tests/bugs/modalg_2/bug5805_46",
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
        Step::Place(Place::Scale(&["e2"], [0.0, 0.0, 0.0, 2.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 1627.98, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 23)
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
    bugs_modalg_2_bug5805_47,
    "tests/bugs/modalg_2/bug5805_47",
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
        Step::Place(Place::Scale(&["e2"], [0.0, 0.0, 0.0, 2.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 1673.57, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 5),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 23)
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
    bugs_modalg_2_bug5805_48,
    "tests/bugs/modalg_2/bug5805_48",
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
        Step::Place(Place::Scale(&["e2"], [0.0, 0.0, 0.0, 2.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 1615.27, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 6),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 22)
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
    bugs_modalg_2_bug5805_49,
    "tests/bugs/modalg_2/bug5805_49",
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
        Step::Prim(Prim::Cone("s", &[30.0, 5.0, 20.0])),
        Step::Topo(Topo::Explode("s", "sh")),
        Step::Topo(Topo::Rename("s_1", "s")),
        Step::Catch(&Step::Block(&[
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 5829.59, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 12)
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
    bugs_modalg_2_bug5805_5,
    "tests/bugs/modalg_2/bug5805_5",
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
        Step::Topo(Topo::Restore("brep/OCC5805.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("a", -1.0, &[])),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Perform("result"))),
            &[Step::Puts("Faulty 5805 : offsetshape is wrong")],
            &[]
        ),
        Step::If(
            Cond::Bound("result"),
            &[
                Step::Check(Check::MaxTolerance("result", None, Some(1.0))),
                Step::Check(Check::Props(Prop::Area, "result", 876.584, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 10),
                        ("EDGE", 15),
                        ("WIRE", 7),
                        ("FACE", 7),
                        ("SHELL", 1),
                        ("SOLID", 1),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 41)
                    ]
                )),
                Step::CheckView("result")
            ],
            &[]
        ),
    ]
);
