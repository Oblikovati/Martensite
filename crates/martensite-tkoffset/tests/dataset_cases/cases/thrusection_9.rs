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
    thrusection_solids_locked_a2,
    "tests/thrusection/solids_locked/A2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a3,
    "tests/thrusection/solids_locked/A3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a4,
    "tests/thrusection/solids_locked/A4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::Orient("w1", 'F')),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a5,
    "tests/thrusection/solids_locked/A5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::Orient("w1", 'F')),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a6,
    "tests/thrusection/solids_locked/A6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::Orient("w1", 'F')),
        Step::Topo(Topo::Orient("w2", 'R')),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a7,
    "tests/thrusection/solids_locked/A7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::Orient("w1", 'F')),
        Step::Topo(Topo::Orient("w2", 'R')),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a8,
    "tests/thrusection/solids_locked/A8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::Orient("w1", 'F')),
        Step::Topo(Topo::Orient("w2", 'R')),
        Step::Topo(Topo::Orient("w1", 'R')),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a9,
    "tests/thrusection/solids_locked/A9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Topo(Topo::Orient("w1", 'F')),
        Step::Topo(Topo::Orient("w2", 'R')),
        Step::Topo(Topo::Orient("w1", 'R')),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_specific_a1,
    "tests/thrusection/specific/A1",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 490.101, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 27)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_a2,
    "tests/thrusection/specific/A2",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 490.101, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 28)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_a3,
    "tests/thrusection/specific/A3",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 651.466, 0.01, None)),
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
    thrusection_specific_a4,
    "tests/thrusection/specific/A4",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 651.466, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 19)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_a5,
    "tests/thrusection/specific/A5",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 485.502, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 27)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_a6,
    "tests/thrusection/specific/A6",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 485.502, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 28)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_a7,
    "tests/thrusection/specific/A7",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 646.621, 0.01, None)),
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
    thrusection_specific_a8,
    "tests/thrusection/specific/A8",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 646.621, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 19)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_a9,
    "tests/thrusection/specific/A9",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 1404.96, 0.01, None)),
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
    thrusection_specific_b1,
    "tests/thrusection/specific/B1",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 1404.96, 0.01, None)),
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
    thrusection_specific_b2,
    "tests/thrusection/specific/B2",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 1829.23, 0.01, None)),
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
    thrusection_specific_b3,
    "tests/thrusection/specific/B3",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 1829.23, 0.01, None)),
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
    thrusection_specific_b4,
    "tests/thrusection/specific/B4",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 2160.94, 0.01, None)),
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
    thrusection_specific_b5,
    "tests/thrusection/specific/B5",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 2160.94, 0.01, None)),
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
    thrusection_specific_b6,
    "tests/thrusection/specific/B6",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 2556.25, 0.01, None)),
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
    thrusection_specific_b7,
    "tests/thrusection/specific/B7",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
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
        Step::Check(Check::Props(Prop::Area, "result", 2556.25, 0.01, None)),
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
    thrusection_specific_b8,
    "tests/thrusection/specific/B8",
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
        Step::Check(Check::Props(Prop::Area, "result", 1021.13, 0.01, None)),
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
    thrusection_specific_b9,
    "tests/thrusection/specific/B9",
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
        Step::Check(Check::Props(Prop::Area, "result", 1021.13, 0.01, None)),
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
    thrusection_specific_c1,
    "tests/thrusection/specific/C1",
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
        Step::Check(Check::Props(Prop::Area, "result", 1346.42, 0.01, None)),
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
    thrusection_specific_c2,
    "tests/thrusection/specific/C2",
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
        Step::Check(Check::Props(Prop::Area, "result", 1346.42, 0.01, None)),
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
    thrusection_specific_c3,
    "tests/thrusection/specific/C3",
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
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1353.78, 0.01, None)),
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
    thrusection_specific_c4,
    "tests/thrusection/specific/C4",
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
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1353.78, 0.01, None)),
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
