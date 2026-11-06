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
    thrusection_specific_f8,
    "tests/thrusection/specific/F8",
    Shape,
    1000,
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
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["wc1", "w1", "wc2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 419.103, 0.01, None)),
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
    thrusection_specific_f9,
    "tests/thrusection/specific/F9",
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
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
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
    thrusection_specific_g1,
    "tests/thrusection/specific/G1",
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
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
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
    thrusection_specific_g2,
    "tests/thrusection/specific/G2",
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
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
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
    thrusection_specific_g3,
    "tests/thrusection/specific/G3",
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
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
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
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 531)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_g4,
    "tests/thrusection/specific/G4",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
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
    thrusection_specific_g5,
    "tests/thrusection/specific/G5",
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
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
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
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 531)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_g6,
    "tests/thrusection/specific/G6",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
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
    thrusection_specific_h1,
    "tests/thrusection/specific/H1",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1187.23, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h2,
    "tests/thrusection/specific/H2",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1192.53, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h3,
    "tests/thrusection/specific/H3",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1187.23, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h4,
    "tests/thrusection/specific/H4",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1192.53, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h5,
    "tests/thrusection/specific/H5",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 790.139, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 18)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_h6,
    "tests/thrusection/specific/H6",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [5.0, 2.5, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1198.3, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h7,
    "tests/thrusection/specific/H7",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [5.0, 2.5, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1192.88, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h8,
    "tests/thrusection/specific/H8",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [5.0, 2.5, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1187.23, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_h9,
    "tests/thrusection/specific/H9",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [5.0, 2.5, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1192.53, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 4),
                ("EDGE", 6),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_i1,
    "tests/thrusection/specific/I1",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Profile(Profile::Polyline(
            "s2",
            &[0.0, 5.0, 0.0, 5.0, 0.0, 0.0, -5.0, -5.0, 0.0, 0.0, 5.0, 0.0]
        )),
        Step::Profile(Profile::Vertex("v1", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 2.5, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 795.95, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 18)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_i2,
    "tests/thrusection/specific/I2",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1294.43, 0.01, None)),
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
    thrusection_specific_i3,
    "tests/thrusection/specific/I3",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1313.1, 0.01, None)),
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
    thrusection_specific_i4,
    "tests/thrusection/specific/I4",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1294.43, 0.01, None)),
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
    thrusection_specific_i5,
    "tests/thrusection/specific/I5",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1313.1, 0.01, None)),
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
    thrusection_specific_i6,
    "tests/thrusection/specific/I6",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 858.182, 0.01, None)),
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
    thrusection_specific_i7,
    "tests/thrusection/specific/I7",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2121.34, 0.01, None)),
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
    thrusection_specific_i8,
    "tests/thrusection/specific/I8",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2285.24, 0.01, None)),
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
    thrusection_specific_i9,
    "tests/thrusection/specific/I9",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1294.43, 0.01, None)),
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
    thrusection_specific_j1,
    "tests/thrusection/specific/J1",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1313.1, 0.01, None)),
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
    thrusection_specific_j2,
    "tests/thrusection/specific/J2",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 5.0])),
        Step::Profile(Profile::Edge("ec1", "c1", None)),
        Step::Profile(Profile::Wire("s2", &["ec1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1259.14, 0.01, None)),
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
    thrusection_specific_j3,
    "tests/thrusection/specific/J3",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2042.02, 0.01, None)),
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
    thrusection_specific_j4,
    "tests/thrusection/specific/J4",
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
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2081.25, 0.01, None)),
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
