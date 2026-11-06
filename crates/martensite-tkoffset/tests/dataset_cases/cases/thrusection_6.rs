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
    thrusection_not_solids_q6,
    "tests/thrusection/not_solids/Q6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C6", "w2")),
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
    thrusection_not_solids_q7,
    "tests/thrusection/not_solids/Q7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C7", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 762.417, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q8,
    "tests/thrusection/not_solids/Q8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C8", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 762.417, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_q9,
    "tests/thrusection/not_solids/Q9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C9", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 572.05, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_r1,
    "tests/thrusection/not_solids/R1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C1", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_r2,
    "tests/thrusection/not_solids/R2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C1", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_r3,
    "tests/thrusection/not_solids/R3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C1", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
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
    thrusection_not_solids_r4,
    "tests/thrusection/not_solids/R4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C1", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_r5,
    "tests/thrusection/not_solids/R5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C2", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_r6,
    "tests/thrusection/not_solids/R6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C2", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_r7,
    "tests/thrusection/not_solids/R7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C2", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
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
    thrusection_not_solids_r8,
    "tests/thrusection/not_solids/R8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C2", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_r9,
    "tests/thrusection/not_solids/R9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C3", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_s1,
    "tests/thrusection/not_solids/S1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C3", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_s2,
    "tests/thrusection/not_solids/S2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C3", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1516.01, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_s3,
    "tests/thrusection/not_solids/S3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C3", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_s4,
    "tests/thrusection/not_solids/S4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C4", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_s5,
    "tests/thrusection/not_solids/S5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C4", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_s6,
    "tests/thrusection/not_solids/S6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C4", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
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
    thrusection_not_solids_s7,
    "tests/thrusection/not_solids/S7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C4", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_s8,
    "tests/thrusection/not_solids/S8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C5", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_s9,
    "tests/thrusection/not_solids/S9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C5", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_t1,
    "tests/thrusection/not_solids/T1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C5", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
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
    thrusection_not_solids_t2,
    "tests/thrusection/not_solids/T2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C5", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_t3,
    "tests/thrusection/not_solids/T3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C6", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_t4,
    "tests/thrusection/not_solids/T4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C6", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_t5,
    "tests/thrusection/not_solids/T5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C6", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1516.01, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_t6,
    "tests/thrusection/not_solids/T6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C6", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_t7,
    "tests/thrusection/not_solids/T7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_t8,
    "tests/thrusection/not_solids/T8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
