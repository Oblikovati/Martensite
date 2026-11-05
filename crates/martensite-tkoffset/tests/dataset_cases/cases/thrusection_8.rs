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
    thrusection_not_solids_locked_d1,
    "tests/thrusection/not_solids_locked/D1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D5", "w1")),
        Step::Topo(Topo::Restore("brep/D4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 161.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_f1,
    "tests/thrusection/not_solids_locked/F1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C2", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2513.27, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_g1,
    "tests/thrusection/not_solids_locked/G1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C3", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2196.82, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_h1,
    "tests/thrusection/not_solids_locked/H1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C4", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2513.27, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_i1,
    "tests/thrusection/not_solids_locked/I1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C5", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2513.27, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_j1,
    "tests/thrusection/not_solids_locked/J1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C6", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2196.82, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_k1,
    "tests/thrusection/not_solids_locked/K1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_l1,
    "tests/thrusection/not_solids_locked/L1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_m1,
    "tests/thrusection/not_solids_locked/M1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_n1,
    "tests/thrusection/not_solids_locked/N1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_o1,
    "tests/thrusection/not_solids_locked/O1",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_p1,
    "tests/thrusection/not_solids_locked/P1",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_q1,
    "tests/thrusection/not_solids_locked/Q1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C1", "w2")),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1295.31, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_r1,
    "tests/thrusection/not_solids_locked/R1",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2020.65, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_s1,
    "tests/thrusection/not_solids_locked/S1",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1690.09, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_t1,
    "tests/thrusection/not_solids_locked/T1",
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
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1834.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_u1,
    "tests/thrusection/not_solids_locked/U1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 762.417, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_v1,
    "tests/thrusection/not_solids_locked/V1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1507.96, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_w1,
    "tests/thrusection/not_solids_locked/W1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 0.0, 40.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("w2", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_w8,
    "tests/thrusection/not_solids_locked/W8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/buc60318.rle", "w")),
        Step::Topo(Topo::Explode("w", "W")),
        Step::Topo(Topo::SetFlags("w", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["w_1", "w_2", "w_3", "w_4", "w_5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 254109.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a1,
    "tests/thrusection/solids/A1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21295_1.brep", "f1")),
        Step::Topo(Topo::Restore("brep/cts21295_2.brep", "f2")),
        Step::Topo(Topo::Restore("brep/cts21295_3.brep", "f3")),
        Step::Topo(Topo::Explode("f1", "w")),
        Step::Topo(Topo::Explode("f2", "w")),
        Step::Topo(Topo::Explode("f3", "w")),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["f1_1", "f2_1", "f3_1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 49757.1, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a2,
    "tests/thrusection/solids/A2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a3,
    "tests/thrusection/solids/A3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21570_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/cts21570_2.rle", "w2")),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a4,
    "tests/thrusection/solids/A4",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a5,
    "tests/thrusection/solids/A5",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a6,
    "tests/thrusection/solids/A6",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a7,
    "tests/thrusection/solids/A7",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a8,
    "tests/thrusection/solids/A8",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_a9,
    "tests/thrusection/solids/A9",
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
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w2", "w1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 29817.5, 0.01, None)),
    ]
);

offset_case!(
    thrusection_solids_locked_a1,
    "tests/thrusection/solids_locked/A1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/cts21295_1.brep", "f1")),
        Step::Topo(Topo::Restore("brep/cts21295_2.brep", "f2")),
        Step::Topo(Topo::Restore("brep/cts21295_3.brep", "f3")),
        Step::Topo(Topo::Explode("f1", "w")),
        Step::Topo(Topo::Explode("f2", "w")),
        Step::Topo(Topo::Explode("f3", "w")),
        Step::Topo(Topo::SetFlags("f1_1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("f2_1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("f3_1", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["f1_1", "f2_1", "f3_1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 49757.1, 0.01, None)),
    ]
);
