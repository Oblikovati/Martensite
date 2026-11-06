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
    bugs_modalg_6_bug26736_1,
    "tests/bugs/modalg_6/bug26736_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26736"),
        Step::Puts("============"),
        Step::View(ViewStep::SmallView),
        Step::Topo(Topo::Restore("brep/bug26736_arc1.brep", "a")),
        Step::Profile(Profile::Wire("ww", &["a"])),
        Step::View(ViewStep::Donly(&["ww"])),
        Step::View(ViewStep::Fit),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "result",
            base: "ww",
            count: 10,
            step: 8.0,
            approx: false,
            join: 'i',
            altitude: 0.0,
            open: false
        })),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26736_2,
    "tests/bugs/modalg_6/bug26736_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26736"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("c1", &[7.0, 0.0, 0.0, 5.0])),
        Step::Curve(Curve::Circle(
            "c2",
            &[-7.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 5.0]
        )),
        Step::More(More::TrimCurve(
            "c1",
            "c1",
            [1.5707963267948966, 4.71238898038469]
        )),
        Step::More(More::TrimCurve(
            "c2",
            "c2",
            [3.141592653589793, 6.283185307179586]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Polyline(
            "pp1",
            &[
                -7.0, 5.0, 0.0, -7.0, 30.0, 0.0, 7.0, 30.0, 0.0, 7.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "pp2",
            &[
                7.0, -5.0, 0.0, 7.0, -30.0, 0.0, -7.0, -30.0, 0.0, -7.0, -5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("ww", &["pp1", "e1", "pp2", "e2"])),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["ww"])),
        Step::View(ViewStep::Fit),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "result",
            base: "ww",
            count: 1,
            step: -3.0,
            approx: false,
            join: 'i',
            altitude: 0.0,
            open: false
        })),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_6_bug27386_5,
    "tests/bugs/modalg_6/bug27386_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("27386"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("cc", &[0.0, 0.0, 0.0, 100.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 100.0, 0.0, 1.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Polyline(
            "pr2",
            &[
                0.0, -90.0, 0.0, 0.0, -100.0, 10.0, 0.0, -110.0, 0.0, 0.0, -100.0, -10.0, 0.0,
                -90.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("sp", "cc", None)),
        Step::Profile(Profile::Wire("sp", &["sp"])),
        Step::Profile(Profile::Edge("pr1", "c1", None)),
        Step::Profile(Profile::Wire("pr1", &["pr1"])),
        Step::View(ViewStep::Donly(&["sp", "pr1", "pr2"])),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "q",
            mode: '-',
            solid: false
        })),
        Step::Topo(Topo::ExplodeChildren("pr1")),
        Step::Topo(Topo::ExplodeChildren("pr2")),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1_1",
            "sweep_hist",
            "pr1_1"
        ))),
        Step::Check(Check::Counts("r1_1", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r1_1", 37077.0, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated(
            "r2_1",
            "sweep_hist",
            "pr2_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_2",
            "sweep_hist",
            "pr2_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_3",
            "sweep_hist",
            "pr2_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_4",
            "sweep_hist",
            "pr2_4"
        ))),
        Step::Check(Check::Props(Prop::Area, "r2_1", 8740.38, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2_2", 9798.12, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2_3", 9798.12, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2_4", 8740.38, 0.01, None)),
        Step::Topo(Topo::Explode("pr1", "v")),
        Step::Topo(Topo::Explode("pr2", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "r1_1",
            "sweep_hist",
            "pr1_1"
        ))),
        Step::Check(Check::Props(Prop::Length, "r1_1", 628.319, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated(
            "r2_1",
            "sweep_hist",
            "pr2_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_2",
            "sweep_hist",
            "pr2_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_3",
            "sweep_hist",
            "pr2_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_4",
            "sweep_hist",
            "pr2_4"
        ))),
        Step::Check(Check::Props(Prop::Length, "r2_1", 565.487, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2_2", 628.319, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2_3", 691.15, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2_4", 628.319, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug27386_6,
    "tests/bugs/modalg_6/bug27386_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("27386"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("cc", &[0.0, 0.0, 0.0, 100.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 100.0, 0.0, 1.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Polyline(
            "pr2",
            &[
                0.0, -90.0, 0.0, 0.0, -100.0, 10.0, 0.0, -110.0, 0.0, 0.0, -100.0, -10.0, 0.0,
                -90.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("sp", "cc", None)),
        Step::Profile(Profile::Wire("sp", &["sp"])),
        Step::Profile(Profile::Edge("pr1", "c1", None)),
        Step::Profile(Profile::Wire("pr1", &["pr1"])),
        Step::Topo(Topo::Explode("sp", "v")),
        Step::View(ViewStep::Donly(&["sp", "pr1", "pr2", "sp_1"])),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "sp_1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "q",
            mode: '-',
            solid: false
        })),
        Step::Topo(Topo::ExplodeChildren("pr1")),
        Step::Topo(Topo::ExplodeChildren("pr2")),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1_1",
            "sweep_hist",
            "pr1_1"
        ))),
        Step::Check(Check::Counts("r1_1", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r1_1", 35117.3, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated(
            "r2_1",
            "sweep_hist",
            "pr2_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_2",
            "sweep_hist",
            "pr2_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_3",
            "sweep_hist",
            "pr2_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_4",
            "sweep_hist",
            "pr2_4"
        ))),
        Step::Check(Check::Props(Prop::Area, "r2_1", 8183.89, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2_2", 9374.75, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2_3", 9374.75, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2_4", 8183.89, 0.01, None)),
        Step::Topo(Topo::Explode("pr1", "v")),
        Step::Topo(Topo::Explode("pr2", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "r1_1",
            "sweep_hist",
            "pr1_1"
        ))),
        Step::Check(Check::Props(Prop::Length, "r1_1", 628.319, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated(
            "r2_1",
            "sweep_hist",
            "pr2_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_2",
            "sweep_hist",
            "pr2_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_3",
            "sweep_hist",
            "pr2_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2_4",
            "sweep_hist",
            "pr2_4"
        ))),
        Step::Check(Check::Props(Prop::Length, "r2_1", 573.265, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2_2", 629.072, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2_3", 684.885, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2_4", 629.072, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "sp_1"
        ))),
        Step::Check(Check::Counts(
            "r3",
            &[("VERTEX", 1), ("EDGE", 4), ("FACE", 0)]
        )),
        Step::Run(crate::one_off::punctual_section),
    ]
);

offset_case!(
    bugs_modalg_6_bug27386_7,
    "tests/bugs/modalg_6/bug27386_7",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("27386"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "sp")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "pr")),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "q",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::ExplodeChildren("pr")),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "pr_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "pr_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "pr_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "pr_4"
        ))),
        Step::Check(Check::Counts("r1", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r1", 256109.0, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r2", 277227.0, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r3", 256109.0, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r4", 233971.0, 0.01, None)),
        Step::Topo(Topo::Explode("pr", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "pr_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "pr_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "pr_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "pr_4"
        ))),
        Step::Check(Check::Props(Prop::Length, "r1", 1169.86, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r2", 1386.13, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r3", 1386.13, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r4", 1169.86, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug27386_8,
    "tests/bugs/modalg_6/bug27386_8",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("27386"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "sp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 10.0, 10.0, 0.0, 10.0, 10.0,
                0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "pr",
            &[
                0.0, -2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0, 2.0, 0.0, -2.0, 2.0, 0.0, -2.0, -2.0
            ]
        )),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "q",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::ExplodeChildren("pr")),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "pr_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "pr_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "pr_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "pr_4"
        ))),
        Step::Check(Check::Counts("r1", &[("FACE", 12)])),
        Step::Check(Check::Props(Prop::Area, "r1", 275.124, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 9)])),
        Step::Check(Check::Props(Prop::Area, "r2", 189.425, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 9)])),
        Step::Check(Check::Props(Prop::Area, "r3", 189.425, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 12)])),
        Step::Check(Check::Props(Prop::Area, "r4", 275.124, 0.01, None)),
        Step::Topo(Topo::Explode("pr", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "pr_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "pr_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "pr_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "pr_4"
        ))),
        Step::Check(Check::Counts("r1", &[("EDGE", 12)])),
        Step::Check(Check::Props(Prop::Length, "r1", 78.8496, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("EDGE", 9)])),
        Step::Check(Check::Props(Prop::Length, "r2", 57.4248, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("EDGE", 6)])),
        Step::Check(Check::Props(Prop::Length, "r3", 36.0, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("EDGE", 9)])),
        Step::Check(Check::Props(Prop::Length, "r4", 57.4248, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug27674,
    "tests/bugs/modalg_6/bug27674",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("27674"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug27674_CircleLineWires.brep", "a")),
        Step::Topo(Topo::ExplodeChildren("a")),
        Step::Profile(Profile::Wire("w", &["a_1", "a_2"])),
        Step::Topo(Topo::ExplodeChildren("w")),
        Step::Profile(Profile::Wire("a1", &["w_1"])),
        Step::Profile(Profile::Wire("a2", &["w_2"])),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["a1", "a2"]
        }),
        Step::Check(Check::Shape("r")),
        Step::Check(Check::Counts("r", &[("VERTEX", 2), ("EDGE", 4)])),
        Step::Puts("Check that edges degenerated and has no 3d curve..."),
        Step::Topo(Topo::Explode("r", "E")),
        Step::Run(crate::one_off::degenerated_edges),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_6_bug27822,
    "tests/bugs/modalg_6/bug27822",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("27822"),
        Step::Puts("========"),
        Step::Curve(Curve::Circle("c1", &[0.0, 100.0, 80.0, 30.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, 30.0, 130.0, 30.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 1.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e3", "v1", "v2")),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Wire("w2", &["e2"])),
        Step::Profile(Profile::Wire("w3", &["e3"])),
        Step::Shell(PipeShell::Start("w3")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w2",
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
        Step::Let("answer_6_buildsweep", Value::SweepOutcome),
        Step::If(
            Cond::Holds("answer_6_buildsweep", "Buildsweep : Not Done"),
            &[Step::Puts("27822 is fixed.")],
            &[Step::Puts("ERROR: 27822 does not fixed correctly.")]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug28189_8,
    "tests/bugs/modalg_6/bug28189_8",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("28189"),
        Step::Puts("========"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[5.0, 5.0, 5.0, 10.0, 10.0, 10.0])),
        Step::More(More::EmptyShape("c1", "Cs")),
        Step::More(More::EmptyShape("c2", "Cs")),
        Step::Topo(Topo::Add("b1", "c1")),
        Step::Topo(Topo::Add("b2", "c2")),
        Step::Bool(Bool::Fuse("result", "c1", "c2")),
        Step::Check(Check::Counts("result", &[("COMPSOLID", 1), ("SOLID", 1)])),
    ]
);

offset_case!(
    bugs_modalg_6_bug28189_9,
    "tests/bugs/modalg_6/bug28189_9",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("28189"),
        Step::Puts("========"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[5.0, 5.0, 5.0, 10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b3", &[20.0, 0.0, 0.0, 10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b4", &[25.0, 5.0, 5.0, 10.0, 10.0, 10.0])),
        Step::More(More::EmptyShape("c1", "Cs")),
        Step::More(More::EmptyShape("c2", "Cs")),
        Step::Topo(Topo::Add("b1", "c1")),
        Step::Topo(Topo::Add("b2", "c2")),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["c1", "b3"]))),
        Step::More(More::General(General::AddTools(&["c2", "b4"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Operation("result", 1))),
        Step::Check(Check::Counts("result", &[("SOLID", 2), ("COMPSOLID", 1)])),
        Step::Topo(Topo::ExplodeChildren("result")),
        Step::If(
            Cond::KindHolds("result_1", "COMPSOLID"),
            &[Step::If(
                Cond::KindLacks("result_2", "SOLID"),
                &[Step::Puts("Error: Incorrect shape types in the result")],
                &[]
            )],
            &[Step::If(
                Cond::KindHolds("result_1", "SOLID"),
                &[Step::If(
                    Cond::KindLacks("result_2", "COMPSOLID"),
                    &[Step::Puts("Error: Incorrect shape types in the result")],
                    &[]
                )],
                &[Step::Puts("Error: Incorrect shape types in the result")]
            )]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug28284_3,
    "tests/bugs/modalg_6/bug28284_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("28284"),
        Step::Puts("========"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b1", "sh")),
        Step::More(More::EmptyShape("b2", "So")),
        Step::Topo(Topo::Add("b1_1", "b2")),
        Step::Bool(Bool::Prepare("b1", "b2")),
        Step::Bool(Bool::TwoStep("rcom", BopKind::Common)),
        Step::Check(Check::Counts("rcom", &[("SOLID", 1)])),
        Step::Bool(Bool::TwoStep("rfuse", BopKind::Fuse)),
        Step::Check(Check::Counts("rfuse", &[("SOLID", 1)])),
        Step::Bool(Bool::TwoStep("rcut", BopKind::Cut)),
        Step::Check(Check::Counts("rcut", &[("SOLID", 0)])),
        Step::Bool(Bool::TwoStep("rtuc", BopKind::Tuc)),
        Step::Check(Check::Counts("rcut", &[("SOLID", 0)])),
    ]
);

offset_case!(
    bugs_modalg_6_bug28468_2,
    "tests/bugs/modalg_6/bug28468_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("28468"),
        Step::Puts("============"),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                3.0, 100.0, 0.0, 0.0, 1.0, 0.0, 100.0, 0.0, 3.0, -100.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Curve(Curve::Bezier(
            "bc2",
            &[
                3.0, -100.0, 0.0, 0.0, 1.0, 0.0, -100.0, 0.0, 3.0, 100.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Edge("e2", "bc2", None)),
        Step::Profile(Profile::Wire("sp", &["e1", "e2"])),
        Step::Profile(Profile::Polyline(
            "pr1",
            &[
                110.0, 0.0, 10.0, 90.0, 0.0, 10.0, 90.0, 0.0, -10.0, 110.0, 0.0, -10.0, 110.0, 0.0,
                10.0
            ]
        )),
        Step::Curve(Curve::Circle(
            "pr2",
            &[-100.0, 0.0, 0.0, 0.0, 1.0, 0.0, 10.0]
        )),
        Step::Profile(Profile::Edge("pr2", "pr2", None)),
        Step::Profile(Profile::Wire("pr2", &["pr2"])),
        Step::View(ViewStep::Donly(&["sp", "pr1", "pr2"])),
        Step::Topo(Topo::Explode("sp", "v")),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr1",
            vertex: Some("sp_1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr2",
            vertex: Some("sp_2"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Shape("r")),
        Step::Check(Check::Counts(
            "r",
            &[
                ("SHELL", 1),
                ("FACE", 10),
                ("WIRE", 10),
                ("EDGE", 25),
                ("VERTEX", 15)
            ]
        )),
        Step::Check(Check::Props(Prop::Area, "r", 28569.4, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug28661_2,
    "tests/bugs/modalg_6/bug28661_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("28661"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("sp", &[0.0, 0.0, 0.0, 6.0])),
        Step::Profile(Profile::Edge("sp", "sp", None)),
        Step::Profile(Profile::Wire("sp", &["sp"])),
        Step::Curve(Curve::Circle("pr", &[0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 10.0])),
        Step::More(More::TrimCurve("pr", "pr", [0.0, 3.141592653589793])),
        Step::Profile(Profile::Edge("pr", "pr", None)),
        Step::Profile(Profile::Wire("pr", &["pr"])),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: '-',
            solid: false
        })),
        Step::Topo(Topo::Explode("r", "f")),
        Step::More(More::SurfaceOfFace("s1", "r_1")),
        Step::Run(crate::one_off::spherical_s1),
        Step::Run(crate::one_off::sweep_error),
    ]
);

offset_case!(
    bugs_modalg_6_bug6768,
    "tests/bugs/modalg_6/bug6768",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("6768"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug6768.brep", "co")),
        Step::Topo(Topo::ExplodeChildren("co")),
        Step::Puts(
            "
*** make offset of wire rco_1
"
        ),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "rco_1",
            base: "co_1",
            count: 1,
            step: -1.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Let("nbv", Value::AloneVertices("rco_1_1")),
        Step::If(
            Cond::Differs("nbv", 0.0),
            &[Step::Puts("Error : Section is incorrect")],
            &[Step::Puts("Section is correct")]
        ),
        Step::Check(Check::Props(Prop::Length, "rco_1_1", 577.397, 0.01, None)),
        Step::Check(Check::Counts("rco_1_1", &[("VERTEX", 99)])),
        Step::Puts(
            "
*** make offset of wire rco_2
"
        ),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "rco_2",
            base: "co_2",
            count: 1,
            step: -1.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Let("nbv", Value::AloneVertices("rco_2_1")),
        Step::If(
            Cond::Differs("nbv", 0.0),
            &[Step::Puts("Error : Section is incorrect")],
            &[Step::Puts("Section is correct")]
        ),
        Step::Check(Check::Props(Prop::Length, "rco_2_1", 591.334, 0.01, None)),
        Step::Check(Check::Counts("rco_2_1", &[("VERTEX", 80)])),
        Step::Puts(
            "
*** make offset of wire rco_3
"
        ),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "rco_3",
            base: "co_3",
            count: 1,
            step: -1.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Let("nbv", Value::AloneVertices("rco_3_1")),
        Step::If(
            Cond::Differs("nbv", 0.0),
            &[Step::Puts("Error : Section is incorrect")],
            &[Step::Puts("Section is correct")]
        ),
        Step::Check(Check::Props(Prop::Length, "rco_3_1", 550.648, 0.01, None)),
        Step::Check(Check::Counts("rco_3_1", &[("VERTEX", 111)])),
        Step::Puts(
            "
*** make offset of wire rco_4
"
        ),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "rco_4",
            base: "co_4",
            count: 1,
            step: -1.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Let("nbv", Value::AloneVertices("rco_4_1")),
        Step::If(
            Cond::Differs("nbv", 0.0),
            &[Step::Puts("Error : Section is incorrect")],
            &[Step::Puts("Section is correct")]
        ),
        Step::Check(Check::Props(Prop::Length, "rco_4_1", 545.219, 0.01, None)),
        Step::Check(Check::Counts("rco_4_1", &[("VERTEX", 290)])),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);
