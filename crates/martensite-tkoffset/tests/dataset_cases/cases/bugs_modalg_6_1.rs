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
    bugs_modalg_6_bug10234,
    "tests/bugs/modalg_6/bug10234",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("10234"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug10234a.brep", "pr")),
        Step::Topo(Topo::Restore("brep/bug10234b.brep", "sp")),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 45.9872, 0.01, None)),
        Step::Check(Check::TypedCountsNamed(
            "result",
            &[
                ("VERTEX", 7),
                ("EDGE", 8),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 20)
            ],
            "Result obtained by Boolean cut operation"
        )),
        Step::Let("tolres", Value::CheckedMaxTolerance("result")),
        Step::If(
            Cond::Above("tolres", 5e-07),
            &[Step::Puts("Error: bad tolerance of result")],
            &[]
        ),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_6_bug24932,
    "tests/bugs/modalg_6/bug24932",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("24932"),
        Step::Puts("========"),
        Step::Profile(Profile::Vertex("v0", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v1", [1.0, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v0", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Shell(PipeShell::Start("w1")),
        Step::More(More::EmptyShape("solid1", "So")),
        Step::CatchKept(
            "msg",
            &Step::Shell(PipeShell::Add(SweepSection {
                section: "solid1",
                vertex: None,
                translate: false,
                rotate: false,
                law: &[]
            }))
        ),
        Step::If(
            Cond::Lacks("msg", "is not a wire"),
            &[Step::Puts(
                "Error: addsweep did not complaint about wrong argument - solid"
            )],
            &[Step::Puts("OK: addsweep noted wrong argument - solid")]
        ),
        Step::Shell(PipeShell::Start("w1")),
        Step::CatchKept(
            "msg",
            &Step::Shell(PipeShell::Add(SweepSection {
                section: "emptyshape",
                vertex: None,
                translate: false,
                rotate: false,
                law: &[]
            }))
        ),
        Step::If(
            Cond::Lacks("msg", "is not a wire"),
            &[Step::Puts(
                "Error: addsweep did not complaint about wrong argument - empty shape"
            )],
            &[Step::Puts(
                "OK: addsweep noted wrong argument - empty shape"
            )]
        ),
        Step::Shell(PipeShell::Start("w1")),
        Step::CatchKept(
            "msg",
            &Step::Shell(PipeShell::Add(SweepSection {
                section: "undefined",
                vertex: None,
                translate: false,
                rotate: false,
                law: &[]
            }))
        ),
        Step::If(
            Cond::Lacks("msg", "is not a wire"),
            &[Step::Puts(
                "Error: addsweep did not complaint about wrong argument - undefined variable"
            )],
            &[Step::Puts(
                "OK: addsweep noted wrong argument - undefined variable"
            )]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug25138,
    "tests/bugs/modalg_6/bug25138",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25138"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug25138a.brep", "s")),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Topo(Topo::Restore("brep/bug25138b.brep", "p")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 112902.0, 0.01, None)),
        Step::Check(Check::TypedCountsNamed(
            "result",
            &[
                ("VERTEX", 6),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 22)
            ],
            "Result obtained by sweep operation"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_6_bug25220,
    "tests/bugs/modalg_6/bug25220",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25220"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug25220.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "a",
            count: 1,
            step: -5.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Check(Check::Props(Prop::Length, "r_1", 939.882, 0.01, None)),
        Step::Let("info", Value::Section("r_1")),
        Step::If(
            Cond::Lacks("info", "nb alone Vertices : 0"),
            &[Step::Puts("Error: Result of offset is incorrect")],
            &[Step::Puts("OK : Result of offset is correct")]
        ),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_1,
    "tests/bugs/modalg_6/bug26150_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, -10.0, 20.0, -10.0, 7.0, 20.0, 10.0, 8.0, 20.0, 0.0, -10.0, 20.0
            ]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Topo(Topo::ExplodeChildren("w3")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r8", "hist", "w3_1"))),
        Step::Check(Check::Counts("r1", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r1", 668.19, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 6)])),
        Step::Check(Check::Props(Prop::Area, "r2", 650.955, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r3", 590.746, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r4", 650.315, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r5", 835.461, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 6)])),
        Step::Check(Check::Props(Prop::Area, "r6", 852.53, 0.01, None)),
        Step::Check(Check::Counts("r7", &[("FACE", 6)])),
        Step::Check(Check::Props(Prop::Area, "r7", 872.216, 0.01, None)),
        Step::Check(Check::Counts("r8", &[("FACE", 16)])),
        Step::Check(Check::Props(Prop::Area, "r8", 2560.21, 0.01, None)),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Topo(Topo::Explode("w3", "v")),
        Step::More(More::History(HistoryStep::Generated("r9", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r10", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r11", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r12", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r16", "hist", "w3_1"))),
        Step::Check(Check::Counts("r9", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r9", 42.6998, 0.01, None)),
        Step::Check(Check::Counts("r10", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r10", 42.8761, 0.01, None)),
        Step::Check(Check::Counts("r11", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r11", 40.5019, 0.01, None)),
        Step::Check(Check::Counts("r12", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r12", 40.7504, 0.01, None)),
        Step::Check(Check::Counts("r13", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r13", 40.0002, 0.01, None)),
        Step::Check(Check::Counts("r14", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r14", 40.2389, 0.01, None)),
        Step::Check(Check::Counts("r15", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r15", 40.2965, 0.01, None)),
        Step::Check(Check::Counts("r16", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r16", 40.7511, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_13,
    "tests/bugs/modalg_6/bug26150_13",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, 10.0, 0.0, -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 8.0, 20.0, 0.0, -10.0, 20.0, -10.0, 7.0, 20.0]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::More(More::TrimCurve(
            "w3",
            "w3",
            [3.141592653589793, 6.283185307179586]
        )),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Topo(Topo::ExplodeChildren("w3")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w3_1"))),
        Step::Check(Check::Counts("r1", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r1", 577.142, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r2", 541.715, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r3", 578.464, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r4", 866.24, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r5", 831.08, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r6", 1697.32, 0.01, None)),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Topo(Topo::Explode("w3", "v")),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r8", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r9", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r10", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r11", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r12", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w3_1"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w3_2"))),
        Step::Check(Check::Counts("r7", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r7", 41.4134, 0.01, None)),
        Step::Check(Check::Counts("r8", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r8", 42.3039, 0.01, None)),
        Step::Check(Check::Counts("r9", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r9", 42.5986, 0.01, None)),
        Step::Check(Check::Counts("r10", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r10", 41.6404, 0.01, None)),
        Step::Check(Check::Counts("r11", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r11", 41.6404, 0.01, None)),
        Step::Check(Check::Counts("r12", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r12", 40.0133, 0.01, None)),
        Step::Check(Check::Counts("r13", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r13", 41.4134, 0.01, None)),
        Step::Check(Check::Counts("r14", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r14", 41.4134, 0.01, None)),
        Step::Check(Check::Counts("r15", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r15", 41.6404, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_14,
    "tests/bugs/modalg_6/bug26150_14",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, 10.0, 0.0, -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 8.0, 20.0, 0.0, -10.0, 20.0, -10.0, 7.0, 20.0]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::More(More::TrimCurve(
            "w3",
            "w3",
            [3.141592653589793, 6.283185307179586]
        )),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Topo(Topo::ExplodeChildren("w3")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w3_1"))),
        Step::Check(Check::Counts("r1", &[("FACE", 1)])),
        Step::Check(Check::Props(Prop::Area, "r1", 574.919, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r2", 543.935, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 1)])),
        Step::Check(Check::Props(Prop::Area, "r3", 576.17, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r4", 865.397, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r5", 829.628, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r6", 1695.02, 0.01, None)),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Topo(Topo::Explode("w3", "v")),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r8", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r9", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r10", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r11", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r12", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w3_1"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w3_2"))),
        Step::Check(Check::Props(Prop::Length, "r7", 41.4735, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r8", 42.8862, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r9", 43.2561, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r10", 41.7738, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r11", 41.7738, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r12", 40.0173, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r13", 41.4735, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r14", 41.4735, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r15", 41.7738, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_17,
    "tests/bugs/modalg_6/bug26150_17",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, 10.0, 0.0, -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 8.0, 20.0, 0.0, -10.0, 20.0, -10.0, 7.0, 20.0]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::More(More::TrimCurve(
            "w3",
            "w3",
            [3.141592653589793, 6.283185307179586]
        )),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -30.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 70.0])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Topo(Topo::ExplodeChildren("w3")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "w2", "w3", "v2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w3_1"))),
        Step::Check(Check::Counts("r1", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r1", 1058.95, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r2", 1023.52, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r3", 1060.27, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r4", 1604.51, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r5", 1538.22, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 16)])),
        Step::Check(Check::Props(Prop::Area, "r6", 3142.73, 0.01, None)),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Topo(Topo::Explode("w3", "v")),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r8", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r9", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r10", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r11", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r12", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w3_1"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w3_2"))),
        Step::More(More::History(HistoryStep::Generated("r16", "hist", "v1"))),
        Step::More(More::History(HistoryStep::Generated("r17", "hist", "v2"))),
        Step::Check(Check::Counts("r7", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r7", 106.202, 0.01, None)),
        Step::Check(Check::Counts("r8", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r8", 107.093, 0.01, None)),
        Step::Check(Check::Counts("r9", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r9", 107.388, 0.01, None)),
        Step::Check(Check::Counts("r10", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r10", 106.429, 0.01, None)),
        Step::Check(Check::Counts("r11", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r11", 106.429, 0.01, None)),
        Step::Check(Check::Counts("r12", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r12", 103.265, 0.01, None)),
        Step::Check(Check::Counts("r13", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r13", 106.202, 0.01, None)),
        Step::Check(Check::Counts("r14", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r14", 106.202, 0.01, None)),
        Step::Check(Check::Counts("r15", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r15", 106.429, 0.01, None)),
        Step::Check(Check::Counts("r16", &[("EDGE", 20)])),
        Step::Check(Check::Props(Prop::Length, "r16", 530.378, 0.01, None)),
        Step::Check(Check::Counts("r17", &[("EDGE", 20)])),
        Step::Check(Check::Props(Prop::Length, "r17", 530.378, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_18,
    "tests/bugs/modalg_6/bug26150_18",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, 10.0, 0.0, -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 8.0, 20.0, 0.0, -10.0, 20.0, -10.0, 7.0, 20.0]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::More(More::TrimCurve(
            "w3",
            "w3",
            [3.141592653589793, 6.283185307179586]
        )),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -30.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 70.0])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Topo(Topo::ExplodeChildren("w3")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "w2", "w3", "v2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w3_1"))),
        Step::Check(Check::Counts("r1", &[("FACE", 1)])),
        Step::Check(Check::Props(Prop::Area, "r1", 1409.51, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r2", 1703.44, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 1)])),
        Step::Check(Check::Props(Prop::Area, "r3", 1401.17, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r4", 2316.52, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r5", 2197.61, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r6", 4514.13, 0.01, None)),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Topo(Topo::Explode("w3", "v")),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r8", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r9", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r10", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r11", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r12", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w3_1"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w3_2"))),
        Step::More(More::History(HistoryStep::Generated("r16", "hist", "v1"))),
        Step::More(More::History(HistoryStep::Generated("r17", "hist", "v2"))),
        Step::Check(Check::Props(Prop::Length, "r7", 109.077, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r8", 122.394, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r9", 124.029, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r10", 109.865, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r11", 109.865, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r12", 104.896, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r13", 109.077, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r14", 109.077, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r15", 109.865, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r16", 570.261, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "r17", 570.261, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_2,
    "tests/bugs/modalg_6/bug26150_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, -10.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [10.0, 8.0, 20.0])),
        Step::Profile(Profile::Vertex("v3", [-10.0, 7.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Topo(Topo::Orient("e1", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Topo(Topo::Orient("e2", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Topo(Topo::Orient("e3", 'R')),
        Step::Profile(Profile::Wire("w2", &["e1", "e2", "e3"])),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w2_3"))),
        Step::Check(Check::Counts("r5", &[("FACE", 6)])),
        Step::Check(Check::Props(Prop::Area, "r5", 872.216, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 6)])),
        Step::Check(Check::Props(Prop::Area, "r6", 852.53, 0.01, None)),
        Step::Check(Check::Counts("r7", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r7", 835.461, 0.01, None)),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w2_3"))),
        Step::Check(Check::Counts("r13", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r13", 40.0002, 0.01, None)),
        Step::Check(Check::Counts("r14", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r14", 40.2965, 0.01, None)),
        Step::Check(Check::Counts("r15", &[("EDGE", 2)])),
        Step::Check(Check::Props(Prop::Length, "r15", 40.2389, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_3,
    "tests/bugs/modalg_6/bug26150_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, -10.0, 20.0, -10.0, 7.0, 20.0, 10.0, 8.0, 20.0, 0.0, -10.0, 20.0
            ]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Topo(Topo::ExplodeChildren("w3")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r8", "hist", "w3_1"))),
        Step::Check(Check::Counts("r1", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r1", 679.305, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r2", 649.975, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 1)])),
        Step::Check(Check::Props(Prop::Area, "r3", 554.732, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r4", 648.481, 0.01, None)),
        Step::Check(Check::Counts("r5", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r5", 828.494, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r6", 837.872, 0.01, None)),
        Step::Check(Check::Counts("r7", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r7", 866.128, 0.01, None)),
        Step::Check(Check::Counts("r8", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r8", 2532.49, 0.01, None)),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Topo(Topo::Explode("w3", "v")),
        Step::More(More::History(HistoryStep::Generated("r9", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r10", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r11", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r12", "hist", "w1_4"))),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w2_3"))),
        Step::More(More::History(HistoryStep::Generated("r16", "hist", "w3_1"))),
        Step::Check(Check::Counts("r9", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r9", 43.4094, 0.01, None)),
        Step::Check(Check::Counts("r10", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r10", 43.6285, 0.01, None)),
        Step::Check(Check::Counts("r11", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r11", 40.596, 0.01, None)),
        Step::Check(Check::Counts("r12", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r12", 40.9228, 0.01, None)),
        Step::Check(Check::Counts("r13", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r13", 40.0003, 0.01, None)),
        Step::Check(Check::Counts("r14", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r14", 40.3103, 0.01, None)),
        Step::Check(Check::Counts("r15", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r15", 40.3794, 0.01, None)),
        Step::Check(Check::Counts("r16", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r16", 40.9892, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_4,
    "tests/bugs/modalg_6/bug26150_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Vertex("v1", [0.0, -10.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [10.0, 8.0, 20.0])),
        Step::Profile(Profile::Vertex("v3", [-10.0, 7.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Topo(Topo::Orient("e1", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Topo(Topo::Orient("e2", 'R')),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Topo(Topo::Orient("e3", 'R')),
        Step::Profile(Profile::Wire("w2", &["e1", "e2", "e3"])),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Topo(Topo::ExplodeChildren("w2")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r7", "hist", "w2_3"))),
        Step::Check(Check::Counts("r5", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r5", 866.128, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r6", 837.872, 0.01, None)),
        Step::Check(Check::Counts("r7", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r7", 828.494, 0.01, None)),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::More(More::History(HistoryStep::Generated("r13", "hist", "w2_1"))),
        Step::More(More::History(HistoryStep::Generated("r14", "hist", "w2_2"))),
        Step::More(More::History(HistoryStep::Generated("r15", "hist", "w2_3"))),
        Step::Check(Check::Counts("r13", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r13", 40.0003, 0.01, None)),
        Step::Check(Check::Counts("r14", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r14", 40.3794, 0.01, None)),
        Step::Check(Check::Counts("r15", &[("EDGE", 1)])),
        Step::Check(Check::Props(Prop::Length, "r15", 40.3103, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_5,
    "tests/bugs/modalg_6/bug26150_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, -10.0, 20.0, -10.0, 7.0, 20.0, 10.0, 8.0, 20.0, 0.0, -10.0, 20.0
            ]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -30.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 70.0])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "w2", "w3", "v2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w1_4"))),
        Step::Check(Check::Counts("r1", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r1", 1232.78, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 12)])),
        Step::Check(Check::Props(Prop::Area, "r2", 1215.55, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r3", 1155.34, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 8)])),
        Step::Check(Check::Props(Prop::Area, "r4", 1214.91, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "v1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "v2"))),
        Step::Check(Check::Counts("r5", &[("EDGE", 32)])),
        Step::Check(Check::Props(Prop::Length, "r5", 841.334, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("EDGE", 32)])),
        Step::Check(Check::Props(Prop::Length, "r6", 841.334, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_6,
    "tests/bugs/modalg_6/bug26150_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, -10.0, 20.0, -10.0, 7.0, 20.0, 10.0, 8.0, 20.0, 0.0, -10.0, 20.0
            ]
        )),
        Step::Curve(Curve::Circle("w3", &[0.0, 0.0, 40.0, 10.0])),
        Step::Profile(Profile::Edge("w3", "w3", None)),
        Step::Profile(Profile::Wire("w3", &["w3"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -30.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 70.0])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["v1", "w1", "w2", "w3", "v2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w1_4"))),
        Step::Check(Check::Counts("r1", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r1", 1970.62, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 3)])),
        Step::Check(Check::Props(Prop::Area, "r2", 1933.42, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 1)])),
        Step::Check(Check::Props(Prop::Area, "r3", 1690.2, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 2)])),
        Step::Check(Check::Props(Prop::Area, "r4", 1859.72, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "v1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "v2"))),
        Step::Check(Check::Counts("r5", &[("EDGE", 8)])),
        Step::Check(Check::Props(Prop::Length, "r5", 902.911, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("EDGE", 8)])),
        Step::Check(Check::Props(Prop::Length, "r6", 902.911, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_7,
    "tests/bugs/modalg_6/bug26150_7",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                -10.0, -10.0, 20.0, 10.0, -10.0, 20.0, 10.0, 10.0, 20.0, -10.0, 10.0, 20.0, -10.0,
                -10.0, 20.0
            ]
        )),
        Step::Place(Place::Rotate(&["w2"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0])),
        Step::Profile(Profile::Polyline(
            "w3",
            &[
                -10.0, -10.0, 40.0, 10.0, -10.0, 40.0, 10.0, 10.0, 40.0, -10.0, 10.0, 40.0, -10.0,
                -10.0, 40.0
            ]
        )),
        Step::Place(Place::Rotate(&["w3"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -30.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 70.0])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: true,
            check: false,
            mutable_input: true,
            sections: &["v1", "w1", "w2", "w3", "v2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w1_4"))),
        Step::Check(Check::Counts("r1", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r1", 1412.16, 0.01, None)),
        Step::Check(Check::Counts("r2", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r2", 1412.16, 0.01, None)),
        Step::Check(Check::Counts("r3", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r3", 1412.16, 0.01, None)),
        Step::Check(Check::Counts("r4", &[("FACE", 4)])),
        Step::Check(Check::Props(Prop::Area, "r4", 1412.16, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "v1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "v2"))),
        Step::Check(Check::Counts("r5", &[("EDGE", 16)])),
        Step::Check(Check::Props(Prop::Length, "r5", 447.259, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("EDGE", 16)])),
        Step::Check(Check::Props(Prop::Length, "r6", 447.259, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26150_8,
    "tests/bugs/modalg_6/bug26150_8",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26150"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -10.0, -10.0, 0.0, 10.0, -10.0, 0.0, 10.0, 10.0, 0.0, -10.0, 10.0, 0.0, -10.0,
                -10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                -10.0, -10.0, 20.0, 10.0, -10.0, 20.0, 10.0, 10.0, 20.0, -10.0, 10.0, 20.0, -10.0,
                -10.0, 20.0
            ]
        )),
        Step::Place(Place::Rotate(&["w2"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0])),
        Step::Profile(Profile::Polyline(
            "w3",
            &[
                -10.0, -10.0, 40.0, 10.0, -10.0, 40.0, 10.0, 10.0, 40.0, -10.0, 10.0, 40.0, -10.0,
                -10.0, 40.0
            ]
        )),
        Step::Place(Place::Rotate(&["w3"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -30.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 70.0])),
        Step::Topo(Topo::ExplodeChildren("w1")),
        Step::Loft(Loft {
            result: "r",
            solid: true,
            ruled: false,
            check: false,
            mutable_input: true,
            sections: &["v1", "w1", "w2", "w3", "v2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::More(More::History(HistoryStep::Generated("r1", "hist", "w1_1"))),
        Step::More(More::History(HistoryStep::Generated("r2", "hist", "w1_2"))),
        Step::More(More::History(HistoryStep::Generated("r3", "hist", "w1_3"))),
        Step::More(More::History(HistoryStep::Generated("r4", "hist", "w1_4"))),
        Step::Check(Check::Props(Prop::Area, "r1", 1694.42, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2", 1694.42, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r3", 1694.42, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r4", 1694.42, 0.01, None)),
        Step::More(More::History(HistoryStep::Generated("r5", "hist", "v1"))),
        Step::More(More::History(HistoryStep::Generated("r6", "hist", "v2"))),
        Step::Check(Check::Counts("r5", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r5", 466.764, 0.01, None)),
        Step::Check(Check::Counts("r6", &[("EDGE", 4)])),
        Step::Check(Check::Props(Prop::Length, "r6", 466.764, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_6_bug26330,
    "tests/bugs/modalg_6/bug26330",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("=========="),
        Step::Puts("26330"),
        Step::Puts("=========="),
        Step::Topo(Topo::Restore("brep/bug26330_s1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/bug26330_s2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["s1", "s2"]
        }),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26334,
    "tests/bugs/modalg_6/bug26334",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26334"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug26334_pw1.brep", "pw1")),
        Step::Topo(Topo::Restore("brep/bug26334_pw2.brep", "pw2")),
        Step::Topo(Topo::Restore("brep/bug26334_pw3.brep", "pw3")),
        Step::Shell(PipeShell::Start("pw1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pw2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pw3",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::TypedCountsNamed(
            "result",
            &[
                ("VERTEX", 10),
                ("EDGE", 13),
                ("WIRE", 4),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 32)
            ],
            "Result obtained by BRepOffsetAPI_MakePipeShell"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26426,
    "tests/bugs/modalg_6/bug26426",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("26426"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26426-prism_shape.brep", "b")),
        Step::Let("bug_info_0", Value::ToleranceMax("b")),
        Step::Topo(Topo::Explode("b", "f")),
        Step::Draft(Draft {
            result: "r",
            shape: "b",
            direction: [0.0, 0.0, -1.0],
            faces: &[("b_1", -10.0, [0.0, 0.0, -40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Let("bug_info_1", Value::ToleranceMax("b")),
        Step::Let("bug_info_2", Value::ToleranceMax("r")),
        Step::If(
            Cond::Below("bug_info_0", "bug_info_1", 0.0),
            &[Step::Puts(
                "ERROR: 26426 is reproduced. Tolerance has been increased (case 1)."
            )],
            &[]
        ),
        Step::If(
            Cond::Below("bug_info_0", "bug_info_2", 1e-07),
            &[Step::Puts(
                "ERROR: 26426 is reproduced. Tolerance has been increased (case 2)."
            )],
            &[]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug26427,
    "tests/bugs/modalg_6/bug26427",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26427"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC26426-prism_shape.brep", "b1")),
        Step::Topo(Topo::Explode("b1", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "b1",
            direction: [0.0, 0.0, -1.0],
            faces: &[("b1_1", -10.0, [0.0, 0.0, -40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::TypedCountsNamed(
            "result",
            &[
                ("VERTEX", 16),
                ("EDGE", 24),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 62)
            ],
            "Shape produced by Draft angle algorithm"
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26470_1,
    "tests/bugs/modalg_6/bug26470_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("26470"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26470-ClosedWire.brep", "a")),
        Step::Topo(Topo::Restore("brep/OCC26470-wprof1.brep", "b")),
        Step::More(More::Evolved(Evolved {
            result: "res",
            spine: "a",
            profile: "b",
            solid: false,
            compute_axes: true,
            volume: false,
            tolerance: 0.0,
            parallel: true
        })),
    ]
);

offset_case!(
    bugs_modalg_6_bug26470_2,
    "tests/bugs/modalg_6/bug26470_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("26470"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26470-ClosedWire.brep", "a")),
        Step::Topo(Topo::Restore("brep/OCC26470-wprof2.brep", "b")),
        Step::More(More::Evolved(Evolved {
            result: "res",
            spine: "a",
            profile: "b",
            solid: false,
            compute_axes: true,
            volume: false,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Check(Check::Shape("res")),
    ]
);

offset_case!(
    bugs_modalg_6_bug26473_2,
    "tests/bugs/modalg_6/bug26473_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("=========="),
        Step::Puts("26473"),
        Step::Puts("=========="),
        Step::Prim(Prim::Box("b1", &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0])),
        Step::Topo(Topo::Explode("b1", "F")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("b1", 0.0, &[])),
        Step::Offset(Offset::OnFace(&[("b1_1", 1.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 10.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26498,
    "tests/bugs/modalg_6/bug26498",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26498"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w",
            &[
                0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.2, 0.2, 0.0, 0.4, 0.2, 0.0, 0.4, 0.4, 0.0, -0.06,
                0.57, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "r",
                base: "w",
                count: 1,
                step: -0.2,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: true
            }))),
            &[Step::Puts("Error : openoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("r_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 0.272485, 0.01, None)),
                Step::Check(Check::Section("result")),
                Step::Check(Check::TypedCountsNamed(
                    "result",
                    &[
                        ("VERTEX", 3),
                        ("EDGE", 2),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 6)
                    ],
                    "Offset"
                )),
                Step::Check(Check::Shape("result")),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_6_bug26565_3,
    "tests/bugs/modalg_6/bug26565_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26565"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b1", "f")),
        Step::Topo(Topo::SharedCopy("b1_1", "f")),
        Step::Topo(Topo::Explode("b1", "e")),
        Step::Topo(Topo::SharedCopy("b1_8", "e")),
        Step::Prim(Prim::Box("b2", &[10.0, 5.0, 5.0])),
        Step::More(More::EmptyShape("sh", "sh")),
        Step::Topo(Topo::Add("f", "sh")),
        Step::Topo(Topo::SharedCopy("sh", "f")),
        Step::More(More::EmptyShape("w", "w")),
        Step::Topo(Topo::Add("e", "w")),
        Step::Topo(Topo::SharedCopy("w", "e")),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["f", "e"]))),
        Step::More(More::General(General::AddTools(&["b2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Operation("r", 0))),
        Step::Check(Check::NbShapes("r")),
        Step::More(More::General(General::Operation("r", 2))),
        Step::Check(Check::NbShapes("r")),
        Step::Topo(Topo::ExplodeChildren("r")),
        Step::If(
            Cond::KindHolds("r_1", "SHELL"),
            &[Step::Puts("OK : There is SHELL; Compsolid is good")],
            &[Step::Puts("Error : There is not SHELL; Compsolid is bad")]
        ),
        Step::If(
            Cond::KindHolds("r_2", "WIRE"),
            &[Step::Puts("OK : There is WIRE; Compsolid is good")],
            &[Step::Puts("Error : There is not WIRE; Compsolid is bad")]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug26565_4,
    "tests/bugs/modalg_6/bug26565_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26565"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[10.0, 0.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b3", &[10.0, 5.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b4", &[10.0, 0.0, 5.0, 10.0, 10.0, 5.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&[
            "b1", "b2", "b3", "b4"
        ]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("r"))),
        Step::Topo(Topo::Explode("r", "so")),
        Step::More(More::EmptyShape("b1", "CS")),
        Step::Topo(Topo::Add("r_1", "b1")),
        Step::Topo(Topo::Add("r_2", "b1")),
        Step::Topo(Topo::Add("r_3", "b1")),
        Step::Topo(Topo::Add("r_4", "b1")),
        Step::Prim(Prim::Box("b2", &[5.0, 0.0, 2.0, 10.0, 10.0, 10.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["b1"]))),
        Step::More(More::General(General::AddTools(&["b2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Operation("r", 2))),
        Step::Topo(Topo::ExplodeChildren("r")),
        Step::If(
            Cond::KindHolds("r_1", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug26565_5,
    "tests/bugs/modalg_6/bug26565_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26565"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[10.0, 0.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b3", &[10.0, 5.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b4", &[10.0, 0.0, 5.0, 10.0, 10.0, 5.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&[
            "b1", "b2", "b3", "b4"
        ]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("r"))),
        Step::Topo(Topo::Explode("r", "so")),
        Step::More(More::EmptyShape("b1", "CS")),
        Step::Topo(Topo::Add("r_1", "b1")),
        Step::Topo(Topo::Add("r_2", "b1")),
        Step::Topo(Topo::Add("r_3", "b1")),
        Step::Topo(Topo::Add("r_4", "b1")),
        Step::Prim(Prim::Box("b2", &[5.0, 0.0, 2.0, 10.0, 10.0, 5.0])),
        Step::Prim(Prim::Box("b3", &[5.0, 0.0, 7.0, 10.0, 10.0, 5.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["b2", "b3"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("r"))),
        Step::Topo(Topo::Explode("r", "so")),
        Step::More(More::EmptyShape("b2", "CS")),
        Step::Topo(Topo::Add("r_1", "b2")),
        Step::Topo(Topo::Add("r_2", "b2")),
        Step::Prim(Prim::Box("b3", &[-5.0, 2.0, 3.0, 30.0, 6.0, 4.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["b1", "b2"]))),
        Step::More(More::General(General::AddTools(&["b3"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Operation("r", 2))),
        Step::Topo(Topo::ExplodeChildren("r")),
        Step::If(
            Cond::KindHolds("r_1", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
        Step::If(
            Cond::KindHolds("r_2", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug26565_6,
    "tests/bugs/modalg_6/bug26565_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26565"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[10.0, 0.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b3", &[10.0, 5.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b4", &[10.0, 0.0, 5.0, 10.0, 10.0, 5.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&[
            "b1", "b2", "b3", "b4"
        ]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("r"))),
        Step::Topo(Topo::Explode("r", "so")),
        Step::More(More::EmptyShape("b1", "CS")),
        Step::Topo(Topo::Add("r_1", "b1")),
        Step::Topo(Topo::Add("r_2", "b1")),
        Step::Topo(Topo::Add("r_3", "b1")),
        Step::Topo(Topo::Add("r_4", "b1")),
        Step::Prim(Prim::Box("b2", &[5.0, 0.0, 2.0, 10.0, 10.0, 5.0])),
        Step::Prim(Prim::Box("b3", &[5.0, 0.0, 7.0, 10.0, 10.0, 5.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["b2", "b3"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("r"))),
        Step::Topo(Topo::Explode("r", "so")),
        Step::More(More::EmptyShape("b2", "CS")),
        Step::Topo(Topo::Add("r_1", "b2")),
        Step::Topo(Topo::Add("r_2", "b2")),
        Step::Prim(Prim::Box("b3", &[5.0, -2.0, -2.0, 10.0, 14.0, 20.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["b1", "b2"]))),
        Step::More(More::General(General::AddTools(&["b3"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Operation("r", 2))),
        Step::Topo(Topo::ExplodeChildren("r")),
        Step::If(
            Cond::KindHolds("r_1", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
        Step::If(
            Cond::KindHolds("r_2", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug26565_7,
    "tests/bugs/modalg_6/bug26565_7",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26565"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[10.0, 0.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b3", &[10.0, 5.0, 0.0, 10.0, 5.0, 5.0])),
        Step::Prim(Prim::Box("b4", &[10.0, 0.0, 5.0, 10.0, 10.0, 5.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&[
            "b1", "b2", "b3", "b4"
        ]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("r"))),
        Step::Topo(Topo::Explode("r", "so")),
        Step::More(More::EmptyShape("b1", "CS")),
        Step::Topo(Topo::Add("r_1", "b1")),
        Step::Topo(Topo::Add("r_2", "b1")),
        Step::Topo(Topo::Add("r_3", "b1")),
        Step::Topo(Topo::Add("r_4", "b1")),
        Step::Prim(Prim::Box("b2", &[5.0, 0.0, 2.0, 10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b3", &[-5.0, 2.0, 3.0, 30.0, 6.0, 4.0])),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::ClearTools)),
        Step::More(More::General(General::AddObjects(&["b1", "b2"]))),
        Step::More(More::General(General::AddTools(&["b3"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Operation("r", 2))),
        Step::Topo(Topo::ExplodeChildren("r")),
        Step::If(
            Cond::KindHolds("r_1", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
        Step::If(
            Cond::KindHolds("r_2", "SOLID"),
            &[Step::Puts("OK : There is SOLID; Compsolid is good")],
            &[Step::Puts("Error : There is not SOLID; Compsolid is bad")]
        ),
    ]
);

offset_case!(
    bugs_modalg_6_bug26565_8,
    "tests/bugs/modalg_6/bug26565_8",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("26565"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b", "f")),
        Step::More(More::EmptyShape("profile", "Sh")),
        Step::Topo(Topo::Add("b_1", "profile")),
        Step::Topo(Topo::Add("b_3", "profile")),
        Step::Profile(Profile::Polyline(
            "spine",
            &[0.0, 0.0, 0.0, 10.0, 10.0, 0.0]
        )),
        Step::Sweep(Sweep::Pipe("sh", "spine", "profile", 0, false)),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[10.0, 10.0, 0.0, -1.0, -1.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::More(More::Face("f", "pl", None)),
        Step::More(More::EmptyShape("sl", "So")),
        Step::More(More::EmptyShape("sl_sh", "Sh")),
        Step::Topo(Topo::Add("f", "sl_sh")),
        Step::Topo(Topo::Add("sl_sh", "sl")),
        Step::Bool(Bool::Cut("res", "sh", "sl")),
        Step::Topo(Topo::ExplodeChildren("res")),
        Step::If(
            Cond::KindHolds("res_1", "COMPSOLID"),
            &[Step::Puts("OK : There is COMPSOLID; Compsolid is good")],
            &[Step::Puts(
                "Error : There is not COMPSOLID; Compsolid is bad"
            )]
        ),
    ]
);
