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
    nproject_base_a1,
    "tests/nproject/base/A1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/projortho001.brep", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Compound(&["p_2", "p_3"], "c")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["c"],
            surface: "p_1",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 222.5, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a2,
    "tests/nproject/base/A2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CNP001_projoiei.rle", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Compound(&["p_2", "p_3"], "s")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["s"],
            surface: "p_1",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 222.5, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a3,
    "tests/nproject/base/A3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/projortho003.brep", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["p_1"],
            surface: "p_2",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 581.299, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a4,
    "tests/nproject/base/A4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/projortho004.brep", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["p_1"],
            surface: "p_2",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 300.0, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a5,
    "tests/nproject/base/A5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/projortho005.brep", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Compound(&["p_1", "p_2"], "c")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["p_3"],
            surface: "c",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 540.983, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a6,
    "tests/nproject/base/A6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/projortho006.brep", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Compound(
            &[
                "p_1", "p_2", "p_3", "p_4", "p_5", "p_6", "p_7", "p_8", "p_9", "p_10", "p_11",
                "p_12", "p_13"
            ],
            "c"
        )),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["c"],
            surface: "p_14",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 90.802, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a7,
    "tests/nproject/base/A7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CNP001_projoien.rle", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Compound(&["p_2", "p_3"], "c")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["c"],
            surface: "p_1",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 95.8118, 0.01, None)),
    ]
);

offset_case!(
    nproject_base_a8,
    "tests/nproject/base/A8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/projortho008.brep", "p")),
        Step::Topo(Topo::ExplodeChildren("p")),
        Step::More(More::Compound(&["p_4", "p_5"], "c1")),
        Step::More(More::Compound(
            &["p_1", "p_2", "p_6", "p_7", "p_3", "p_8"],
            "c2"
        )),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["c2"],
            surface: "c1",
            unlimited: false,
            max_distance: None,
            tolerance: 0.001,
            continuity: 2,
            max_degree: 14,
            max_segments: 10
        })),
        Step::Check(Check::Props(Prop::Length, "result", 561.837, 0.01, None)),
    ]
);
