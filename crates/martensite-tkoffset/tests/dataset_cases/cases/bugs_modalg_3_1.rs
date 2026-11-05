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
    bugs_modalg_3_bug5805_50,
    "tests/bugs/modalg_3/bug5805_50",
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
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_2"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 12150.0, 0.01, None)),
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
    bugs_modalg_3_bug5805_51,
    "tests/bugs/modalg_3/bug5805_51",
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
    bugs_modalg_3_bug5805_52,
    "tests/bugs/modalg_3/bug5805_52",
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
            Step::Topo(Topo::Explode("s", "f")),
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &["s_2", "s_3"])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 7051.59, 0.01, None)),
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
    bugs_modalg_3_bug5805_53,
    "tests/bugs/modalg_3/bug5805_53",
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
                0.0, 0.0, -10.0, 5.0, 5.0, -10.0, 0.0, 10.0, -10.0, 0.0, 0.0, -10.0
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
        Step::Check(Check::Props(Prop::Area, "result", 889.96, 0.01, None)),
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
    bugs_modalg_3_bug5805_54,
    "tests/bugs/modalg_3/bug5805_54",
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
                0.0, 0.0, -10.0, 5.0, 5.0, -10.0, 0.0, 10.0, -10.0, 0.0, 0.0, -10.0
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
        Step::Check(Check::Props(Prop::Area, "result", 376.733, 0.01, None)),
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

offset_case!(
    bugs_modalg_3_bug5805_55,
    "tests/bugs/modalg_3/bug5805_55",
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
                0.0, 0.0, -10.0, 5.0, 5.0, -10.0, 0.0, 10.0, -10.0, 0.0, 0.0, -10.0
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
        Step::Check(Check::Props(Prop::Area, "result", 889.96, 0.01, None)),
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
    bugs_modalg_3_bug5805_56,
    "tests/bugs/modalg_3/bug5805_56",
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
                0.0, 0.0, -10.0, 5.0, 5.0, -10.0, 0.0, 10.0, -10.0, 0.0, 0.0, -10.0
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
        Step::Check(Check::Props(Prop::Area, "result", 870.345, 0.01, None)),
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
    bugs_modalg_3_bug5805_57,
    "tests/bugs/modalg_3/bug5805_57",
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
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 5.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 664.447, 0.01, None)),
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
    bugs_modalg_3_bug5805_58,
    "tests/bugs/modalg_3/bug5805_58",
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
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 5.0])),
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
    bugs_modalg_3_bug5805_6,
    "tests/bugs/modalg_3/bug5805_6",
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
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Shape(OffsetShape {
                result: "result",
                shape: "a",
                value: -1.0,
                tolerance: None,
                complete: false,
                faces: &[]
            }))),
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

offset_case!(
    bugs_modalg_3_bug5805_7,
    "tests/bugs/modalg_3/bug5805_7",
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
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 5.0])),
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
    bugs_modalg_3_bug5805_8,
    "tests/bugs/modalg_3/bug5805_8",
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
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, -10.0, 5.0])),
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
    bugs_modalg_3_bug5805_9,
    "tests/bugs/modalg_3/bug5805_9",
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
            Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
            Step::Offset(Offset::Load("s", -0.5, &[])),
            Step::Offset(Offset::Perform("result"))
        ])),
        Step::Check(Check::Props(Prop::Area, "result", 518.774, 0.01, None)),
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
    bugs_modalg_3_bug616,
    "tests/bugs/modalg_3/bug616",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 616"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/OCC616.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "result",
                base: "a",
                count: 5,
                step: -2.0,
                approx: false,
                join: 'a',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts(
                "Faulty 616: function MKOFFSET works with exception"
            )],
            &[
                Step::Topo(Topo::Rename("result_1", "result")),
                Step::Check(Check::Shape("result")),
                Step::Puts("616 OK: function MKOFFSET works OK")
            ]
        ),
        Step::Check(Check::Props(Prop::Length, "result", 1062.78, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);
