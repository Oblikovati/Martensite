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
    thrusection_specific_c5,
    "tests/thrusection/specific/C5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1683.87, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 8)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_c6,
    "tests/thrusection/specific/C6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1683.87, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_c7,
    "tests/thrusection/specific/C7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 449.295, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 13)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_c8,
    "tests/thrusection/specific/C8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 449.295, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_c9,
    "tests/thrusection/specific/C9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 597.473, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 8)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d1,
    "tests/thrusection/specific/D1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 597.473, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d2,
    "tests/thrusection/specific/D2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 740.079, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 13)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d3,
    "tests/thrusection/specific/D3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 740.079, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d4,
    "tests/thrusection/specific/D4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 881.045, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 8)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d5,
    "tests/thrusection/specific/D5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 881.045, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d6,
    "tests/thrusection/specific/D6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 245.485, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 13)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d7,
    "tests/thrusection/specific/D7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 245.485, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d8,
    "tests/thrusection/specific/D8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 327.161, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 8)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_d9,
    "tests/thrusection/specific/D9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 327.161, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e1,
    "tests/thrusection/specific/E1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 400.925, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 13)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e2,
    "tests/thrusection/specific/E2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 400.925, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e3,
    "tests/thrusection/specific/E3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 478.063, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 8)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e4,
    "tests/thrusection/specific/E4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 422.859, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e5,
    "tests/thrusection/specific/E5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [55.0, 2.5, 100.0])),
        Step::Profile(Profile::Vertex("v5", [55.0, 2.5, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 12651.5, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 57),
                ("EDGE", 121),
                ("WIRE", 66),
                ("FACE", 66),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 311)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e6,
    "tests/thrusection/specific/E6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [55.0, 2.5, 100.0])),
        Step::Profile(Profile::Vertex("v5", [55.0, 2.5, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 12651.5, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 57),
                ("EDGE", 121),
                ("WIRE", 66),
                ("FACE", 66),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 312)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e7,
    "tests/thrusection/specific/E7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [55.0, 2.5, 100.0])),
        Step::Profile(Profile::Vertex("v5", [55.0, 2.5, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 56691.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 33),
                ("WIRE", 11),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 58)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e8,
    "tests/thrusection/specific/E8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [55.0, 2.5, 100.0])),
        Step::Profile(Profile::Vertex("v5", [55.0, 2.5, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 56691.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 33),
                ("WIRE", 11),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 59)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_e9,
    "tests/thrusection/specific/E9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [2.5, 5.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [2.5, 5.0, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 12301.8, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 57),
                ("EDGE", 121),
                ("WIRE", 66),
                ("FACE", 66),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 311)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f1,
    "tests/thrusection/specific/F1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [2.5, 5.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [2.5, 5.0, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 12301.8, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 57),
                ("EDGE", 121),
                ("WIRE", 66),
                ("FACE", 66),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 312)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f2,
    "tests/thrusection/specific/F2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [2.5, 5.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [2.5, 5.0, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 37693.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 33),
                ("WIRE", 11),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 58)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f3,
    "tests/thrusection/specific/F3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v4", [2.5, 5.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [2.5, 5.0, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "w11", "w21", "w111", "w101", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 37693.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 33),
                ("WIRE", 11),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 59)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f4,
    "tests/thrusection/specific/F4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Faulty 7166 : result is not Closed shape"),
        Step::Puts("TODO 7166 ALL: Error :  is WRONG because number of  "),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Curve(Curve::Circle(
            "c1",
            &[0.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[0.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 256.812, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 6),
                ("EDGE", 10),
                ("WIRE", 4),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 25)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f5,
    "tests/thrusection/specific/F5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error :  is WRONG because number of  "),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Curve(Curve::Circle(
            "c1",
            &[0.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[0.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 256.875, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 6),
                ("EDGE", 10),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 30)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f6,
    "tests/thrusection/specific/F6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Faulty 7166 : result is not Closed shape"),
        Step::Puts("TODO 7166 ALL: Error :  is WRONG because number of  "),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Curve(Curve::Circle(
            "c1",
            &[0.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[0.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 332.724, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 15)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_f7,
    "tests/thrusection/specific/F7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error :  is WRONG because number of  "),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
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
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Curve(Curve::Circle(
            "c1",
            &[0.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[0.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 314.974, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);
