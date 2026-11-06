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
    thrusection_not_solids_n3,
    "tests/thrusection/not_solids/N3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1690.09, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_n4,
    "tests/thrusection/not_solids/N4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_n5,
    "tests/thrusection/not_solids/N5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_n6,
    "tests/thrusection/not_solids/N6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1690.09, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_n7,
    "tests/thrusection/not_solids/N7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C7", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1507.96, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_n8,
    "tests/thrusection/not_solids/N8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C8", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1507.96, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_n9,
    "tests/thrusection/not_solids/N9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C9", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o1,
    "tests/thrusection/not_solids/O1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o2,
    "tests/thrusection/not_solids/O2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o3,
    "tests/thrusection/not_solids/O3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1690.09, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o4,
    "tests/thrusection/not_solids/O4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o5,
    "tests/thrusection/not_solids/O5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o6,
    "tests/thrusection/not_solids/O6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1690.09, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o7,
    "tests/thrusection/not_solids/O7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C7", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1507.96, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o8,
    "tests/thrusection/not_solids/O8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C8", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1507.96, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_o9,
    "tests/thrusection/not_solids/O9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C9", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p1,
    "tests/thrusection/not_solids/P1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p2,
    "tests/thrusection/not_solids/P2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p3,
    "tests/thrusection/not_solids/P3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1516.02, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p4,
    "tests/thrusection/not_solids/P4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p5,
    "tests/thrusection/not_solids/P5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p6,
    "tests/thrusection/not_solids/P6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1516.02, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p7,
    "tests/thrusection/not_solids/P7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C7", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p8,
    "tests/thrusection/not_solids/P8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C8", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_p9,
    "tests/thrusection/not_solids/P9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C9", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1131.35, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q1,
    "tests/thrusection/not_solids/Q1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1295.31, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q2,
    "tests/thrusection/not_solids/Q2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C2", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1295.31, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q3,
    "tests/thrusection/not_solids/Q3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C3", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 972.397, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q4,
    "tests/thrusection/not_solids/Q4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C4", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1295.31, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q5,
    "tests/thrusection/not_solids/Q5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C5", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1295.31, 0.01, None)),
    ]
);
