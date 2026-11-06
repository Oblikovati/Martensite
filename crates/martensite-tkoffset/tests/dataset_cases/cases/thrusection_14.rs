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
    thrusection_specific_locked_d3,
    "tests/thrusection/specific_locked/D3",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_d4,
    "tests/thrusection/specific_locked/D4",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_d5,
    "tests/thrusection/specific_locked/D5",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_d7,
    "tests/thrusection/specific_locked/D7",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_d8,
    "tests/thrusection/specific_locked/D8",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_d9,
    "tests/thrusection/specific_locked/D9",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e1,
    "tests/thrusection/specific_locked/E1",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e2,
    "tests/thrusection/specific_locked/E2",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e3,
    "tests/thrusection/specific_locked/E3",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e4,
    "tests/thrusection/specific_locked/E4",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e6,
    "tests/thrusection/specific_locked/E6",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e7,
    "tests/thrusection/specific_locked/E7",
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
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_e8,
    "tests/thrusection/specific_locked/E8",
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
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_f1,
    "tests/thrusection/specific_locked/F1",
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
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_f2,
    "tests/thrusection/specific_locked/F2",
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
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_f3,
    "tests/thrusection/specific_locked/F3",
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
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_f5,
    "tests/thrusection/specific_locked/F5",
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
        Step::Topo(Topo::SetFlags("wc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wc2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_f6,
    "tests/thrusection/specific_locked/F6",
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
        Step::Topo(Topo::SetFlags("wc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wc2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
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
    thrusection_specific_locked_f7,
    "tests/thrusection/specific_locked/F7",
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
        Step::Topo(Topo::SetFlags("wc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wc2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
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

offset_case!(
    thrusection_specific_locked_f9,
    "tests/thrusection/specific_locked/F9",
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
            &[40.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[40.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Topo(Topo::SetFlags("wc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wc2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 419.166, 0.01, None)),
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
    thrusection_specific_locked_g1,
    "tests/thrusection/specific_locked/G1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Faulty 7166 : result is not Closed shape"),
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
            &[40.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[40.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Topo(Topo::SetFlags("wc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wc2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 702.858, 0.01, None)),
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
    thrusection_specific_locked_g2,
    "tests/thrusection/specific_locked/G2",
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
            &[40.0, 0.0, 20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("wc1", &["ec1"])),
        Step::Curve(Curve::Circle(
            "c2",
            &[40.0, 0.0, -20.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, 0.1]
        )),
        Step::Profile(Profile::Edge("ec2", "c2", None)),
        Step::Profile(Profile::Wire("wc2", &["ec2"])),
        Step::Topo(Topo::SetFlags("wc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wc2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 702.921, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 4),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 20)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_g4,
    "tests/thrusection/specific_locked/G4",
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
        Step::Profile(Profile::Vertex("v4", [55.0, 2.5, 100.0])),
        Step::Profile(Profile::Vertex("v5", [55.0, 2.5, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Curve(Curve::Bezier(
            "bzc1",
            &[
                5.0, 5.0, 5.0, 30.0, 5.0, -5.0, 30.0, -5.0, -5.0, 30.0, -5.0, 5.0, 30.0, 5.0, 5.0,
                30.0
            ]
        )),
        Step::Profile(Profile::Edge("ebzc1", "bzc1", None)),
        Step::Profile(Profile::Wire("wbzc1", &["ebzc1"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbc1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 10.0, 1.0, -2.0, 2.0, 10.0, 1.0, 0.0, 1.0, 10.0,
                1.0, 2.0, 2.0, 10.0, 1.0, 1.0, 0.0, 10.0, 1.0, 2.0, -2.0, 10.0, 1.0, 0.0, -1.0,
                10.0, 1.0, -2.0, -2.0, 10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("epbc1", "pbc1", None)),
        Step::Profile(Profile::Wire("wpbc1", &["epbc1"])),
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
        Step::Curve(Curve::PeriodicBSpline(
            "pbc2",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, -10.0, 1.0, -2.0, 2.0, -10.0, 1.0, 0.0, 1.0, -10.0,
                1.0, 2.0, 2.0, -10.0, 1.0, 1.0, 0.0, -10.0, 1.0, 2.0, -2.0, -10.0, 1.0, 0.0, -1.0,
                -10.0, 1.0, -2.0, -2.0, -10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("epbc2", "pbc2", None)),
        Step::Profile(Profile::Wire("wpbc2", &["epbc2"])),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Curve(Curve::Bezier(
            "bzc2",
            &[
                5.0, 5.0, 5.0, -30.0, 5.0, -5.0, -30.0, -5.0, -5.0, -30.0, -5.0, 5.0, -30.0, 5.0,
                5.0, -30.0
            ]
        )),
        Step::Profile(Profile::Edge("ebzc2", "bzc2", None)),
        Step::Profile(Profile::Wire("wbzc2", &["ebzc2"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wbzc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wpbc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wpbc2", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wbzc2", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &[
                "v4", "w1", "wbzc1", "w11", "wpbc1", "w21", "wpbc2", "w111", "wbzc2", "w101", "v5"
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 12786.5, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 101),
                ("EDGE", 209),
                ("WIRE", 110),
                ("FACE", 110),
                ("SHELL", 1),
                ("SOLID", 2),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 532)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_g6,
    "tests/thrusection/specific_locked/G6",
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
        Step::Curve(Curve::Bezier(
            "bzc1",
            &[
                5.0, 5.0, 5.0, 30.0, 5.0, -5.0, 30.0, -5.0, -5.0, 30.0, -5.0, 5.0, 30.0, 5.0, 5.0,
                30.0
            ]
        )),
        Step::Profile(Profile::Edge("ebzc1", "bzc1", None)),
        Step::Profile(Profile::Wire("wbzc1", &["ebzc1"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbc1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 10.0, 1.0, -2.0, 2.0, 10.0, 1.0, 0.0, 1.0, 10.0,
                1.0, 2.0, 2.0, 10.0, 1.0, 1.0, 0.0, 10.0, 1.0, 2.0, -2.0, 10.0, 1.0, 0.0, -1.0,
                10.0, 1.0, -2.0, -2.0, 10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("epbc1", "pbc1", None)),
        Step::Profile(Profile::Wire("wpbc1", &["epbc1"])),
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
        Step::Curve(Curve::PeriodicBSpline(
            "pbc2",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, -10.0, 1.0, -2.0, 2.0, -10.0, 1.0, 0.0, 1.0, -10.0,
                1.0, 2.0, 2.0, -10.0, 1.0, 1.0, 0.0, -10.0, 1.0, 2.0, -2.0, -10.0, 1.0, 0.0, -1.0,
                -10.0, 1.0, -2.0, -2.0, -10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("epbc2", "pbc2", None)),
        Step::Profile(Profile::Wire("wpbc2", &["epbc2"])),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Curve(Curve::Bezier(
            "bzc2",
            &[
                5.0, 5.0, 5.0, -30.0, 5.0, -5.0, -30.0, -5.0, -5.0, -30.0, -5.0, 5.0, -30.0, 5.0,
                5.0, -30.0
            ]
        )),
        Step::Profile(Profile::Edge("ebzc2", "bzc2", None)),
        Step::Profile(Profile::Wire("wbzc2", &["ebzc2"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wbzc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w11", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wpbc1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w21", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wpbc2", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w111", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("wbzc2", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w101", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &[
                "v4", "w1", "wbzc1", "w11", "wpbc1", "w21", "wpbc2", "w111", "wbzc2", "w101", "v5"
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 12436.8, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 101),
                ("EDGE", 209),
                ("WIRE", 110),
                ("FACE", 110),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 532)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_n5,
    "tests/thrusection/specific_locked/N5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("28642"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline("p1", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p2", &[2.0, 1.0, 0.0, 2.0, 2.0, 0.0])),
        Step::Profile(Profile::Polyline("p3", &[3.0, 0.0, 0.0, 4.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p4", &[2.0, -1.0, 0.0, 2.0, -2.0, 0.0])),
        Step::Topo(Topo::SetFlags("p1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("p2", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("p3", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("p4", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["p1", "p2", "p3", "p4", "p1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 10.1587, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_locked_n6,
    "tests/thrusection/specific_locked/N6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("28642"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline("p1", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p2", &[2.0, 1.0, 0.0, 2.0, 2.0, 0.0])),
        Step::Profile(Profile::Polyline("p3", &[3.0, 0.0, 0.0, 4.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p4", &[2.0, -1.0, 0.0, 2.0, -2.0, 0.0])),
        Step::Topo(Topo::SetFlags("p1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("p2", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("p3", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("p4", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["p1", "p2", "p3", "p4", "p1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6.0, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 8),
                ("EDGE", 12),
                ("WIRE", 4),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 29)
            ]
        )),
    ]
);
