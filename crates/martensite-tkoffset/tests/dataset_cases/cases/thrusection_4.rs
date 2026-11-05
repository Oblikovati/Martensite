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
    thrusection_not_solids_j9,
    "tests/thrusection/not_solids/J9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C6", "w1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 1516.01, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_k1,
    "tests/thrusection/not_solids/K1",
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
    thrusection_not_solids_k2,
    "tests/thrusection/not_solids/K2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k3,
    "tests/thrusection/not_solids/K3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k4,
    "tests/thrusection/not_solids/K4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k5,
    "tests/thrusection/not_solids/K5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k6,
    "tests/thrusection/not_solids/K6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k7,
    "tests/thrusection/not_solids/K7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k8,
    "tests/thrusection/not_solids/K8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_k9,
    "tests/thrusection/not_solids/K9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C7", "w1")),
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
    thrusection_not_solids_l1,
    "tests/thrusection/not_solids/L1",
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
    thrusection_not_solids_l2,
    "tests/thrusection/not_solids/L2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l3,
    "tests/thrusection/not_solids/L3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l4,
    "tests/thrusection/not_solids/L4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l5,
    "tests/thrusection/not_solids/L5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l6,
    "tests/thrusection/not_solids/L6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l7,
    "tests/thrusection/not_solids/L7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l8,
    "tests/thrusection/not_solids/L8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_l9,
    "tests/thrusection/not_solids/L9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C8", "w1")),
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
    thrusection_not_solids_m1,
    "tests/thrusection/not_solids/M1",
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
    thrusection_not_solids_m2,
    "tests/thrusection/not_solids/M2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m3,
    "tests/thrusection/not_solids/M3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m4,
    "tests/thrusection/not_solids/M4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m5,
    "tests/thrusection/not_solids/M5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m6,
    "tests/thrusection/not_solids/M6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m7,
    "tests/thrusection/not_solids/M7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m8,
    "tests/thrusection/not_solids/M8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_m9,
    "tests/thrusection/not_solids/M9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C9", "w1")),
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
    thrusection_not_solids_n1,
    "tests/thrusection/not_solids/N1",
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
    thrusection_not_solids_n2,
    "tests/thrusection/not_solids/N2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/C10", "w1")),
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
