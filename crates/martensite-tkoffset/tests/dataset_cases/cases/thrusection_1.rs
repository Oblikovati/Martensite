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
    thrusection_bugs_bug24997,
    "tests/thrusection/bugs/bug24997",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("REQUIRED ALL: Error : The command cannot be built."),
        Step::Puts("========"),
        Step::Puts("0024997: S I G S E G V in BRepOffsetAPIThruSections"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug24997_loft3-draw-Loft-00-section.brep",
            "Loft-00-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug24997_loft3-draw-Loft-01-section.brep",
            "Loft-01-section"
        )),
        Step::Loft(Loft {
            result: "Loft",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["Loft-00-section", "Loft-01-section"]
        }),
    ]
);

offset_case!(
    thrusection_not_solids_a1,
    "tests/thrusection/not_solids/A1",
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
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 200.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a2,
    "tests/thrusection/not_solids/A2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D1", "w1")),
        Step::Topo(Topo::Restore("brep/D2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 211.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a3,
    "tests/thrusection/not_solids/A3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D1", "w1")),
        Step::Topo(Topo::Restore("brep/D3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 250.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a4,
    "tests/thrusection/not_solids/A4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D1", "w1")),
        Step::Topo(Topo::Restore("brep/D4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 200.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a5,
    "tests/thrusection/not_solids/A5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D1", "w1")),
        Step::Topo(Topo::Restore("brep/D5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 211.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a6,
    "tests/thrusection/not_solids/A6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D1", "w1")),
        Step::Topo(Topo::Restore("brep/D6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 250.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a7,
    "tests/thrusection/not_solids/A7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D2", "w1")),
        Step::Topo(Topo::Restore("brep/D1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 161.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a8,
    "tests/thrusection/not_solids/A8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D2", "w1")),
        Step::Topo(Topo::Restore("brep/D2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 173.205, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_a9,
    "tests/thrusection/not_solids/A9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D2", "w1")),
        Step::Topo(Topo::Restore("brep/D3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 199.103, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b1,
    "tests/thrusection/not_solids/B1",
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
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 161.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b2,
    "tests/thrusection/not_solids/B2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D2", "w1")),
        Step::Topo(Topo::Restore("brep/D5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 173.205, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b3,
    "tests/thrusection/not_solids/B3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D2", "w1")),
        Step::Topo(Topo::Restore("brep/D6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 199.103, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b4,
    "tests/thrusection/not_solids/B4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D3", "w1")),
        Step::Topo(Topo::Restore("brep/D1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 250.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b5,
    "tests/thrusection/not_solids/B5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D3", "w1")),
        Step::Topo(Topo::Restore("brep/D2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 274.103, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b6,
    "tests/thrusection/not_solids/B6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D3", "w1")),
        Step::Topo(Topo::Restore("brep/D3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 300.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b7,
    "tests/thrusection/not_solids/B7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D3", "w1")),
        Step::Topo(Topo::Restore("brep/D4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 250.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b8,
    "tests/thrusection/not_solids/B8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D3", "w1")),
        Step::Topo(Topo::Restore("brep/D5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 274.103, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_b9,
    "tests/thrusection/not_solids/B9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D3", "w1")),
        Step::Topo(Topo::Restore("brep/D6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 300.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c1,
    "tests/thrusection/not_solids/C1",
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
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 200.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c2,
    "tests/thrusection/not_solids/C2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D4", "w1")),
        Step::Topo(Topo::Restore("brep/D2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 211.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c3,
    "tests/thrusection/not_solids/C3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D4", "w1")),
        Step::Topo(Topo::Restore("brep/D3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 250.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c4,
    "tests/thrusection/not_solids/C4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D4", "w1")),
        Step::Topo(Topo::Restore("brep/D4", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 200.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c5,
    "tests/thrusection/not_solids/C5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D4", "w1")),
        Step::Topo(Topo::Restore("brep/D5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 211.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c6,
    "tests/thrusection/not_solids/C6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D4", "w1")),
        Step::Topo(Topo::Restore("brep/D6", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 250.0, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c7,
    "tests/thrusection/not_solids/C7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D5", "w1")),
        Step::Topo(Topo::Restore("brep/D1", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 161.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c8,
    "tests/thrusection/not_solids/C8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D5", "w1")),
        Step::Topo(Topo::Restore("brep/D2", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 173.205, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_c9,
    "tests/thrusection/not_solids/C9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D5", "w1")),
        Step::Topo(Topo::Restore("brep/D3", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 199.103, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_d1,
    "tests/thrusection/not_solids/D1",
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
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 161.603, 0.01, None)),
    ]
);

offset_case!(
    thrusection_not_solids_d2,
    "tests/thrusection/not_solids/D2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/D5", "w1")),
        Step::Topo(Topo::Restore("brep/D5", "w2")),
        Step::Place(Place::Translate(&["w2"], [0.0, 20.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 173.205, 0.01, None)),
    ]
);
