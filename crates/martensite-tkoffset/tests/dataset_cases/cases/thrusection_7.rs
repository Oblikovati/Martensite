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
    thrusection_not_solids_t9,
    "tests/thrusection/not_solids/T9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_u1,
    "tests/thrusection/not_solids/U1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_u2,
    "tests/thrusection/not_solids/U2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_u3,
    "tests/thrusection/not_solids/U3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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

offset_case!(
    thrusection_not_solids_u4,
    "tests/thrusection/not_solids/U4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_u5,
    "tests/thrusection/not_solids/U5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_u6,
    "tests/thrusection/not_solids/U6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_u7,
    "tests/thrusection/not_solids/U7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_u8,
    "tests/thrusection/not_solids/U8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1131.35, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_u9,
    "tests/thrusection/not_solids/U9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_v1,
    "tests/thrusection/not_solids/V1",
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
    thrusection_not_solids_v2,
    "tests/thrusection/not_solids/V2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
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

offset_case!(
    thrusection_not_solids_v3,
    "tests/thrusection/not_solids/V3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_v4,
    "tests/thrusection/not_solids/V4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_v5,
    "tests/thrusection/not_solids/V5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
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
    thrusection_not_solids_v6,
    "tests/thrusection/not_solids/V6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
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

offset_case!(
    thrusection_not_solids_v7,
    "tests/thrusection/not_solids/V7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_v8,
    "tests/thrusection/not_solids/V8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C11", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_v9,
    "tests/thrusection/not_solids/V9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1314.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_w1,
    "tests/thrusection/not_solids/W1",
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
    thrusection_not_solids_w2,
    "tests/thrusection/not_solids/W2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1131.35, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_w3,
    "tests/thrusection/not_solids/W3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C12", "w1")),
        Step::Topo(Topo::Restore("brep/C13", "w2")),
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
    thrusection_not_solids_w4,
    "tests/thrusection/not_solids/W4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C10", "w2")),
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
    thrusection_not_solids_w5,
    "tests/thrusection/not_solids/W5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C11", "w2")),
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
    thrusection_not_solids_w6,
    "tests/thrusection/not_solids/W6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C13", "w1")),
        Step::Topo(Topo::Restore("brep/C12", "w2")),
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
    thrusection_not_solids_w7,
    "tests/thrusection/not_solids/W7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/buc60281_1.rle", "w1")),
        Step::Topo(Topo::Restore("brep/buc60281_2.rle", "w2")),
        Step::Topo(Topo::Restore("brep/buc60281_3.rle", "w3")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 476995.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_w8,
    "tests/thrusection/not_solids/W8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/buc60318.rle", "w")),
        Step::Topo(Topo::Explode("w", "W")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w_1", "w_2", "w_3", "w_4", "w_5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 254109.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_a1,
    "tests/thrusection/not_solids_locked/A1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D1", "w1")),
        Step::Topo(Topo::Restore("brep/D1", "w2")),
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
        Step::Check(Check::Props(Prop::Area, "result", 200.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_locked_b1,
    "tests/thrusection/not_solids_locked/B1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D2", "w1")),
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
    thrusection_not_solids_locked_c1,
    "tests/thrusection/not_solids_locked/C1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D4", "w1")),
        Step::Topo(Topo::Restore("brep/D1", "w2")),
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
        Step::Check(Check::Props(Prop::Area, "result", 200.0, 0.01, None)),
    ]
);
